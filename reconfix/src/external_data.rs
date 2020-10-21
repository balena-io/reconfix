//! External data nodes.
//!
//! Interaction with data outside a transformation graph is abstracted through
//! objects implementing the [`ExternalData`] trait. This trait defines an
//! asynchronous, bidirectional, event-based interface that communicates
//! using JSON patches.

use crate::{Error, Result};
use async_trait::async_trait;
use json_patch::{AddOperation, Patch, PatchOperation, ReplaceOperation};
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
    /// repository and pushes deltas into `patcher` as they happen. The
    /// implementor must set the initial value for the data it represents with
    /// a single, root [`AddOperation`](json_patch::AddOperation) before
    /// sending delta patches.
    ///
    /// It is not required that implementors support repeated calls to
    /// `listen()` after it returned `Ok(())` once.
    ///
    /// **TODO: actually this probably should take `self` by value. Would
    /// require splitting this trait into two though**
    async fn listen(&self, patcher: Patcher) -> anyhow::Result<()>;

    /// Commit a patch to the external resource. A successful commit
    /// **must not** cycle back into a call to [`Patcher::apply`].
    async fn commit(&self, patch: Patch) -> anyhow::Result<()>;
}

#[async_trait]
impl<'a, T> ExternalData for &'a T
where
    T: ExternalData + Sync + 'a,
{
    async fn listen(&self, patcher: Patcher) -> anyhow::Result<()> {
        T::listen(self, patcher).await
    }

    async fn commit(&self, patch: Patch) -> anyhow::Result<()> {
        T::commit(self, patch).await
    }
}

/// Object responsible for transporting changes from [`ExternalData`] objects
/// into the transformation graph.
pub struct Patcher {
    id: usize,
    gen: usize,
    sink: Sender<PatchRequest>,
}

impl Patcher {
    pub(crate) fn new(id: usize) -> (Self, Receiver<PatchRequest>) {
        let (sink, source) = mpsc::channel(1);

        (Self { id, gen: 0, sink }, source)
    }

    /// Apply and propagate a patch inside the transformation graph. This
    /// function will fail if the transformation graph is being shut down or
    /// if the patch fails to synchronize due to conflicts.
    ///
    /// It is safe to call this function again before a previous call fully
    /// resolves as all patches are serialized before being applied. But in
    /// this case, if one patch fails all other pending patches from this
    /// [`Patcher`] will also fail with a [`ApplyError::SequenceError`] error.
    /// This is done to avoid desynchronization and the caller must handle
    /// this case gracefully.
    ///
    /// No changes are commited if `apply()` fails.
    pub async fn apply(&self, patch: Patch) -> ApplyResult {
        let (request, response_source) =
            PatchRequest::new(self.id, self.gen, patch);
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

/// Possible error values for a failed [`Patcher::apply`] call.
#[derive(Debug, thiserror::Error)]
pub enum ApplyError {
    /// This is an initial patch and it was invalid.
    #[error("initial patch was invalid")]
    InvalidInitialPatch,

    /// The patch caused a conflict within the transformation graph.
    #[error("patch conflicts: {0}")]
    PatchConflicts(#[from] anyhow::Error),

    /// Patch was skipped due to a previous enqueued patch from the same
    /// [`Patcher`] failing.
    #[error("correlated patch failed")]
    SequenceError,

    /// The transformation graph is being shut down and so no more patches can
    /// be applied.
    #[error("transformation graph is being shut down")]
    ShuttingDown,
}

/// [`Result`](std::result::Result) type for [`Patcher::apply`].
pub type ApplyResult = result::Result<(), ApplyError>;

pub(crate) struct PatchRequest {
    pub(crate) id: usize,
    pub(crate) gen: usize,
    pub(crate) patch: Patch,
    pub(crate) response_sink: oneshot::Sender<ApplyResult>,
}

impl PatchRequest {
    pub(crate) fn new(
        id: usize,
        gen: usize,
        patch: Patch,
    ) -> (Self, oneshot::Receiver<ApplyResult>) {
        let (sink, source) = oneshot::channel();

        (
            Self {
                id,
                gen,
                patch,
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
    /// [`set_cloned`](InMemoryExternalData::set_cloned) will push a patch into
    /// the associated transformation graph.
    pub async fn is_listening(&self) -> bool {
        self.inner.lock().await.patcher.is_some()
    }

    /// Stop sending patches into the transformation graph.
    ///
    /// The orchestrator may still call
    /// [`commit`](InMemoryExternalData::commit).
    pub async fn unlisten(&self) {
        self.inner.lock().await.patcher.take();
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
                data: serde_json::to_value(data)
                    .map_err(Error::JsonSerializationError)?,
                patcher: None,

                _phantom: PhantomData,
            })),
        })
    }

    /// Set the wrapped value for this struct.
    ///
    /// A patch will be pushed into the transformation graph if
    /// [`listen`](InMemoryExternalData::listen) has been called on this
    /// struct. In case this fails, `set_cloned()` returns an error.
    pub async fn set_cloned(&self, new_data: &T) -> Result<&Self> {
        let new_data = serde_json::to_value(new_data)
            .map_err(Error::JsonSerializationError)?;
        let mut inner = self.inner.lock().await;
        if let Some(patcher) = &inner.patcher {
            patcher
                .apply(Patch(vec![PatchOperation::Replace(ReplaceOperation {
                    path: String::new(),
                    value: new_data.clone(),
                })]))
                .await?;
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
        serde_json::from_value(self.inner.lock().await.data.clone())
            .map_err(Error::JsonDeserializationError)
    }
}

#[async_trait]
impl<T> ExternalData for InMemoryExternalData<T>
where
    T: Send + 'static,
{
    async fn listen(&self, patcher: Patcher) -> anyhow::Result<()> {
        let mut inner = self.inner.lock().await;
        patcher
            .apply(Patch(vec![PatchOperation::Add(AddOperation {
                path: String::new(),
                value: inner.data.clone(),
            })]))
            .await
            .unwrap();
        inner.patcher = Some(patcher);

        Ok(())
    }

    async fn commit(&self, patch: Patch) -> anyhow::Result<()> {
        json_patch::patch(&mut self.inner.lock().await.data, &patch)?;

        Ok(())
    }
}

struct InMemoryExternalDataInner<T> {
    data: Value,
    patcher: Option<Patcher>,
    _phantom: PhantomData<T>,
}
