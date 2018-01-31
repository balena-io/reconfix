
use std::fmt;
use std::sync::mpsc;

use neon::scope::Scope;
use neon::js::{JsFunction, JsUndefined, JsNull, JsValue, JsNumber, JsArray, JsString, Object, Value};
use neon::js::binary::JsBuffer;
use neon::js::class::Class;
use neon::js::error::{JsError, Kind};
use neon::mem::{Handle, PersistentHandle};
use neon::vm::{Call, JsResult, VmResult, Lock};
use neon::task::Task;

use reconfix::FileNode;

pub(crate) struct ReadHandle {
    tx: mpsc::SyncSender<FileNode>,
    rx: mpsc::Receiver<Result<Vec<u8>, ErrorWrapper>>,
}

impl ReadHandle {
    pub fn new<'a, S: Scope<'a>>(scope: &mut S, callback: Handle<JsFunction>) -> VmResult<ReadHandle> {
        let (call_tx, call_rx) = mpsc::sync_channel(1);
        let (return_tx, return_rx) = mpsc::sync_channel(1);
        let p = PersistentHandle::new(callback);

        (ReadTask { rx: call_rx, tx: return_tx.clone(), callback: p }).schedule(noop(scope, return_tx.clone())?);

        Ok(ReadHandle { tx: call_tx, rx: return_rx })
    }

    pub fn read(&mut self, file: FileNode) -> Result<Vec<u8>, Option<ErrorWrapper>> {
        debug!("sending read signal");
        self.tx.send(file).map_err(|_| None)?;
        debug!("waiting for read result signal");
        let result = self.rx.recv().map_err(|_| None)?;
        debug!("read result signal received");
        result.map_err(Some)
    }
}

struct ReadTask {
    rx: mpsc::Receiver<FileNode>,
    tx: mpsc::SyncSender<Result<Vec<u8>, ErrorWrapper>>,
    callback: PersistentHandle,
}

impl Task for ReadTask {
    type Output = FileNode;
    type Error = ();
    type JsEvent = JsValue;

    fn perform(&mut self) -> Result<Self::Output, Self::Error> {
        debug!("waiting for read dispatcher signal");
        let value = self.rx.recv().map_err(|_| ())?;
        debug!("read dispatcher signal received");
        Ok(value)
    }

    fn complete<'a, S: Scope<'a>>(self, scope: &'a mut S, result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
        let file = match result {
            Ok(f) => f,
            Err(_) => return Ok(JsUndefined::new().upcast()),
        };

        let callback = self.callback.clone().into_handle(scope).check::<JsFunction>()?;

        let sender = self.tx.clone();
        let continuation = JsFunction::new(scope, Box::new(move |call: Call| {
            debug!("read continuation invoked");

            let scope = call.scope;
            let error = call.arguments.require(scope, 0)?;
            if !error.is_a::<JsUndefined>() && !error.is_a::<JsNull>() {
                debug!("received error value in read continuation");
                return send(&sender, Err(ErrorWrapper::new(error)));
            }

            let mut buffer = call.arguments.require(scope, 1)?.check::<JsBuffer>()?;
            let copied = buffer.grab(|inner| {
                inner.as_slice().to_vec()
            });

            debug!("sending read buffer");
            send(&sender, Ok(copied))
        }))?;

        let partition = JsNumber::new(scope, file.partition.num() as f64).as_value(scope);
        let path = to_array(scope, &file.path)?.upcast();
        let func = continuation.upcast();
        let args = vec![partition, path, func];

        debug!("invoking read dispatcher callback");
        callback.call(scope, JsUndefined::new(), args)?;
        debug!("read dispatcher callback returned");

        let sender = self.tx.clone();
        self.schedule(noop(scope, sender)?);

        Ok(JsUndefined::new().upcast())
    }
}

pub(crate) struct WriteHandle {
    tx: mpsc::SyncSender<(FileNode, Vec<u8>)>,
    rx: mpsc::Receiver<Result<(), ErrorWrapper>>,
}

impl WriteHandle {
    pub fn new<'a, S: Scope<'a>>(scope: &mut S, callback: Handle<JsFunction>) -> VmResult<WriteHandle> {
        let (call_tx, call_rx) = mpsc::sync_channel(1);
        let (return_tx, return_rx) = mpsc::sync_channel(1);
        let p = PersistentHandle::new(callback);

        (WriteTask { rx: call_rx, tx: return_tx.clone(), callback: p }).schedule(noop(scope, return_tx.clone())?);

        Ok(WriteHandle { tx: call_tx, rx: return_rx })
    }

