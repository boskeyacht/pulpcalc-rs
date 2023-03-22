use super::super::PersonasUser;
use neo4rs::{Graph, Query};
use pulpcalc_common::models::{Debate, Response};
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct Learned {
    pub id: String,

    pub learned_content: String,

    pub reason: String,
}

impl Learned {
    pub fn new(id: String, learned_content: String, reason: String) -> Self {
        Self {
            id,
            learned_content,
            reason,
        }
    }

    /// Super hacky here :(
    pub async fn create(&self, graph: &Graph) -> String {
        let id = Uuid::new_v4().to_string();
        let q = Query::new(
            "CREATE (l:Learned {id: $id, learned_content: $learned_content, reason: $reason}) RETURN(l.id)"
                .to_string(),
        )
        .param("id", id.clone())
        .param("learned_content", self.learned_content.to_string())
        .param("reason", self.reason.to_string());

        let tx = graph.start_txn().await.unwrap();

        let id = match tx.execute(q).await {
            Ok(mut res) => {
                let row = res.next().await.unwrap().unwrap();

                let id = row.get("(l.id)");

                id
            }

            Err(e) => {
                println!("Error: {:#?}", e);

                Some(id)
            }
        }
        .unwrap();

        if let Err(e) = tx.commit().await {
            println!("Error: {:#?}", e);
        };

        id
    }

    pub async fn get_learned(&self, graph: &Graph) -> Self {
        let q = Query::new("MATCH (l:Learned {id: $id}) RETURN (l)".to_string())
            .param("id", self.id.clone());

        let tx = graph.start_txn().await.unwrap();

        let user = match tx.execute(q).await {
            Ok(mut res) => {
                let row = res.next().await.unwrap().unwrap();

                let mut l = Learned::default();

                l.id = row.get("(l.id)").unwrap_or_default();
                l.learned_content = row.get("(l.learned_content)").unwrap_or_default();
                l.reason = row.get("(l.reason)").unwrap_or_default();

                Some(l)
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

        user
    }

    pub async fn update_learned(&self, graph: Graph) {
        let q = Query::new(
            "MATCH (l:Learned {id: $id}) SET l.learned_content = $learned_content, l.reason = $reason RETURN (l.id)"
                .to_string(),
        )
        .param("id", self.id.clone())
        .param("learned_content", self.learned_content.clone())
        .param("reason", self.reason.clone());

        let tx = graph.start_txn().await.unwrap();

        if let Err(e) = tx.execute(q).await {
            println!("Error: {:#?}", e);
        }

        if let Err(e) = tx.commit().await {
            println!("Error: {:#?}", e);
        };
    }

    pub async fn delete_learned(&self, graph: Graph) {
        let q = Query::new("MATCH (l:Learned {id: $id}) DETACH DELETE l".to_string())
            .param("id", self.id.clone());

        let tx = graph.start_txn().await.unwrap();

        if let Err(e) = tx.execute(q).await {
            println!("Error: {:#?}", e);
        }

        if let Err(e) = tx.commit().await {
            println!("Error: {:#?}", e);
        };
    }

    pub async fn add_user_learned(&self, graph: &Graph, user: PersonasUser) {
        let q = Query::new(
            "MATCH (l:Learned {id: $id}), (pu:PersonaUser {id: $user_id}) CREATE (pu)-[:LEARNED]->(l)"
                .to_string(),
        )
        .param("id", self.id.clone())
        .param("user_id", user.base_user.id);

        let tx = graph.start_txn().await.unwrap();

        if let Err(e) = tx.run(q).await {
            println!("Error: {:#?}", e);
        }

        if let Err(e) = tx.commit().await {
            println!("Error: {:#?}", e);
        }
    }

    pub async fn add_learned_in(&self, graph: &Graph, debate: Debate) {
        let q = Query::new(
            "MATCH (l:Learned {id: $id}), (d:Debate {id: $debate_id}) CREATE (l)-[:LEARNED_IN]->(d)"
                .to_string(),
        )
        .param("id", self.id.clone())
        .param("debate_id", debate.id);

        let tx = graph.start_txn().await.unwrap();

        if let Err(e) = tx.run(q).await {
            println!("Error: {:#?}", e);
        }

        if let Err(e) = tx.commit().await {
            println!("Error: {:#?}", e);
        }
    }

    pub async fn add_learned_from(&self, graph: &Graph, response: Response) {
        let q = Query::new(
            "MATCH (l:Learned {id: $id}), (r:Response {id: $response_id}) CREATE (l)-[:LEARNED_FROM]->(r)"
                .to_string(),
        )
        .param("id", self.id.clone())
        .param("response_id", response.id);

        let tx = graph.start_txn().await.unwrap();

        if let Err(e) = tx.run(q).await {
            println!("Error: {:#?}", e);
        }

        if let Err(e) = tx.commit().await {
            println!("Error: {:#?}", e);
        }
    }
}
