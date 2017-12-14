
#[macro_use] extern crate log;
extern crate env_logger;
#[macro_use] extern crate neon;
extern crate reconfix;
extern crate serde_json;

mod json;
mod buffer;
mod task;

use neon::vm::{VmResult, JsResult, Lock, Throw};
use neon::js::{Object, JsValue, JsUndefined, JsString, JsFunction};
use neon::js::error as jserror;
use neon::mem::PersistentHandle;
use neon::scope::Scope;
use neon::task::Task;

use std::io;
use std::error;

use reconfix::{Reconfix, Plugin, FileNode};

struct CallbackPlugin {
    reader: task::ReadHandle,
    writer: task::WriteHandle,
}

impl CallbackPlugin {
    fn new<'a, S: Scope<'a>>(scope: &mut S, read: PersistentHandle, write: PersistentHandle) -> VmResult<CallbackPlugin> {
        let read = read.into_handle(scope).check::<JsFunction>()?;
        let write = write.into_handle(scope).check::<JsFunction>()?;

        let reader = task::ReadHandle::new(scope, read)?;
        let writer = task::WriteHandle::new(scope, write)?;

        Ok(CallbackPlugin { 
            reader: reader,
            writer: writer,
        })
    }
}

impl<'a> Plugin for &'a mut CallbackPlugin {
    fn read(self, file: FileNode) -> Result<Vec<u8>, Box<error::Error + Send>> {
        debug!("js plugin read invoked");
        let result = self.reader.read(file).map_err(handle_error);
        debug!("js plugin read finished");
        result
    }

    fn write(self, file: FileNode, data: Vec<u8>) -> Result<(), Box<error::Error + Send>> {
        debug!("js plugin write invoked");
        let result = self.writer.write(file, data).map_err(handle_error);
        debug!("js plugin write finished");
        result
    }
}

fn handle_error(err: Option<task::ErrorWrapper>) -> Box<error::Error + Send> {
    match err {
        Some(e) => Box::new(e),
        None => Box::new(io::Error::new(io::ErrorKind::Other, "channel closed")),
    }
}

fn render_error<'a, S: Scope<'a>, T>(_: &mut S, err: &::std::error::Error) -> VmResult<T> {
    use std::fmt::Write;

    let mut buf = String::new();
    writeln!(&mut buf, "{}", err).expect("unable to serialize error");

    let mut e = err;
    while let Some(next) = e.cause() {
        writeln!(&mut buf, "Caused by: {}", next).expect("unable to serialize error cause");
        e = next;
    }

    jserror::JsError::throw(jserror::Kind::Error, buf.as_str())
}

fn throw_if_native<'a, S: Scope<'a>>(
    scope: &mut S, 
    err: Box<error::Error + Send>) -> Result<Box<error::Error + Send>, Throw> 
{
    let other = match err.downcast::<task::ErrorWrapper>() {
        Ok(wrapper) => {
            debug!("detected wrapped error");
            let value = wrapper.into_handle(scope);
            return neon::js::error::throw(value);
        },
        Err(g) => g,
    };

    match other.downcast::<Throw>() {
        Ok(js) => {
            debug!("detected thrown error");
            Err(*js)
        },
        Err(g) => {
            debug!("error is of unknown type");
            Ok(g)
        },
    }
}

fn to_managed_error<'a, S: Scope<'a>, T>(
    scope: &mut S, 
    err: reconfix::Error) -> VmResult<T>
{
    debug!("converting to managed error");
    match err.0 {
        reconfix::ErrorKind::Plugin(e) => {
            let generic = throw_if_native(scope, e)?;
            render_error(scope, &*generic)
        },
        reconfix::ErrorKind::Io(e) => {
            match e.kind() {
                io::ErrorKind::Other => match e.into_inner() {
                    Some(inner) => {
                        let generic = throw_if_native(scope, inner)?;
                        render_error(scope, &*generic)
                    },
                    None => {
                        jserror::JsError::throw(jserror::Kind::Error, "uknown IO error occured")
                    },
                },
                _ => render_error(scope, &e)
            }
        },
        _ => render_error(scope, &err)
    }
}

struct ReadTask {
    reconfix: Reconfix,
    plugin: CallbackPlugin,
}

