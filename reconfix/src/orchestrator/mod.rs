//! Build and run transformation graphs.
//!
//! A transformation graph is an undirected graph where nodes are either
//! [`Lens`](crate::Lens)es or [`ExternalData`](crate::ExternalData) instances.
//! [`Orchestrator`] encapsulates a transformation graph and everything
//! required to execute it.

use crate::{
    external_data::{ApplyError, ApplyResult, PatchRequest, Patcher},
    lens::Lens,
    Error, ExternalData, Result,
};
use futures::{
    future,
    stream::{self, FuturesUnordered, StreamExt},
};
use json_patch::{AddOperation, Patch, PatchOperation, ReplaceOperation};
use petgraph::{graph::NodeIndex, Directed, Direction, Graph};
use serde_json::Value;
use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    sync::Arc,
};
use tokio::sync::{mpsc::Receiver, Mutex};
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(test)]
mod test;

/// A node in a transformation graph.
pub enum Node<'a> {
    /// A [`Lens`](crate::Lens) providing bidirectional transformation between
    /// nodes it is connected to.
    Lens(Lens),

    /// An [`ExternalData`](crate::ExternalData) instance asynchronously
    /// providing and accepting patches in accordance with some external data
    /// repository.
    ExternalData(Box<dyn ExternalData + Send + Sync + 'a>),
}

impl<'a> Node<'a> {
    fn is_external_data(&self) -> bool {
        matches!(self, Self::ExternalData(_))
    }
}

impl<'a> From<Lens> for Node<'a> {
    fn from(x: Lens) -> Self {
        Self::Lens(x)
    }
}

impl<'a, E> From<E> for Node<'a>
where
    E: ExternalData + Send + Sync + 'a,
{
    fn from(x: E) -> Self {
        Self::ExternalData(Box::new(x))
    }
}

/// Wrapper and executor for transformation graphs.
#[derive(Default)]
pub struct Orchestrator<'a> {
    graph: Graph<Node<'a>, (), Directed, usize>,
    external_data_nodes: Vec<NodeIndex<usize>>,
}

impl<'a> Orchestrator<'a> {
    /// Create a new, empty [`Orchestrator`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a node to the transformation graph. Returns a handle that can be
    /// used to connect nodes together.
    pub fn add_node<N>(&mut self, node: N) -> NodeHandle
    where
        N: Into<Node<'a>>,
    {
        let node = node.into();
        let is_external_data_node = node.is_external_data();
        let index = self.graph.add_node(node);
        if is_external_data_node {
            self.external_data_nodes.push(index);
        }

        NodeHandle(index)
    }

    /// Connect node `a` into node `b`.
    ///
    /// Edge direction is important, with specific semantics depending on the
    /// types of nodes being connected:
    ///
    /// - `a` and `b` are [`ExternalData`] nodes: `a` and `b` are unified.
    /// - `a` is an [`ExternalData`] node and `b` is a [`Lens`] node: `a` is
    ///   unified with `b.X`.
    /// - `a` is a [`Lens`] node and `b` is an [`ExternalData`] node: `a.Y` is
    ///   unified with `b`.
    /// - `a` and `b` are [`Lens`] nodes: `a.X` is unified with `b.Y`.
    ///
    /// # Panics
    ///
    /// This function will panic if either `a` or `b` are invalid.
    pub fn add_edge(&mut self, a: NodeHandle, b: NodeHandle) -> &mut Self {
        self.graph.add_edge(a.0, b.0, ());

        self
    }

    /// Run the transformation graph asynchronously.
    ///
    /// **TODO: might be useful to expose a separate `init` method**
    pub async fn run(&self) -> Result<()> {
        if self.external_data_nodes.is_empty() {
            // Running a transformation graph without a single external data
            // node is just a no op
            return Ok(());
        }

        // Setup all external data nodes. These will drive the tranformation
        // graph
        let (external_data, patch_sources) =
            self.start_external_data_nodes().await?;

        // Run an initial check to make sure all external data nodes are
        // coherent
        // TODO: check if there are any patches too. Also, have to take into
        // account disconnected subgraphs
        self.propagate_external_value(
            self.external_data_nodes[0],
            external_data
                .get(&self.external_data_nodes[0].index())
                .expect("BUG: external data node has not been setup"),
            &external_data,
        )?;

        // Setup some shared state for the runner task. Wrapping these in a
        // mutex is not strictly necessary because all patch requests are
        // processed serially and thus these are never contended. But because
        // `Stream::for_each` (see below) takes a future there's no way to
        // prove this fact to the type system. Overhead is minimal though
        let external_data = Mutex::new(external_data);
        let external_node_gens = Mutex::new(
            self.external_data_nodes
                .iter()
                .map(|x| (x.index(), 0_usize))
                .collect::<HashMap<_, _>>(),
        );

        // Listen for patch requests and apply them as they come. We do this by
        // multiplexing all channels `Patcher`s push `PatchRequest`s into into
        // a single asynchronous `Stream`. We then process each patch request
        // serially as they come. It would be unsafe to process patch requests
        // concurrently since different patches may end up creating conflicting
        // internal states
        let external_data = &external_data;
        let external_node_gens = &external_node_gens;
        stream::select_all(patch_sources.into_iter().map(|patch_source| {
            // Convert a `Receiver` into a `Stream` to make it easier to poll
            // them concurrently
            Box::pin(stream::unfold(patch_source, |mut patch_source| async {
                patch_source.recv().await.map(|patch| (patch, patch_source))
            }))
        }))
        .for_each(|request| async move {
            // TODO: increase `gen` on error
            let _ = request.response_sink.send(
                self.process_request(
                    request.id,
                    request.gen,
                    &request.patch,
                    &*external_node_gens.lock().await,
                    &mut *external_data.lock().await,
                )
                .await,
            );
        })
        .await;

        Ok(())
    }