    pub fn write(&mut self, file: FileNode, data: Vec<u8>) -> Result<(), Option<ErrorWrapper>> {
        debug!("sending write signal");
        self.tx.send((file, data)).map_err(|_| None)?;
        debug!("waiting for write result signal");
        let result = self.rx.recv().map_err(|_| None)?;
        debug!("write result signal received");
        result.map_err(Some)
    }
}

pub(crate) struct WriteTask {
    rx: mpsc::Receiver<(FileNode, Vec<u8>)>,
    tx: mpsc::SyncSender<Result<(), ErrorWrapper>>,
    callback: PersistentHandle,
}

impl Task for WriteTask {
    type Output = (FileNode, Vec<u8>);
    type Error = ();
    type JsEvent = JsValue;

    fn perform(&mut self) -> Result<Self::Output, Self::Error> {
        debug!("waiting for write dispatcher signal");
        let value = self.rx.recv().map_err(|_| ())?;
        debug!("write dispatcher signal received");
        Ok(value)
    }

    fn complete<'a, S: Scope<'a>>(self, scope: &'a mut S, result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
        let (file, data) = match result {
            Ok(t) => t,
            Err(_) => return Ok(JsUndefined::new().upcast()),
        };
        let callback = self.callback.clone().into_handle(scope).check::<JsFunction>()?;

        let sender = self.tx.clone();
        let continuation = JsFunction::new(scope, Box::new(move |call| {
            debug!("write continuation invoked");
            let error = call.arguments.require(call.scope, 0)?;
            if !error.is_a::<JsUndefined>() && !error.is_a::<JsNull>() {
                debug!("received error value in write continuation");
                return send(&sender, Err(ErrorWrapper::new(error)));
            }

            debug!("sending write completion signal");
            send(&sender, Ok(()))
        }))?;

        let mut buffer = JsBuffer::new(scope, data.len() as u32)?;
        buffer.grab(move |mut inner| {
            inner.as_mut_slice().clone_from_slice(&data);
        });

        // let args: Vec<Handle<JsValue>> = Vec::new();
        // let mut buffer = JsBufferStream::class(scope)?
        //     .constructor(scope)?
        //     .construct(scope, args)?;

        // buffer.grab(move |inner| {
        //     inner.buf.extend_from_slice(data.as_slice());
        // });

        let partition = JsNumber::new(scope, file.partition.num() as f64).as_value(scope);
        let path = to_array(scope, &file.path)?.upcast();
        let data = buffer.upcast();
        let func = continuation.upcast();
        let args = vec![partition, path, data, func];

        debug!("invoking write dispatcher callback");
        callback.call(scope, JsUndefined::new(), args)?;
        debug!("write dispatcher callback returned");

        let sender = self.tx.clone();
        self.schedule(noop(scope, sender)?);

        Ok(JsUndefined::new().upcast())
    }
}

fn to_array<'a, S: Scope<'a>>(scope: &mut S, vec: &[String]) -> JsResult<'a, JsArray> {
    let arr = JsArray::new(scope, vec.len() as u32);

    for (index, item) in vec.iter().enumerate() {
        let string = JsString::new_or_throw(scope, item)?;
        (*arr).set(index as u32, string)?;
    }

    Ok(arr)
}

fn send<'a, T>(sender: &mpsc::SyncSender<T>, data: T) -> JsResult<'a, JsValue> {
    if let Err(_) = sender.send(data) {
        JsError::throw(Kind::Error, "channel closed")
    } else {
        Ok(JsUndefined::new().upcast())
    }
}

pub(crate) struct ErrorWrapper {
    handle: PersistentHandle,
}

impl ErrorWrapper {
    pub(crate) fn new(handle: Handle<JsValue>) -> ErrorWrapper {
        ErrorWrapper {
            handle: PersistentHandle::new(handle),
        }
    }

    pub(crate) fn into_handle<'a, S: Scope<'a>>(self, scope: &mut S) -> Handle<'a, JsValue> {
        self.handle.into_handle(scope)
    }
}

impl fmt::Display for ErrorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "error wrapper")
    }
}

impl fmt::Debug for ErrorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "error wrapper")
    }
}

impl ::std::error::Error for ErrorWrapper {
    fn description(&self) -> &'static str {
        "v8 error value wrapper"
    }
}

fn noop<'a, S: Scope<'a>, T: 'static>(
    s: &mut S, 
    sender: mpsc::SyncSender<Result<T, ErrorWrapper>>) -> JsResult<'a, JsFunction> 
{
    JsFunction::new(s, Box::new(move |call| {
        let error = call.arguments.require(call.scope, 0)?;
        if !error.is_a::<JsUndefined>() && !error.is_a::<JsNull>() {
            debug!("unhandled error occured in dispatcher");
            return send(&sender, Err(ErrorWrapper::new(error)));
        }

        Ok(JsUndefined::new().upcast())
    }))
}