impl Task for ReadTask {
    type Output = serde_json::Value;
    type Error = reconfix::Error;
    type JsEvent = JsValue;

    fn perform(&mut self) -> Result<Self::Output, Self::Error> {
        debug!("beginning background read task");
        let result = self.reconfix.read_values_plugin(&mut self.plugin);
        debug!("backround read task complete");
        result
    }

    fn complete<'a, T: Scope<'a>>(self, scope: &'a mut T, result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
        debug!("read task completed");
        let json = result.or_else(|e| to_managed_error(scope, e))?;
        let converted = json::from_native(scope, json)?;
        Ok(converted)
    }
}

struct WriteTask {
    reconfix: Reconfix,
    plugin: CallbackPlugin,
    data: serde_json::Value,
}

impl Task for WriteTask {
    type Output = ();
    type Error = reconfix::Error;
    type JsEvent = JsValue;

    fn perform(&mut self) -> Result<Self::Output, Self::Error> {
        debug!("beginning background write task");
        let result = self.reconfix.write_values_plugin(self.data.clone(), &mut self.plugin);
        debug!("background write task complete");
        result
    }

    fn complete<'a, T: Scope<'a>>(self, scope: &'a mut T, result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
        debug!("write task completed");
        result.or_else(|e| to_managed_error(scope, e))?;

        Ok(JsUndefined::new().upcast())
    }
}

#[derive(Clone)]
pub struct ReconfixWrapper {
    reconfix: Reconfix,
    read: PersistentHandle,
    write: PersistentHandle,
}

impl ReconfixWrapper {
    fn into_pair<'a, S: Scope<'a>>(self, scope: &mut S) -> VmResult<(Reconfix, CallbackPlugin)> {
        let plugin = CallbackPlugin::new(scope, self.read, self.write)?;
        Ok((self.reconfix, plugin))
    }
}

declare_types! {
    pub class JsReconfix for ReconfixWrapper {
        init(call) {
            use std::error::Error;

            if let Err(e) = env_logger::init() {
                return jserror::JsError::throw(jserror::Kind::Error, e.description());
            }

            let scope = call.scope;
            let read = call.arguments.require(scope, 0)?.check::<JsFunction>()?;
            let write = call.arguments.require(scope, 1)?.check::<JsFunction>()?;

            let reconfix = Reconfix::new();

            Ok(ReconfixWrapper {
                reconfix: reconfix,
                read: PersistentHandle::new(read),
                write: PersistentHandle::new(write),
            })
        }

        method loadSchema(call) {
            debug!("loading schema...");

            let scope = call.scope;
            let schema = call.arguments.require(scope, 0)?.check::<JsString>()?;
            let data = schema.value();

            call.arguments.this(scope).grab(move |wrapper| {
                wrapper.reconfix.load_schema(data.as_bytes())
            })
            .or_else(|e| to_managed_error(scope, e))?;
            
            Ok(JsUndefined::new().upcast())
        }

        method readValues(call) {
            let scope = call.scope;
            let callback = call.arguments.require(scope, 0)?.check::<JsFunction>()?;
            let wrapper = call.arguments.this(scope).grab(|wrapper| {
                wrapper.clone()
            });

            let (reconfix, plugin) = wrapper.into_pair(scope)?;

            (ReadTask { reconfix: reconfix, plugin: plugin }).schedule(callback);

            Ok(JsUndefined::new().upcast())
        }

        method writeValues(call) {
            let scope = call.scope;
            let json = call.arguments.require(scope, 0)?;
            let dry = json::from_managed(scope, json)?;
            let callback = call.arguments.require(scope, 1)?.check::<JsFunction>()?;
            let wrapper = call.arguments.this(scope).grab(|wrapper| {
                wrapper.clone()
            });

            let (reconfix, plugin) = wrapper.into_pair(scope)?;

            (WriteTask { reconfix: reconfix, plugin: plugin, data: dry }).schedule(callback);

            Ok(JsUndefined::new().upcast())
        }
    }
}

register_module!(m, {
    use neon::js::class::Class;

    let constructor = buffer::JsBufferStream::class(m.scope)?.constructor(m.scope)?;
    m.exports.set("BufferStream", constructor)?;

    let constructor = JsReconfix::class(m.scope)?.constructor(m.scope)?;
    m.exports.set("Reconfix", constructor)?;

    Ok(())
});
