use std::sync::Arc;

use super::vote::Vote;
use neo4rs::{Graph, Query};
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct User {
    pub id: String,

    pub debates: Vec<String>,

    pub votes: Vec<Vote>,

    pub simulation_data: String,
}

impl User {
    pub fn new(id: String, simulation_data: String) -> Self {
        Self {
            id,
            simulation_data,
            ..Default::default()
        }
    }

    pub async fn create(&self, graph: Arc<Graph>) -> String {
        let q = Query::new(
            "CREATE (u:User {id: $id,  simulation_data: $simulation_data}) RETURN(u.id)"
                .to_string(),
        )
        .param("id", Uuid::new_v4().to_string())
        .param("simulation_data", self.simulation_data.to_string());

        let tx = graph.start_txn().await.unwrap();

        let id = match tx.execute(q).await {
            Ok(mut res) => {
                let row = res.next().await.unwrap().unwrap();

                let id = row.get("(u.id)");

                id
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

    pub async fn get_user(&self, graph: Graph) {
        let q = Query::new("MATCH (u:User {id: $id})".to_string())
            .param("id", Uuid::new_v4().to_string());

        let tx = graph.start_txn().await.unwrap();

        if let Err(e) = tx.run(q).await {
            println!("Error: {:#?}", e);
        }

        if let Err(e) = tx.commit().await {
            println!("Error: {:#?}", e);
        }
    }

    pub async fn update_user(&self, graph: Graph) {
        let q = Query::new("MATCH (u:User {id: $id})".to_string())
            .param("id", Uuid::new_v4().to_string());

        let tx = graph.start_txn().await.unwrap();

        if let Err(e) = tx.run(q).await {
            println!("Error: {:#?}", e);
        }

        if let Err(e) = tx.commit().await {
            println!("Error: {:#?}", e);
        }
    }

    pub async fn delete_user(&self, graph: Graph) {
        let q = Query::new("MATCH (u:User {id: $id})".to_string())
            .param("id", Uuid::new_v4().to_string());

        let tx = graph.start_txn().await.unwrap();

        if let Err(e) = tx.run(q).await {
            println!("Error: {:#?}", e);
        }

        if let Err(e) = tx.commit().await {
            println!("Error: {:#?}", e);
        }
    }
}
