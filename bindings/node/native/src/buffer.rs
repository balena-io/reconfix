
use std::cmp;
use std::mem;

use neon::js::{JsUndefined, JsNull, JsValue, JsNumber, JsFunction };
use neon::js::binary::JsBuffer;
use neon::scope::Scope;
use neon::task::Task;
use neon::vm::{JsResult, Lock};

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
        }

        method write(call) {
            let scope = call.scope;
            let mut chunk = call.arguments.require(scope, 0)?.check::<JsBuffer>()?;

            let buffer = chunk.grab(|inner| {
                let mut tmp = Vec::with_capacity(inner.len());
                tmp.extend_from_slice(inner.as_slice());
                tmp
            });

            call.arguments.this(scope).grab(move |inner| {
                inner.buf.extend_from_slice(&buffer);
            });

            Ok(JsNull::new().upcast())
        }
    }
}
