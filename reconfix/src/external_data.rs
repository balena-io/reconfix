//! External data nodes.
//!
//! Interaction with data outside a transformation graph is abstracted through
//! objects implementing the [`ExternalData`] trait. This trait defines an
//! asynchronous, bidirectional, event-based interface that communicates
//! using JSON values.

use crate::{Error, Result};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use std::{marker::PhantomData, result, sync::Arc};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    oneshot, Mutex,
};

/// Trait for data that is synchronized with an external resource.
///
/// Implementors should use [`async_trait`](async_trait::async_trait). For an
/// example implementation see [`InMemoryExternalData`].
///
/// **TODO: would be nice to allow implementors to publish their schema**
#[async_trait]
pub trait ExternalData {
    /// Spawn a task that listens for changes from some external data
    /// repository and pushes new values into the transformation graph as they
    /// happen, through [`Synchronizer::apply`]. The implementor must return
    /// the initial value for the data it represents.
    ///
    /// It is not required that implementors support repeated calls to
    /// `listen()` after it returned `Ok(())` once.
    async fn listen(
        &self,
        synchronizer: Synchronizer,
    ) -> anyhow::Result<Arc<Value>>;

    /// Commit a new value to the external resource. A successful commit
    /// may cycle back into a call to [`Synchronizer::apply`].
    async fn commit(&self, new_value: &Arc<Value>) -> anyhow::Result<()>;
}

#[async_trait]
impl<'a, T> ExternalData for &'a T
where
    T: ExternalData + Sync + 'a,
{
    async fn listen(
        &self,
        synchronizer: Synchronizer,
    ) -> anyhow::Result<Arc<Value>> {
        T::listen(self, synchronizer).await
    }

    async fn commit(&self, new_value: &Arc<Value>) -> anyhow::Result<()> {
        T::commit(self, new_value).await
    }
}

/// Object responsible for transporting new values from [`ExternalData`]
/// objects into a transformation graph.
pub struct Synchronizer {
    id: usize,
    sink: Sender<SynchronizationRequest>,
}

impl Synchronizer {
    pub(crate) fn new(id: usize) -> (Self, Receiver<SynchronizationRequest>) {
        let (sink, source) = mpsc::channel(1);

        (Self { id, sink }, source)
    }

    /// Apply and propagate a new value inside the transformation graph. This
    /// function will fail if the transformation graph is being shut down or
    /// if the new value fails to synchronize due to conflicts.
    ///
    /// It is safe to call this function again before a previous call fully
    /// resolves as all values are serialized before being synchronized.
    ///
    /// No changes are commited if `apply()` fails.
    pub async fn apply<A>(&self, new_value: A) -> ApplyResult
    where
        A: Into<Arc<Value>>,
    {
        let (request, response_source) =
            SynchronizationRequest::new(self.id, new_value.into());
        self.sink
            .send(request)
            .await
            .map_err(|_| ApplyError::ShuttingDown)?;
        response_source
            .await
            .map_err(|_| ApplyError::ShuttingDown)??;

        Ok(())
    }
}

