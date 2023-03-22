use std::sync::Arc;

use neo4rs::{Graph, Query};
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct Reference {
    pub id: String,

    pub internal: bool,

    pub trust: i64,

    pub distrust: i64,

    pub content: String,
}

impl Reference {
    pub fn new(id: String, internal: bool, trust: i64, distrust: i64, content: String) -> Self {
        Self {
            id,
            internal,
            trust,
            distrust,
            content,
        }
    }

    pub async fn create(&self, graph: &Graph) -> String {
        let q = Query::new("CREATE (r:Reference {id: $id, internal: $internal, trust: $trust, distrust: $distrust, content: $content}) RETURN(r.id)".to_string())
            .param("id", Uuid::new_v4().to_string())
            .param("internal", self.internal.to_string())
            .param("trust", self.trust)
            .param("distrust", self.distrust)
            .param("content", self.content.clone());

        let tx = graph.start_txn().await.unwrap();

        let id = match tx.execute(q).await {
            Ok(mut res) => {
                let row = res.next().await.unwrap().unwrap();

                let id: String = row.get("(r.id)").unwrap();

                Some(id)
            }

            Err(e) => {
                println!("Error: {:#?}", e);

                None
            }
        }
        .unwrap();

        if let Err(e) = tx.commit().await {
            println!("Error: {:#?}", e);
        };

        id
    }

    pub async fn get_reference(&self, graph: &Graph) {
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

    pub async fn update_reference(&self, graph: &Graph) {
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

    pub async fn delete_reference(&self, graph: &Graph) {
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