    async fn start_external_data_nodes(
        &self,
    ) -> Result<(HashMap<usize, Value>, Vec<Receiver<PatchRequest>>)> {
        // Initialize all external data nodes concurrently. It would be better
        // to spawn each in its own task to potentially run all initializations
        // in parallel, but that would create a dependency on a specific
        // executor
        let mut external_data_init = self
            .external_data_nodes
            .iter()
            .copied()
            .map(move |index| self.start_external_data_node(index))
            .collect::<FuturesUnordered<_>>();
        let mut initial_data = HashMap::new();
        let mut patch_sources = Vec::new();
        while let Some(res) = external_data_init.next().await {
            let (index, initial_value, source) = res?;
            initial_data.insert(index, initial_value);
            patch_sources.push(source);
        }

        Ok((initial_data, patch_sources))
    }

    async fn start_external_data_node(
        &self,
        index: NodeIndex<usize>,
    ) -> Result<(usize, Value, Receiver<PatchRequest>)> {
        // Create a `Patcher` and ask this node to use it through the
        // `listen()` method
        let id = index.index();
        let (patcher, mut source) = Patcher::new(id);
        let node = self
            .graph
            .node_weight(index)
            .expect("BUG: saved external data node index is invalid");
        let listen_future = async {
            match node {
                Node::ExternalData(external_data) => {
                    external_data
                        .listen(patcher)
                        .await
                        .map_err(|x| Error::ListenError(id, x))?
                }
                _ => panic!("BUG: saved external data node does not point to an external data node"),
            }

            Result::Ok(())
        };

        // Get the first patch and use that as the initial value for
        // this node
        let initial_value_future = async {
            Ok(match source.recv().await {
                Some(request) => {
                    assert_eq!(request.id, id);
                    assert_eq!(request.gen, 0);
                    match Self::process_initial_patch(id, request.patch) {
                        Ok(value) => {
                            let _ = request.response_sink.send(Ok(()));

                            value
                        }
                        Err(err) => {
                            let _ = request
                                .response_sink
                                .send(Err(ApplyError::InvalidInitialPatch));

                            return Err(err);
                        }
                    }
                }
                None => Value::Null,
            })
        };

        // Since `listen()` can await on the first call to `Patcher::apply()`,
        // awaiting on it could cause a deadlock. On the other hand if
        // `listen()` doesn't await on it, awaiting fist on the first patch
        // would cause a deadlock too. Thus the only correct way is to await on
        // both concurrently
        let (listen_res, initial_value) =
            future::join(listen_future, initial_value_future).await;
        listen_res?;

        Ok((id, initial_value?, source))
    }

    fn process_initial_patch(id: usize, mut patch: Patch) -> Result<Value> {
        if patch.0.len() != 1 {
            return Err(Error::InvalidInitialPatch(id, patch));
        }

        match patch.0.pop() {
            Some(PatchOperation::Add(AddOperation { path, value }))
                if path.is_empty() =>
            {
                Ok(value)
            }
            x => Err(Error::InvalidInitialPatch(id, Patch(vec![x.unwrap()]))),
        }
    }

