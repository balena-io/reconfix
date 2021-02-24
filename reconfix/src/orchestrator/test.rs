use super::{Orchestrator, ResolvedNode};
use crate::{external_data::Synchronizer, ExternalData, Lens};
use async_trait::async_trait;
use lazy_static::lazy_static;
use petgraph::Direction;
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};

lazy_static! {
    static ref LENS_A: Lens = Lens::new(
        r#"
        X: Y + 1
        Y: X - 1
        "#,
    )
    .unwrap();
    static ref LENS_B: Lens = Lens::new(
        r#"
        X: Y + 2
        Y: X - 2
        "#,
    )
    .unwrap();
}

struct DummyExternalData;

#[async_trait]
impl ExternalData for DummyExternalData {
    async fn listen(&self, _: Synchronizer) -> anyhow::Result<Arc<Value>> {
        Ok(Arc::new(Value::Null))
    }

    async fn commit(&self, _: &Arc<Value>) -> anyhow::Result<()> {
        Ok(())
    }
}

#[test]
fn resolve_external_data_to_external_data() {
    let mut orchestrator = Orchestrator::new();
    let a = orchestrator.add_node(DummyExternalData);
    let b = orchestrator.add_node(DummyExternalData);
    orchestrator.add_edge(a, b);
    let a_value = Value::from(0);
    let final_value = a_value.clone();

    let resolved = orchestrator
        .resolve(
            &ResolvedNode::ExternalData(Arc::new(a_value)),
            Direction::Outgoing,
            b.0,
            &HashMap::new(),
        )
        .unwrap();

    assert_eq!(
        resolved.0,
        ResolvedNode::ExternalData(Arc::new(final_value.clone()))
    );
    assert_eq!(resolved.1, Some(Arc::new(final_value)));
}

#[test]
fn resolve_external_data_from_external_data() {
    let mut orchestrator = Orchestrator::new();
    let a = orchestrator.add_node(DummyExternalData);
    let b = orchestrator.add_node(DummyExternalData);
    orchestrator.add_edge(a, b);
    let a_value = Value::from(0);
    let final_value = a_value.clone();

    let resolved = orchestrator
        .resolve(
            &ResolvedNode::ExternalData(Arc::new(a_value)),
            Direction::Incoming,
            b.0,
            &HashMap::new(),
        )
        .unwrap();

    assert_eq!(
        resolved.0,
        ResolvedNode::ExternalData(Arc::new(final_value.clone()))
    );
    assert_eq!(resolved.1, Some(Arc::new(final_value)));
}

#[test]
fn resolve_external_data_to_lens() {
    let mut orchestrator = Orchestrator::new();
    let a = orchestrator.add_node(DummyExternalData);
    let b = orchestrator.add_node(LENS_A.clone());
    orchestrator.add_edge(a, b);
    let a_value = Value::from(1);
    let b_x_value = a_value.clone();
    let b_y_value = Value::from(0);

    let resolved = orchestrator
        .resolve(
            &ResolvedNode::ExternalData(Arc::new(a_value)),
            Direction::Outgoing,
            b.0,
            &HashMap::new(),
        )
        .unwrap();

    assert_eq!(
        resolved.0,
        ResolvedNode::XY(Arc::new(b_x_value), Arc::new(b_y_value))
    );
    assert!(resolved.1.is_none());
}

#[test]
fn resolve_external_data_from_lens() {
    let mut orchestrator = Orchestrator::new();
    let a = orchestrator.add_node(DummyExternalData);
    let b = orchestrator.add_node(LENS_A.clone());
    orchestrator.add_edge(a, b);
    let a_value = Value::from(1);
    let b_x_value = Value::from(2);
    let b_y_value = a_value.clone();

    let resolved = orchestrator
        .resolve(
            &ResolvedNode::ExternalData(Arc::new(a_value)),
            Direction::Incoming,
            b.0,
            &HashMap::new(),
        )
        .unwrap();

    assert_eq!(
        resolved.0,
        ResolvedNode::XY(Arc::new(b_x_value), Arc::new(b_y_value))
    );
    assert!(resolved.1.is_none());
}

#[test]
fn resolve_lens_to_external_data() {
    let mut orchestrator = Orchestrator::new();
    let a = orchestrator.add_node(LENS_A.clone());
    let b = orchestrator.add_node(DummyExternalData);
    orchestrator.add_edge(a, b);
    let a_x_value = Value::from(1);
    let a_y_value = Value::from(0);
    let final_value = a_y_value.clone();

    let resolved = orchestrator
        .resolve(
            &ResolvedNode::XY(Arc::new(a_x_value), Arc::new(a_y_value)),
            Direction::Outgoing,
            b.0,
            &HashMap::new(),
        )
        .unwrap();

    assert_eq!(
        resolved.0,
        ResolvedNode::ExternalData(Arc::new(final_value.clone()))
    );
    assert_eq!(resolved.1, Some(Arc::new(final_value)));
}

#[test]
fn resolve_lens_from_external_data() {
    let mut orchestrator = Orchestrator::new();
    let a = orchestrator.add_node(LENS_A.clone());
    let b = orchestrator.add_node(DummyExternalData);
    orchestrator.add_edge(a, b);
    let a_x_value = Value::from(1);
    let a_y_value = Value::from(0);
    let final_value = a_x_value.clone();

    let resolved = orchestrator
        .resolve(
            &ResolvedNode::XY(Arc::new(a_x_value), Arc::new(a_y_value)),
            Direction::Incoming,
            b.0,
            &HashMap::new(),
        )
        .unwrap();

    assert_eq!(
        resolved.0,
        ResolvedNode::ExternalData(Arc::new(final_value.clone()))
    );
    assert_eq!(resolved.1, Some(Arc::new(final_value)));
}

#[test]
fn resolve_lens_to_lens() {
    let mut orchestrator = Orchestrator::new();
    let a = orchestrator.add_node(LENS_A.clone());
    let b = orchestrator.add_node(LENS_B.clone());
    orchestrator.add_edge(a, b);
    let a_x_value = Value::from(3);
    let a_y_value = Arc::new(Value::from(2));

    let resolved = orchestrator
        .resolve(
            &ResolvedNode::XY(Arc::new(a_x_value), a_y_value.clone()),
            Direction::Outgoing,
            b.0,
            &HashMap::new(),
        )
        .unwrap();

    assert_eq!(resolved.0, ResolvedNode::XY(a_y_value, Arc::new(0.into())));
    assert!(resolved.1.is_none());
}

#[test]
fn resolve_lens_from_lens() {
    let mut orchestrator = Orchestrator::new();
    let a = orchestrator.add_node(LENS_A.clone());
    let b = orchestrator.add_node(LENS_B.clone());
    orchestrator.add_edge(a, b);
    let a_x_value = Arc::new(Value::from(2));
    let a_y_value = Value::from(1);

    let resolved = orchestrator
        .resolve(
            &ResolvedNode::XY(a_x_value.clone(), Arc::new(a_y_value)),
            Direction::Incoming,
            b.0,
            &HashMap::new(),
        )
        .unwrap();

    assert_eq!(resolved.0, ResolvedNode::XY(Arc::new(4.into()), a_x_value));
    assert!(resolved.1.is_none());
}
