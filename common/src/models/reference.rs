use neo4rs::{Graph, Query};
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct Reference {
    pub id: String,

    pub internal: bool,

    pub trust: i64,

    pub distrust: i64,
}

impl Reference {
    pub fn new(id: String, internal: bool, trust: i64, distrust: i64) -> Self {
        Self {
            id,
            internal,
            trust,
            distrust,
        }
    }

    pub async fn create(&self, graph: Graph) {
        let q = Query::new("CREATE (r:Reference {id: $id, internal: $internal, trust: $trust, distrust: $distrust})".to_string())
            .param("id", Uuid::new_v4().to_string())
            .param("internal", self.internal.to_string())
            .param("trust", self.trust)
            .param("distrust", self.distrust);

        let tx = graph.start_txn().await.unwrap();

        if let Err(e) = tx.run(q).await {
            println!("Error: {:?}", e);
        }

        if let Err(e) = tx.commit().await {
            println!("Error: {:?}", e);
        }
    }

    pub async fn get_reference(&self, graph: Graph) {
        let q = Query::new("MATCH (r:Reference {id: $id})".to_string())
            .param("id", Uuid::new_v4().to_string());

        let tx = graph.start_txn().await.unwrap();

        if let Err(e) = tx.run(q).await {
            println!("Error: {:?}", e);
        }

        if let Err(e) = tx.commit().await {
            println!("Error: {:?}", e);
        }
    }

    pub async fn update_reference(&self, graph: Graph) {
        let q = Query::new("MATCH (r:Reference {id: $id})".to_string())
            .param("id", Uuid::new_v4().to_string());

        let tx = graph.start_txn().await.unwrap();

        if let Err(e) = tx.run(q).await {
            println!("Error: {:?}", e);
        }

        if let Err(e) = tx.commit().await {
            println!("Error: {:?}", e);
        }
    }

    pub async fn delete_reference(&self, graph: Graph) {
        let q = Query::new("MATCH (r:Reference {id: $id})".to_string())
            .param("id", Uuid::new_v4().to_string());

        let tx = graph.start_txn().await.unwrap();

        if let Err(e) = tx.run(q).await {
            println!("Error: {:?}", e);
        }

        if let Err(e) = tx.commit().await {
            println!("Error: {:?}", e);
        }
    }
}