    async fn process_request(
        &self,
        id: usize,
        gen: usize,
        patch: &Patch,
        external_node_gens: &HashMap<usize, usize>,
        external_data: &mut HashMap<usize, Value>,
    ) -> ApplyResult {
        // Ignore the request if its `gen` is lower than expected. See the docs
        // for `Patcher::apply()` for the explanation
        let expected_gen = *external_node_gens
            .get(&id)
            .expect("BUG: received a patch request with an unknown ID");
        assert!(gen <= expected_gen);
        if gen < expected_gen {
            return Err(ApplyError::SequenceError);
        }

        // Try to apply the patch into a copy of the current value for the
        // specified external data node
        let mut patched_value = external_data.get(&id).unwrap().clone();
        json_patch::patch_unsafe(&mut patched_value, patch)
            .map_err(|x| ApplyError::PatchConflicts(x.into()))?;

        // Try to propagate changes
        let patches = self
            .propagate_external_value(
                NodeIndex::from(id),
                &patched_value,
                external_data,
            )
            .map_err(|x| ApplyError::PatchConflicts(x.into()))?;

        // If it all goes well, we can commit all changes concurrently
        external_data.insert(id, patched_value);
        patches
            .into_iter()
            .map(|(index, mut patch)| {
                // JSON patch generates an invalid patch if the whole document
                // has been replaced. Fix that here before applying the patch.
                // See https://github.com/idubrov/json-patch/issues/12
                for operation in &mut patch.0 {
                    match operation {
                        PatchOperation::Replace(ReplaceOperation {
                            ref mut path,
                            ..
                        }) => *path = String::new(),
                        _ => unimplemented!(),
                    }
                }

                json_patch::patch_unsafe(
                    external_data.get_mut(&index.index()).unwrap(),
                    &patch,
                )
                .unwrap();
                let is_empty = patch.0.is_empty();

                async move {
                    if is_empty {
                        return;
                    }

                    // TODO: handle `commit()` failures
                    if let Node::ExternalData(external_data_node) =
                        self.graph.node_weight(index).unwrap()
                    {
                        external_data_node.commit(patch).await.unwrap();
                    } else {
                        unreachable!();
                    }
                }
            })
            .collect::<FuturesUnordered<_>>()
            .for_each(|_| async {})
            .await;

        Ok(())
    }

    fn propagate_external_value(
        &self,
        index: NodeIndex<usize>,
        set_to: &Value,
        external_data: &HashMap<usize, Value>,
    ) -> Result<Vec<(NodeIndex<usize>, Patch)>> {
        assert!(matches!(
            self.graph.node_weight(index).unwrap(),
            Node::ExternalData(_)
        ));

        // Save `set_to` as the resolved value for the external data node
        // pointed by `index`
        let mut resolved = HashMap::new();
        resolved.insert(
            index,
            ResolvedNode::ExternalData(Arc::new(set_to.clone())),
        );

        // Build an initial queue of nodes that can be resolved given the above
        let mut resolvable = self
            .graph
            .neighbors_undirected(index)
            .map(|x| (index, x))
            .collect::<Vec<_>>();

        // Build an initial set for edges that have already been traversed or
        // have already been queued for traversal. This is a set of
        // `(node_a_index, node_b_index)` pairs where
        // `node_a_index < node_b_index`
        let mut traversed = self
            .graph
            .neighbors_undirected(index)
            .map(|x| (x.min(index), x.max(index)))
            .collect::<HashSet<_>>();

        // Exhaust the `resolvable` queue by filling the `resolved` map with
        // concrete values and checking for internal consistency
        // TODO: we can actually build some pretty good errors here
        let mut patches = Vec::new();
        while let Some((resolved_index, resolvable_index)) = resolvable.pop() {
            let (_, direction) = self
                .graph
                .find_edge_undirected(resolved_index, resolvable_index)
                .expect("BUG: trying to resolve unconnected nodes");

            let resolved_data = resolved.get(&resolved_index).unwrap().clone();
            match resolved.entry(resolvable_index) {
                Entry::Occupied(entry) => {
                    // If the node at `resolvable_index` is already resolved we
                    // only need to check for conflicts
                    Self::check_conflict(
                        resolved_index.index(),
                        &resolved_data,
                        direction,
                        resolvable_index.index(),
                        entry.get(),
                    )?;
                }
                Entry::Vacant(entry) => {
                    // Otherwise, resolve the node and enqueue all untraversed
                    // edges

                    let (resolvable_data, patch) = self.resolve(
                        external_data,
                        &resolved_data,
                        direction,
                        resolvable_index,
                    )?;
                    entry.insert(resolvable_data);

                    if let Some(patch) = patch {
                        if !patch.0.is_empty() {
                            patches.push((resolvable_index, patch));
                        }
                    }

                    for x in self.graph.neighbors_undirected(resolvable_index) {
                        if traversed.insert((
                            x.min(resolvable_index),
                            x.max(resolvable_index),
                        )) {
                            resolvable.push((resolvable_index, x));
                        }
                    }
                }
            }
        }

        Ok(patches)
    }

