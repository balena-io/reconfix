use crate::orchestrator::NodeHandle;
use anyhow::anyhow;
use async_trait::async_trait;
use js_sys::{Function, Promise};
use serde_json::Value;
use std::{rc::Rc, result, sync::Arc};
use tokio::sync::oneshot;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::JsFuture;

type Result<T> = result::Result<T, JsValue>;

#[wasm_bindgen]
pub struct Lens(crate::Lens);

#[wasm_bindgen]
impl Lens {
    #[wasm_bindgen(constructor)]
    pub fn new(source: &str) -> Result<Lens> {
        crate::Lens::new(source)
            .map(Self)
            .map_err(|x| x.to_string().into())
    }

    pub fn invert(&mut self) {
        self.0.invert();
    }
}

#[wasm_bindgen]
pub struct Orchestrator(crate::Orchestrator<'static>);

#[wasm_bindgen]
impl Orchestrator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Orchestrator {
        Self(crate::Orchestrator::new())
    }

    pub fn add_lens(&mut self, node: Lens) -> NodeHandle {
        self.0.add_node(node.0)
    }

    pub fn add_external_data(
        &mut self,
        listen: Function,
        commit: Function,
    ) -> NodeHandle {
        self.0.add_node(JsExternalData::new(listen, commit))
    }

    pub fn add_edge(&mut self, a: NodeHandle, b: NodeHandle) {
        self.0.add_edge(a, b);
    }

    pub async fn run(self) -> Result<()> {
        self.0.run().await.map_err(|x| x.to_string().into())
    }
}

#[wasm_bindgen]
pub struct Synchronizer(Rc<crate::external_data::Synchronizer>);

#[wasm_bindgen]
impl Synchronizer {
    pub fn apply(&self, new_value: JsValue) -> Promise {
        let synchronizer = self.0.clone();

        wasm_bindgen_futures::future_to_promise(async move {
            synchronizer
                .apply(JsValue::into_serde::<Value>(&new_value).map_err(
                    |_| ApplyError {
                        message: format!("invalid value: {:?}", new_value),
                        code: ApplyErrorCode::InvalidValue,
                    },
                )?)
                .await
                .map(|_| JsValue::NULL)
                .map_err(|x| {
                    let code = match x {
                        crate::external_data::ApplyError::NewValueConflicts(
                            _,
                        ) => ApplyErrorCode::NewValueConflicts,
                        crate::external_data::ApplyError::ShuttingDown => {
                            ApplyErrorCode::ShuttingDown
                        }
                    };

                    ApplyError {
                        message: x.to_string(),
                        code,
                    }
                    .into()
                })
        })
    }
}

#[wasm_bindgen]
struct ApplyError {
    message: String,
    code: ApplyErrorCode,
}

#[wasm_bindgen]
impl ApplyError {
    pub fn message(&self) -> String {
        self.message.clone()
    }

    pub fn code(&self) -> ApplyErrorCode {
        self.code
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum ApplyErrorCode {
    InvalidValue,
    NewValueConflicts,
    ShuttingDown,
}

struct JsExternalData {
    listen: Function,
    commit: Function,
}

// This is ok since we can only run in a single thread anyway, and creating
// two versions of the orchestrator (Send and !Send) is too much of a
// bother. The root of the issue is that `JsValue` is !Send and there's no
// other way to send JS closures between threads.
//
// Of course, this will have to be revisited when WASM gets threads
unsafe impl Send for JsExternalData {}
unsafe impl Sync for JsExternalData {}

impl JsExternalData {
    fn new(listen: Function, commit: Function) -> Self {
        Self { listen, commit }
    }
}

#[async_trait]
impl crate::ExternalData for JsExternalData {
    async fn listen(
        &self,
        synchronizer: crate::external_data::Synchronizer,
    ) -> anyhow::Result<Arc<Value>> {
        // Due to Send/!Send shenanigans, we cannot await on a JS
        // `commit()` here. Instead we have to use
        // `wasm_bindgen_futures::spawn_local`. We cannot send `&self` into
        // that function since it requires `'static`, so we are forced to
        // clone the JS function. However because we await at the end of
        // this async method, its local variables must be `Send` too.
        // `SendableFunction` is used to make the compiler happy without
        // having to transmute the lifetime of `&self` which is arguably
        // worse in terms of safety
        let listen = SendableFunction(self.listen.clone());

        let (res_sink, res_source) = oneshot::channel();
        wasm_bindgen_futures::spawn_local(async move {
            let res = async move {
                let listen_value = listen
                    .0
                    .call1(
                        &JsValue::NULL,
                        &JsValue::from(Synchronizer(Rc::new(synchronizer))),
                    )
                    .map_err(|x| {
                        anyhow!("failed to call JS `listen()`: {:?}", x)
                    })?;

                JsFuture::from(Promise::from(listen_value))
                    .await
                    .map_err(|x| anyhow!("{:?}", x))
                    .and_then(|x| {
                        Ok(Arc::new(
                            JsValue::into_serde(&x)
                                .map_err(|err| {
                                    anyhow!(
                                        "failed to deserialize the initial value returned by `listen()`: {}",
                                        err
                                    )
                                })?
                        ))
                    })
            }
            .await;
            let _ = res_sink.send(res);
        });

        res_source.await.unwrap()
    }

    async fn commit(&self, new_value: &Arc<Value>) -> anyhow::Result<()> {
        // See `listen()` above
        let commit = SendableFunction(self.commit.clone());

        let new_value = (**new_value).clone();
        let (res_sink, res_source) = oneshot::channel();
        wasm_bindgen_futures::spawn_local(async move {
            let res = async move {
                let new_value = JsValue::from_serde(&new_value)
                    .map_err(|x| {
                        anyhow!("failed to serialize new value: {}", x)
                    })?
                    .into();
                let commit_value =
                    commit.0.call1(&JsValue::NULL, &new_value).map_err(
                        |x| anyhow!("failed to call JS `commit()`: {:?}", x),
                    )?;

                JsFuture::from(Promise::from(commit_value))
                    .await
                    .map(|_| ())
                    .map_err(|x| anyhow!("{:?}", x))
            }
            .await;
            let _ = res_sink.send(res);
        });

        res_source.await.unwrap()
    }
}

struct SendableFunction(Function);

// This is safe because of how this struct is used. See
// `JsExternalData::listen()`
unsafe impl Send for SendableFunction {}
