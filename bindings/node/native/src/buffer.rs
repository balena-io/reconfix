
use std::cmp;
use std::mem;

use neon::js::{JsUndefined, JsNull, JsValue, JsNumber, JsFunction };
use neon::js::binary::JsBuffer;
use neon::scope::Scope;
use neon::task::Task;
use neon::vm::{JsResult, Lock};

struct VoidTask;

impl Task for VoidTask {
    type Output = ();
    type Error = ();
    type JsEvent = JsUndefined;

    fn perform(&mut self) -> Result<Self::Output, Self::Error> {
        Ok(())
    }

    fn complete<'a, S: Scope<'a>>(self, _: &'a mut S, _: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
        Ok(JsUndefined::new())
    }
}

struct ChunkTask {
    chunk: Vec<u8>,
}

impl ChunkTask {
    pub fn new(data: Vec<u8>) -> ChunkTask {
        ChunkTask { chunk: data }
    }
}

impl Task for ChunkTask {
    type Output = ();
    type Error = ();
    type JsEvent = JsValue;

    fn perform(&mut self) -> Result<Self::Output, Self::Error> {
        Ok(())
    }

    fn complete<'a, S: Scope<'a>>(self, scope: &'a mut S, _: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
        // if self.chunk.len() == 0 {
        //     debug!("interpreting empty chunk as end");
        //     return Ok(JsNull::new().upcast())
        // }

        let mut chunk = JsBuffer::new(scope, self.chunk.len() as u32)?;
        chunk.grab(|mut inner| {
            inner.as_mut_slice().clone_from_slice(self.chunk.as_slice());
        });

        debug!("returning copied data");
        Ok(chunk.upcast())
    }
}

pub struct BufferStream {
    pub(crate) buf: Vec<u8>
}

declare_types! {
    pub class JsBufferStream for BufferStream {
        init(_) {
            Ok(BufferStream {
                buf: Vec::new(),
            })
        }

        method read(call) {
            let scope = call.scope;
            let size = call.arguments.require(scope, 0)?.check::<JsNumber>()?.value() as usize;
            //let push = call.arguments.require(scope, 1)?.check::<JsFunction>()?;

            let tmp_buf = call.arguments.this(scope).grab(move |inner| {
                let to_drain = cmp::min(inner.buf.len(), size);
                if (to_drain == 0) {
                    return None;
                }

                debug!("draining {} bytes from buffer", to_drain);
                let read: Vec<u8> = inner.buf.drain(..to_drain).collect();
                Some(read)
            });

            tmp_buf.map(|buffer| {
                let mut chunk = JsBuffer::new(scope, buffer.len() as u32)?;
                chunk.grab(|mut inner| {
                    inner.as_mut_slice().clone_from_slice(buffer.as_slice());
                });
                Ok(chunk.upcast())
            }).unwrap_or_else(|| Ok(JsNull::new().upcast()))

            //debug!("scheduling buffer read callback");
            //ChunkTask::new(tmp_buf).schedule(push);
        }

        method write(call) {
            let scope = call.scope;
            let mut chunk = call.arguments.require(scope, 0)?.check::<JsBuffer>()?;
            let callback = call.arguments.require(scope, 2)?.check::<JsFunction>()?;

            let buffer = chunk.grab(|inner| {
                let mut tmp = Vec::with_capacity(inner.len());
                tmp.extend_from_slice(inner.as_slice());
                tmp
            });

            call.arguments.this(scope).grab(move |inner| {
                inner.buf.extend_from_slice(&buffer);
            });

            (VoidTask {}).schedule(callback);

            Ok(JsUndefined::new().upcast())
        }
    }
}