    fn check_conflict(
        left_id: usize,
        left: &ResolvedNode,
        direction: Direction,
        right_id: usize,
        right: &ResolvedNode,
    ) -> Result<()> {
        let (left_value, right_value) = match (left, direction, right) {
            (
                ResolvedNode::ExternalData(left_value),
                _,
                ResolvedNode::ExternalData(right_value),
            ) => (left_value, right_value),
            (
                ResolvedNode::ExternalData(left_value),
                Direction::Outgoing,
                ResolvedNode::XY(_, right_y_value),
            ) => (left_value, right_y_value),
            (
                ResolvedNode::ExternalData(left_value),
                Direction::Incoming,
                ResolvedNode::XY(right_x_value, _),
            ) => (left_value, right_x_value),
            (
                ResolvedNode::XY(left_x_value, _),
                Direction::Outgoing,
                ResolvedNode::ExternalData(right_value),
            ) => (left_x_value, right_value),
            (
                ResolvedNode::XY(_, left_y_value),
                Direction::Incoming,
                ResolvedNode::ExternalData(right_value),
            ) => (left_y_value, right_value),
            (
                ResolvedNode::XY(_, left_y_value),
                Direction::Outgoing,
                ResolvedNode::XY(right_x_value, _),
            ) => (left_y_value, right_x_value),
            (
                ResolvedNode::XY(left_x_value, _),
                Direction::Incoming,
                ResolvedNode::XY(_, right_y_value),
            ) => (left_x_value, right_y_value),
        };

        if left_value == right_value {
            Ok(())
        } else {
            Err(Error::Conflict(
                left_id,
                left_value.clone(),
                right_id,
                right_value.clone(),
            ))
        }
    }

    fn resolve(
        &self,
        external_data: &HashMap<usize, Value>,
        left: &ResolvedNode,
        direction: Direction,
        right_index: NodeIndex<usize>,
    ) -> Result<(ResolvedNode, Option<Patch>)> {
        let right_id = right_index.index();

        match (
            left,
            direction,
            self.graph.node_weight(right_index).unwrap(),
        ) {
            (
                ResolvedNode::ExternalData(left_value),
                _,
                Node::ExternalData(_),
            ) => Ok((
                ResolvedNode::ExternalData(left_value.clone()),
                Some(json_patch::diff(
                    external_data.get(&right_id).unwrap(),
                    left_value,
                )),
            )),
            (
                ResolvedNode::ExternalData(left_value),
                Direction::Outgoing,
                Node::Lens(lens),
            ) => Ok((
                ResolvedNode::XY(
                    left_value.clone(),
                    Arc::new(cuelang_value_to_json_value(
                        &lens.apply_x(&**left_value)?,
                    )?),
                ),
                None,
            )),
            (
                ResolvedNode::ExternalData(left_value),
                Direction::Incoming,
                Node::Lens(lens),
            ) => Ok((
                ResolvedNode::XY(
                    Arc::new(cuelang_value_to_json_value(
                        &lens.apply_y(&**left_value)?,
                    )?),
                    left_value.clone(),
                ),
                None,
            )),
            (
                ResolvedNode::XY(_, left_y_value),
                Direction::Outgoing,
                Node::ExternalData(_),
            ) => Ok((
                ResolvedNode::ExternalData(left_y_value.clone()),
                Some(json_patch::diff(
                    external_data.get(&right_id).unwrap(),
                    left_y_value,
                )),
            )),
            (
                ResolvedNode::XY(left_x_value, _),
                Direction::Incoming,
                Node::ExternalData(_),
            ) => Ok((
                ResolvedNode::ExternalData(left_x_value.clone()),
                Some(json_patch::diff(
                    external_data.get(&right_id).unwrap(),
                    left_x_value,
                )),
            )),
            (
                ResolvedNode::XY(_, left_y_value),
                Direction::Outgoing,
                Node::Lens(lens),
            ) => Ok((
                ResolvedNode::XY(
                    left_y_value.clone(),
                    Arc::new(cuelang_value_to_json_value(
                        &lens.apply_x(&**left_y_value)?,
                    )?),
                ),
                None,
            )),
            (
                ResolvedNode::XY(left_x_value, _),
                Direction::Incoming,
                Node::Lens(lens),
            ) => Ok((
                ResolvedNode::XY(
                    Arc::new(cuelang_value_to_json_value(
                        &lens.apply_y(&**left_x_value)?,
                    )?),
                    left_x_value.clone(),
                ),
                None,
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ResolvedNode {
    ExternalData(Arc<Value>),
    XY(Arc<Value>, Arc<Value>),
}

/// Handle to a node inserted into an [`Orchestrator`].
///
/// This handle is specific to the [`Orchestrator`] it was created from.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeHandle(NodeIndex<usize>);

fn cuelang_value_to_json_value(
    cuelang_value: &cuelang::Value,
) -> Result<Value> {
    cuelang_value
        .as_json_value()
        .ok_or(Error::ValueConversionError)
}
