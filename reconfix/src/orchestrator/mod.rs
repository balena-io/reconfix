//! Build and run transformation graphs.
//!
//! A transformation graph is an directed graph where nodes are either
//! [`Lens`](crate::Lens)es or [`ExternalData`](crate::ExternalData) instances.
//! The [`Orchestrator`] struct encapsulates a transformation graph and
//! everything required to execute it.

use crate::{
    external_data::{
        ApplyError, ApplyResult, SynchronizationRequest, Synchronizer,
    },
    lens::Lens,
    Error, ExternalData, Result,
};
use futures::stream::{self, FuturesUnordered, StreamExt};
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
    /// providing and accepting new values in accordance with some external
    /// data repository.
    ExternalData(Box<dyn ExternalData + Send + Sync + 'a>),
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
        let index = self.graph.add_node(node);

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
    /// - `a` and `b` are [`Lens`] nodes: `a.Y` is unified with `b.X`.
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
        // Setup all external data nodes. These will drive the tranformation
        // graph
        let (state_map, request_sources) =
            if let Some(x) = self.initialize().await? {
                x
            } else {
                return Ok(());
            };

        // Technically this doesn't need to be inside a mutex, but because of
        // the `for_each()` call below we would need >1 &mut references, which
        // is forbidden.
        let state_map = Mutex::new(state_map);

        // Listen for synchronization requests and apply them as they come. We
        // do this by multiplexing all `Synchronizer` channels pushing
        // `SynchronizationRequest`s into into a single asynchronous `Stream`.
        // We then process each synchronization request as they come, one by
        // one. It would be unsafe to process synchronization requests
        // concurrently since different values may end up creating conflicting
        // internal states
        let state_map = &state_map;
        stream::select_all(request_sources.into_iter().map(
            |new_value_source| {
                // Convert a `Receiver` into a `Stream` to make it easier to poll
                // them concurrently
                Box::pin(stream::unfold(
                    new_value_source,
                    |mut new_value_source| async {
                        new_value_source
                            .recv()
                            .await
                            .map(|new_value| (new_value, new_value_source))
                    },
                ))
            },
        ))
        .for_each(|request| async move {
            let _ = request.response_sink.send(
                self.process_request(
                    request.id,
                    request.new_value,
                    &mut *state_map.lock().await,
                )
                .await,
            );
        })
        .await;

        Ok(())
    }

    async fn initialize(
        &self,
    ) -> Result<
        Option<(
            HashMap<usize, Arc<Value>>,
            Vec<Receiver<SynchronizationRequest>>,
        )>,
    > {
        // First, find all nodes that hold state so we can initialize them
        let mut external_data_nodes = Vec::new();
        let mut stateful_lenses = Vec::new();
        for node_index in self.graph.node_indices() {
            match self.graph.node_weight(node_index).unwrap() {
                Node::ExternalData(external_data) => {
                    external_data_nodes.push((node_index, external_data))
                }
                Node::Lens(lens) if lens.is_stateful() => {
                    stateful_lenses.push(node_index)
                }
                _ => (),
            }
        }

        if external_data_nodes.is_empty() {
            // Running a transformation graph without a single external data
            // node is just a no op
            return Ok(None);
        }

        // Initialize all external data nodes concurrently. It would be better
        // to spawn each in its own task to potentially run all initializations
        // in parallel, but that would create a dependency on a specific
        // executor
        let mut external_data_init = external_data_nodes
            .into_iter()
            .map(move |(index, node)| {
                self.start_external_data_node(index, node)
            })
            .collect::<FuturesUnordered<_>>();
        let mut state_map = HashMap::new();
        let mut request_sources = Vec::new();
        while let Some(res) = external_data_init.next().await {
            let (index, initial_value, source) = res?;
            state_map.insert(index, initial_value);
            request_sources.push(source);
        }

        // Derive the initial state for lenses. We do this by deriving the
        // actual initial state based on the initial state of each external
        // data node in isolation, and then merging them. This process errors
        // out if the merge conflicts, or if there are lenses without an
        // initial state
        unimplemented!();

        Ok(Some((state_map, request_sources)))
    }

    async fn start_external_data_node(
        &self,
        index: NodeIndex<usize>,
        node: &Box<dyn ExternalData + Send + Sync + 'a>,
    ) -> Result<(usize, Arc<Value>, Receiver<SynchronizationRequest>)> {
        // Create a `Synchronizer` and ask this node to use it through the
        // `listen()` method
        let id = index.index();
        let (synchronizer, request_source) = Synchronizer::new(id);
        let initial_value = node
            .listen(synchronizer)
            .await
            .map_err(|x| Error::ListenError(id, x))?;

        Ok((id, initial_value, request_source))
    }

    async fn process_request(
        &self,
        id: usize,
        new_value: Arc<Value>,
        state_map: &mut HashMap<usize, Arc<Value>>,
    ) -> ApplyResult {
        // There's nothing to do if the new value is already synchronized
        if *state_map.get(&id).unwrap() == new_value {
            return Ok(());
        }

        // Try to propagate changes
        let new_states = self
            .propagate_external_value(
                NodeIndex::from(id),
                &new_value,
                state_map,
            )
            .map_err(|x| ApplyError::NewValueConflicts(x.into()))?;

        // If it all goes well, we can commit all changes concurrently
        new_states
            .iter()
            .map(|(index, new_state)| {
                async move {
                    // TODO: handle `commit()` failures
                    if let Node::ExternalData(external_data_node) =
                        self.graph.node_weight(*index).unwrap()
                    {
                        external_data_node.commit(new_state).await.unwrap();
                    }
                }
            })
            .collect::<FuturesUnordered<_>>()
            .for_each(|_| async {})
            .await;

        // Lastly, commit the new state
        *state_map.get_mut(&id).unwrap() = new_value;
        for (index, new_state) in new_states {
            *state_map.get_mut(&index.index()).unwrap() = new_state;
        }

        Ok(())
    }

    fn propagate_external_value(
        &self,
        index: NodeIndex<usize>,
        set_to: &Value,
        state_map: &HashMap<usize, Arc<Value>>,
    ) -> Result<Vec<(NodeIndex<usize>, Arc<Value>)>> {
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

        // Build an initial queue of nodes that can be resolved given the
        // above. This queue is in the form:
        // `(resolved_index, resolvable_index)`
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
        // concrete values and checking for internal consistency in case the
        // resolvable node has already been resolved
        // TODO: we can actually build some pretty good errors here
        let mut new_states = Vec::new();
        while let Some((resolved_index, resolvable_index)) = resolvable.pop() {
            let (_, direction) = self
                .graph
                .find_edge_undirected(resolved_index, resolvable_index)
                .expect(
                    "BUG: trying to propagate values through unconnected nodes",
                );

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

                    let (resolved_node, new_state) = self.resolve(
                        &resolved_data,
                        direction,
                        resolvable_index,
                        state_map,
                    )?;
                    entry.insert(resolved_node);

                    if let Some(new_state) = new_state {
                        if new_state
                            != *state_map
                                .get(&resolvable_index.index())
                                .unwrap()
                        {
                            new_states.push((resolvable_index, new_state));
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

        Ok(new_states)
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
        left: &ResolvedNode,
        direction: Direction,
        right_index: NodeIndex<usize>,
        state_map: &HashMap<usize, Arc<Value>>,
    ) -> Result<(ResolvedNode, Option<Arc<Value>>)> {
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
                Some(left_value.clone()),
            )),
            (
                ResolvedNode::ExternalData(left_value),
                Direction::Outgoing,
                Node::Lens(lens),
            ) => {
                let (y, new_state) =
                    apply_x(lens, &**left_value, state_map, right_id)?;

                Ok((ResolvedNode::XY(left_value.clone(), y), new_state))
            }
            (
                ResolvedNode::ExternalData(left_value),
                Direction::Incoming,
                Node::Lens(lens),
            ) => {
                let (x, new_state) =
                    apply_y(lens, &**left_value, state_map, right_id)?;

                Ok((ResolvedNode::XY(x, left_value.clone()), new_state))
            }
            (
                ResolvedNode::XY(_, left_y_value),
                Direction::Outgoing,
                Node::ExternalData(_),
            ) => Ok((
                ResolvedNode::ExternalData(left_y_value.clone()),
                Some(left_y_value.clone()),
            )),
            (
                ResolvedNode::XY(left_x_value, _),
                Direction::Incoming,
                Node::ExternalData(_),
            ) => Ok((
                ResolvedNode::ExternalData(left_x_value.clone()),
                Some(left_x_value.clone()),
            )),
            (
                ResolvedNode::XY(_, left_y_value),
                Direction::Outgoing,
                Node::Lens(lens),
            ) => {
                let (y, new_state) =
                    apply_x(lens, &**left_y_value, state_map, right_id)?;

                Ok((ResolvedNode::XY(left_y_value.clone(), y), new_state))
            }
            (
                ResolvedNode::XY(left_x_value, _),
                Direction::Incoming,
                Node::Lens(lens),
            ) => {
                let (x, new_state) =
                    apply_y(lens, &**left_x_value, state_map, right_id)?;

                Ok((ResolvedNode::XY(x, left_x_value.clone()), new_state))
            }
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

fn apply_x(
    lens: &Lens,
    x: &Value,
    state_map: &HashMap<usize, Arc<Value>>,
    id: usize,
) -> Result<(Arc<Value>, Option<Arc<Value>>)> {
    let xsave = if lens.has_xsave() {
        Some(&**state_map.get(&id).unwrap())
    } else {
        None
    };
    let (y, new_state) = lens.apply_x(x, xsave)?;

    Ok((
        Arc::new(cuelang_value_to_json_value(&y)?),
        new_state
            .map(|save| cuelang_value_to_json_value(&save).map(Arc::new))
            .transpose()?,
    ))
}

fn apply_y(
    lens: &Lens,
    y: &Value,
    state_map: &HashMap<usize, Arc<Value>>,
    id: usize,
) -> Result<(Arc<Value>, Option<Arc<Value>>)> {
    let ysave = if lens.has_ysave() {
        Some(&**state_map.get(&id).unwrap())
    } else {
        None
    };
    let (x, new_state) = lens.apply_y(y, ysave)?;

    Ok((
        Arc::new(cuelang_value_to_json_value(&x)?),
        new_state
            .map(|save| cuelang_value_to_json_value(&save).map(Arc::new))
            .transpose()?,
    ))
}

fn cuelang_value_to_json_value(
    cuelang_value: &cuelang::Value,
) -> Result<Value> {
    cuelang_value
        .as_json_value()
        .ok_or(Error::ValueConversionError)
}
