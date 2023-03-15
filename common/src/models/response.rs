use std::sync::Arc;

use super::reference::Reference;
use super::user::User;
use neo4rs::{Graph, Query};
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
pub struct Response {
    pub id: String,

    pub content: String,

    pub confidence: f64,

    pub score: i64,

    pub valid_vote_count: i64,

    pub invalid_vote_count: i64,

    pub abstain_vote_count: i64,

    pub author_id: String,

    pub replies: Vec<Response>,
}

impl Response {
    pub fn new(
        id: String,
        content: String,
        confidence: f64,
        score: i64,
        valid_vote_count: i64,
        invalid_vote_count: i64,
        abstain_vote_count: i64,
        author_id: String,
        replies: Vec<Response>,
    ) -> Self {
        Self {
            id,
            content,
            confidence,
            score,
            valid_vote_count,
            invalid_vote_count,
            abstain_vote_count,
            author_id,
            replies,
        }
    }

    pub fn generate_engagement(&self, response: &Self) {}

    pub async fn create(&self, graph: Arc<Graph>) {
        let q = Query::new("CREATE (r:Response {id: $id, content: $content})".to_string())
            .param("id", Uuid::new_v4().to_string())
            .param("content", self.content.to_string());

        let tx = graph.start_txn().await.unwrap();

        if let Err(e) = tx.run(q).await {
            println!("Error: {:#?}", e);
        }

        if let Err(e) = tx.commit().await {
            println!("Error: {:#?}", e);
        }
    }

    pub async fn get_response(&self, graph: Graph) {
        let q = Query::new("MATCH (r:Response {id: $id})".to_string()).param("id", self.id.clone());

        let tx = graph.start_txn().await.unwrap();

        if let Err(e) = tx.run(q).await {
            println!("Error: {:#?}", e);
        }

        if let Err(e) = tx.commit().await {
            println!("Error: {:#?}", e);
        }
    }

    pub async fn update_response(&self, graph: Graph) {
        let q = Query::new("MATCH (r:Response {id: $id} SET r.content = $content, r.confidence = $confidence, r.score = $score, r.valid_vote_count = $vvc, r.invalid_vote_count = $ivc, r.abstain_vote_count = $avc, r.author_id = $author_id)".to_string())
            .param("id", self.id.clone())
            .param("content", self.content.clone())
            .param("confidence", self.confidence.to_string())
            .param("score", self.score)
            .param("vvc", self.valid_vote_count)
            .param("ivc", self.invalid_vote_count)
            .param("avc", self.abstain_vote_count)
            .param("author_id", self.author_id.clone());

        let tx = graph.start_txn().await.unwrap();

        if let Err(e) = tx.run(q).await {
            println!("Error: {:#?}", e);
        }

        if let Err(e) = tx.commit().await {
            println!("Error: {:#?}", e);
        }
    }

    pub async fn delete_response(&self, graph: Graph) {
        let q = Query::new("MATCH (r:Response {id: $id}) DETACH DELETE r".to_string())
            .param("id", self.id.clone());

        let tx = graph.start_txn().await.unwrap();

        if let Err(e) = tx.run(q).await {
            println!("Error: {:#?}", e);
        }

        if let Err(e) = tx.commit().await {
            println!("Error: {:#?}", e);
        }
    }

    pub async fn add_user_responded(&self, graph: Arc<Graph>, user: User) {
        let q = Query::new("MATCH (r:Response {id: $id}) MATCH (u:User {id: $user_id}) CREATE (u)-[:RESPONDED]->(r)".to_string())
            .param("id", self.id.clone())
            .param("user_id", user.id.clone());

        let tx = graph.start_txn().await.unwrap();

        if let Err(e) = tx.run(q).await {
            println!("Error: {:#?}", e);
        }

        if let Err(e) = tx.commit().await {
            println!("Error: {:#?}", e);
        }
    }

    pub async fn add_has_referecne(&self, graph: Graph, reference: Reference) {
        let q = Query::new("MATCH (r:Response {id: $id}) MATCH (ref:Reference {id: $reference_id}) CREATE (r)-[:HAS_REFERENCE]->(ref)".to_string())
            .param("id", self.id.clone())
            .param("reference_id", reference.id.clone());

        let tx = graph.start_txn().await.unwrap();

        if let Err(e) = tx.run(q).await {
            println!("Error: {:#?}", e);
        }

        if let Err(e) = tx.commit().await {
            println!("Error: {:#?}", e);
        }
    }
}
