use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use clap::{clap_app, AppSettings, ArgMatches};
use ex::fs;
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use reconfix::{
    external_data::Synchronizer, orchestrator::Node, ExternalData, Lens,
    Orchestrator,
};
use serde_json::Value;
use std::{
    iter,
    path::PathBuf,
    process,
    str::FromStr,
    sync::{mpsc, Arc},
    thread,
    time::Duration,
};
use tokio::{runtime::Runtime, sync::Mutex};
use version::version;

const DEFAULT_DEBOUNCE_DELAY: Duration = Duration::from_millis(100);

struct JsonFileExternalData {
    path: PathBuf,
    debounce_delay: Duration,
    watcher: Mutex<Option<RecommendedWatcher>>,
    value: Arc<Mutex<Arc<Value>>>,
}

impl JsonFileExternalData {
    fn new<P>(path: P, debounce_delay: Option<Duration>) -> Self
    where
        P: Into<PathBuf>,
    {
        Self {
            path: path.into(),
            debounce_delay: debounce_delay.unwrap_or(DEFAULT_DEBOUNCE_DELAY),
            watcher: Mutex::new(None),
            value: Arc::new(Mutex::new(Arc::new(Value::Null))),
        }
    }
}

#[async_trait]
impl ExternalData for JsonFileExternalData {
    async fn listen(&self, synchronizer: Synchronizer) -> Result<Arc<Value>> {
        // Watch the JSON file so we can send a new value when it changes
        let (watcher_sink, watcher_source) = mpsc::channel();
        let mut watcher = notify::watcher(watcher_sink, self.debounce_delay)?;
        watcher.watch(&self.path, RecursiveMode::NonRecursive)?;

        // Translate `notify`'s blocking channel in a futures-aware channel
        let (async_watcher_sink, mut async_watcher_source) =
            tokio::sync::mpsc::channel(1);
        thread::spawn(move || {
            while let Ok(event) = watcher_source.recv() {
                if async_watcher_sink.blocking_send(event).is_err() {
                    break;
                }
            }
        });

        // Parse the JSON file and send the first value
        let initial_value = Arc::new(serde_json::from_str::<Value>(
            &tokio::fs::read_to_string(&self.path).await?,
        )?);
        *self.value.lock().await = initial_value.clone();

        // If there's no error, keep the watcher
        *self.watcher.lock().await = Some(watcher);

        // Spawn the pusher task
        let value = self.value.clone();
        tokio::spawn(async move {
            while let Some(event) = async_watcher_source.recv().await {
                if let DebouncedEvent::Write(path) = event {
                    let mut value = value.lock().await;
                    let res: Result<()> = async {
                        let new_value = Arc::new(
                            serde_json::from_str::<Value>(
                                &tokio::fs::read_to_string(&path)
                                    .await
                                    .unwrap(),
                            )
                            .unwrap(),
                        );
                        synchronizer.apply(new_value.clone()).await?;
                        *value = new_value;

                        Ok(())
                    }
                    .await;
                    if let Err(err) = res {
                        eprintln!(
                            "cannot synchronize {}, reverting due to: {:#?}",
                            path.display(),
                            err
                        );
                        tokio::fs::write(
                            path,
                            &serde_json::to_vec(&**value).unwrap(),
                        )
                        .await
                        .unwrap();
                    }
                }
            }
        });

        Ok(initial_value)
    }

    async fn commit(&self, new_value: &Arc<Value>) -> Result<()> {
        let mut value = self.value.lock().await;
        tokio::fs::write(&self.path, &serde_json::to_vec(&**new_value)?)
            .await?;
        *value = new_value.clone();

        Ok(())
    }
}

fn main() {
    let args = clap_app!(reconfix =>
        (version: version!())
        (about: "(Re)Configuration toolkit command-line interface")
        (@subcommand check =>
            (about: "Check whether a lens is valid")
            (@arg LENS: +required "Lens to be checked")
        )
        (@subcommand run =>
            (about: "Run a transformation pipeline between two external data nodes")
            (@arg PIPELINE: +required ... "Pipeline definition")
        )
    )
    .setting(AppSettings::ArgRequiredElseHelp)
    .get_matches();

    let res = if let Some(args) = args.subcommand_matches("check") {
        run_check(args)
    } else if let Some(args) = args.subcommand_matches("run") {
        run_run(args)
    } else {
        unreachable!()
    };
    if let Err(err) = res {
        eprintln!("fatal: {:#?}", err);
        process::exit(1);
    }
}

fn run_check(args: &ArgMatches) -> Result<()> {
    Lens::new(&fs::read_to_string(args.value_of("LENS").unwrap())?)?;

    Ok(())
}

fn run_run(args: &ArgMatches) -> Result<()> {
    let runtime = Runtime::new().context("while creating a tokio runtime")?;
    let _tokio_context_guard = runtime.enter();

    let mut orchestrator = Orchestrator::new();
    let mut last_node = None;
    let mut node_definition = Vec::new();
    for item in args.values_of("PIPELINE").unwrap().chain(iter::once("!")) {
        if item == "!" {
            let node = orchestrator.add_node(build_node(&node_definition)?);
            if let Some(last_node) = last_node {
                orchestrator.add_edge(last_node, node);
            }

            last_node = Some(node);
            node_definition.clear();
        } else {
            node_definition.push(item);
        }
    }

    runtime
        .block_on(orchestrator.run())
        .context("while running the orchestrator")?;

    Ok(())
}

fn build_node(definition: &[&str]) -> Result<Node<'static>> {
    if definition.is_empty() {
        return Err(anyhow!("empty node definition"));
    }

    let args = definition[1..].iter().map(|arg| {
        let (key, mut value) =
            arg.find('=').map(|x| arg.split_at(x)).unwrap_or((arg, ""));
        if !value.is_empty() {
            value = &value[1..];
        }

        (key, value)
    });

    match definition[0] {
        "json" => new_json_node(args),
        "lens" => new_lens_node(args),
        name => Err(anyhow!("unknown node constructor: {}", name)),
    }
}

fn new_json_node<'a>(
    args: impl Iterator<Item = (&'a str, &'a str)>,
) -> Result<Node<'static>> {
    #[derive(Default)]
    struct Configuration<'a> {
        path: Option<&'a str>,
        debounce_delay: Option<Duration>,
    }

    let mut config = Configuration::default();
    for (key, value) in args {
        match key {
            "path" => config.path = Some(value),
            "debounce-delay" => config.debounce_delay = Some(Duration::from_millis(FromStr::from_str(value).context("while trying to parse value for 'debounce-delay' for the 'json' constructor")?)),
            _ => {
                return Err(anyhow!(
                    "unknown argument for the 'json' constructor: {}",
                    key
                ))
            }
        }
    }

    Ok(JsonFileExternalData::new(
        config.path.ok_or_else(|| {
            anyhow!("argument 'path' is required for the 'json' constructor")
        })?,
        config.debounce_delay,
    )
    .into())
}

fn new_lens_node<'a>(
    args: impl Iterator<Item = (&'a str, &'a str)>,
) -> Result<Node<'static>> {
    #[derive(Default)]
    struct Configuration<'a> {
        path: Option<&'a str>,
    }

    let mut config = Configuration::default();
    for (key, value) in args {
        match key {
            "path" => config.path = Some(value),
            _ => {
                return Err(anyhow!(
                    "unknown argument for the 'lens' constructor: {}",
                    key
                ))
            }
        }
    }

    Ok(Lens::new(&fs::read_to_string(config.path.ok_or_else(|| {
        anyhow!("argument 'path' is required for the 'lens' constructor")
    })?)?)?
    .into())
}
