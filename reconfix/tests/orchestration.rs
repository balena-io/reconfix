use reconfix::{external_data::InMemoryExternalData, Lens, Orchestrator};
use std::future::Future;
use tokio::{
    task,
    time::{self, Duration},
};

const ONE_SECOND: Duration = Duration::from_secs(1);

#[tokio::test]
async fn running_empty_graph_should_resolve() {
    timeout(Orchestrator::new().run()).await.unwrap();
}

#[tokio::test]
async fn running_graph_without_external_data_should_resolve() {
    let mut orchestrator = Orchestrator::new();
    orchestrator.add_node(
        Lens::new(
            r#"
            X: Y
            Y: X
            "#,
        )
        .unwrap(),
    );
    timeout(orchestrator.run()).await.unwrap();
}

#[tokio::test]
async fn running_graph_should_resolve_when_all_synchronizers_drop() {
    let data = InMemoryExternalData::new(&()).unwrap();
    let mut orchestrator = Orchestrator::new();
    orchestrator.add_node(data.clone());
    let run_join = tokio::spawn(async move {
        orchestrator.run().await.unwrap();
    });

    timeout(async move {
        while !data.is_listening().await {
            task::yield_now().await;
        }

        data.unlisten().await;
        run_join.await.unwrap();
    })
    .await;
}

#[tokio::test]
async fn should_synchronize_two_external_data_nodes() {
    let data1 = InMemoryExternalData::new(&0).unwrap();
    let data2 = InMemoryExternalData::new(&0).unwrap();
    let mut orchestrator = Orchestrator::new();
    let data1_node = orchestrator.add_node(data1.clone());
    let data2_node = orchestrator.add_node(data2.clone());
    orchestrator.add_edge(data1_node, data2_node);
    tokio::spawn(async move {
        orchestrator.run().await.unwrap();
    });

    timeout(async move {
        while !data1.is_listening().await {
            task::yield_now().await;
        }
        while !data2.is_listening().await {
            task::yield_now().await;
        }

        data1.set_cloned(&1).await.unwrap();
        assert_eq!(data2.get_cloned().await.unwrap(), 1);
        data2.set_cloned(&2).await.unwrap();
        assert_eq!(data1.get_cloned().await.unwrap(), 2);
    })
    .await;
}

#[tokio::test]
async fn should_synchronize_two_external_data_nodes_and_a_lens() {
    let data1 = InMemoryExternalData::new(&1).unwrap();
    let data2 = InMemoryExternalData::new(&0).unwrap();
    let mut orchestrator = Orchestrator::new();
    let data1_node = orchestrator.add_node(data1.clone());
    let lens_node = orchestrator.add_node(
        Lens::new(
            r#"
            X: Y + 1
            Y: X - 1
            "#,
        )
        .unwrap(),
    );
    let data2_node = orchestrator.add_node(data2.clone());
    orchestrator.add_edge(data1_node, lens_node);
    orchestrator.add_edge(lens_node, data2_node);
    tokio::spawn(async move {
        orchestrator.run().await.unwrap();
    });

    timeout(async move {
        while !data1.is_listening().await {
            task::yield_now().await;
        }
        while !data2.is_listening().await {
            task::yield_now().await;
        }

        data1.set_cloned(&2).await.unwrap();
        assert_eq!(data2.get_cloned().await.unwrap(), 1);
        data2.set_cloned(&2).await.unwrap();
        assert_eq!(data1.get_cloned().await.unwrap(), 3);
    })
    .await;
}

#[tokio::test]
async fn should_synchronize_two_external_data_nodes_and_two_lenses() {
    let data1 = InMemoryExternalData::new(&3).unwrap();
    let data2 = InMemoryExternalData::new(&0).unwrap();
    let mut orchestrator = Orchestrator::new();
    let data1_node = orchestrator.add_node(data1.clone());
    let lens1_node = orchestrator.add_node(
        Lens::new(
            r#"
            X: Y + 1
            Y: X - 1
            "#,
        )
        .unwrap(),
    );
    let lens2_node = orchestrator.add_node(
        Lens::new(
            r#"
            X: Y + 2
            Y: X - 2
            "#,
        )
        .unwrap(),
    );
    let data2_node = orchestrator.add_node(data2.clone());
    orchestrator.add_edge(data1_node, lens1_node);
    orchestrator.add_edge(lens1_node, lens2_node);
    orchestrator.add_edge(lens2_node, data2_node);
    tokio::spawn(async move {
        orchestrator.run().await.unwrap();
    });

    timeout(async move {
        while !data1.is_listening().await {
            task::yield_now().await;
        }
        while !data2.is_listening().await {
            task::yield_now().await;
        }

        data1.set_cloned(&4).await.unwrap();
        assert_eq!(data2.get_cloned().await.unwrap(), 1);
        data2.set_cloned(&4).await.unwrap();
        assert_eq!(data1.get_cloned().await.unwrap(), 7);
    })
    .await;
}

async fn timeout<F, I>(future: F) -> I
where
    F: Future<Output = I>,
{
    time::timeout(ONE_SECOND, future).await.unwrap()
}