/// Possible error values for a failed [`Synchronizer::apply`] call.
#[derive(Debug, thiserror::Error)]
pub enum ApplyError {
    /// The new value caused a conflict within the transformation graph.
    #[error("new value conflicts: {0}")]
    NewValueConflicts(#[from] anyhow::Error),

    /// The transformation graph is being shut down and so no more values can
    /// be pushed.
    #[error("the transformation graph is being shut down")]
    ShuttingDown,
}

/// [`Result`](std::result::Result) type for [`Synchronizer::apply`].
pub type ApplyResult = result::Result<(), ApplyError>;

pub(crate) struct SynchronizationRequest {
    pub(crate) id: usize,
    pub(crate) new_value: Arc<Value>,
    pub(crate) response_sink: oneshot::Sender<ApplyResult>,
}

impl SynchronizationRequest {
    pub(crate) fn new(
        id: usize,
        new_value: Arc<Value>,
    ) -> (Self, oneshot::Receiver<ApplyResult>) {
        let (sink, source) = oneshot::channel();

        (
            Self {
                id,
                new_value,
                response_sink: sink,
            },
            source,
        )
    }
}

/// A helper struct that allows in-memory data to serve as [`ExternalData`]
/// nodes.
///
/// This struct wraps JSON-compatible [`Serialize`](serde::Serialize) data and
/// exposes it through the [`ExternalData`] trait. Internally, this data is
/// kept as a JSON [`Value`][serde_json::Value], and thus is not as performant
/// as more specialized implementations might be.
///
/// This struct is usually used by keeping a clone and interacting with it
/// through [`get_cloned`](InMemoryExternalData::get_cloned) and
/// [`set_cloned`](InMemoryExternalData::set_cloned).
#[derive(Clone)]
pub struct InMemoryExternalData<T> {
    inner: Arc<Mutex<InMemoryExternalDataInner<T>>>,
}

impl<T> InMemoryExternalData<T> {
    /// Check whether [`listen`](InMemoryExternalData::listen) has been
    /// successfully called on this struct.
    ///
    /// If this function returns true, any subsequent call to
    /// [`set_cloned`](InMemoryExternalData::set_cloned) will push the new
    /// value into the associated transformation graph.
    pub async fn is_listening(&self) -> bool {
        self.inner.lock().await.synchronizer.is_some()
    }

    /// Stop sending new values into the transformation graph.
    ///
    /// The orchestrator may still call
    /// [`commit`](InMemoryExternalData::commit).
    pub async fn unlisten(&self) {
        self.inner.lock().await.synchronizer.take();
    }
}

impl<T> InMemoryExternalData<T>
where
    T: Serialize,
{
    /// Build a new [`InMemoryExternalData`] with the given initial data.
    pub fn new(data: &T) -> Result<Self> {
        Ok(Self {
            inner: Arc::new(Mutex::new(InMemoryExternalDataInner {
                data: Arc::new(
                    serde_json::to_value(data)
                        .map_err(Error::JsonSerializationError)?,
                ),
                synchronizer: None,

                _phantom: PhantomData,
            })),
        })
    }

    /// Set the wrapped value for this struct.
    ///
    /// The new value will be pushed into the transformation graph if
    /// [`listen`](InMemoryExternalData::listen) has been called on this
    /// struct. In case this fails, `set_cloned()` returns an error.
    pub async fn set_cloned(&self, new_data: &T) -> Result<&Self> {
        let new_data = Arc::new(
            serde_json::to_value(new_data)
                .map_err(Error::JsonSerializationError)?,
        );
        let mut inner = self.inner.lock().await;
        if let Some(synchronizer) = &inner.synchronizer {
            synchronizer.apply(new_data.clone()).await?;
        }
        inner.data = new_data;

        Ok(self)
    }
}

impl<T> InMemoryExternalData<T>
where
    T: DeserializeOwned,
{
    /// Get a deserialized clone of the wrapped data.
    pub async fn get_cloned(&self) -> Result<T> {
        serde_json::from_value((*self.inner.lock().await.data).clone())
            .map_err(Error::JsonDeserializationError)
    }
}

#[async_trait]
impl<T> ExternalData for InMemoryExternalData<T>
where
    T: Send + 'static,
{
    async fn listen(
        &self,
        synchronizer: Synchronizer,
    ) -> anyhow::Result<Arc<Value>> {
        let mut inner = self.inner.lock().await;
        inner.synchronizer = Some(synchronizer);

        Ok(inner.data.clone())
    }

    async fn commit(&self, new_value: &Arc<Value>) -> anyhow::Result<()> {
        self.inner.lock().await.data = new_value.clone();

        Ok(())
    }
}

struct InMemoryExternalDataInner<T> {
    data: Arc<Value>,
    synchronizer: Option<Synchronizer>,
    _phantom: PhantomData<T>,
}
