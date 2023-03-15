use std::sync::Arc;

use super::user::User;
use neo4rs::{Graph, Query};
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
pub struct Debate {
    pub id: String,

    pub score: i64,

    pub topic: String,

    pub category: String,

    pub registered_speakers: i64,

    pub commenters: i64,

    pub voters: i64,

    pub comments: i64,

    pub inactive_participants: i64,

    pub responses: i64,
}

impl Debate {
    pub fn new(
        id: String,
        score: i64,
        topic: String,
        category: String,
        registered_speakers: i64,
        commenters: i64,
        voters: i64,
        comments: i64,
        inactive_participants: i64,
        responses: i64,
    ) -> Self {
        Self {
            id,
            score,
            topic,
            category,
            registered_speakers,
            commenters,
            voters,
            comments,
            inactive_participants,
            responses,
        }
    }

    pub async fn create(&self, graph: Arc<Graph>) {
        let q = Query::new("CREATE(d: Debate {id: $id, score: $score, topic: $topic, category: $category, registered_speakers: $registered_speakers, commenters: $commenters, voters: $voters, inactive_participants: $inactive_participants, comments: $comments, responses: $responses})".to_string())
            .param("id", Uuid::new_v4().to_string())
            .param("score", self.score.to_string())
            .param("topic", self.topic.to_string())
            .param("category", self.category.to_string())
            .param("registered_speakers", self.registered_speakers.to_string())
            .param("commenters", self.commenters.to_string())
            .param("voters", self.voters.to_string())
            .param("comments", self.comments.to_string())
            .param("inactive_participants", self.inactive_participants.to_string())
            .param("responses", self.responses.to_string());

        let tx = graph.start_txn().await.unwrap();

        if let Err(e) = tx.run(q).await {
            println!("Error: {:#?}", e);
        }

        if let Err(e) = tx.commit().await {
            println!("Error: {:#?}", e);
        }
    }

    pub async fn get_debate(&self, graph: Arc<Graph>) {
        let q = Query::new("MATCH (d:Debate {id: $id})".to_string())
            .param("id", Uuid::new_v4().to_string());

        let tx = graph.start_txn().await.unwrap();

        if let Err(e) = tx.run(q).await {
            println!("Error: {:#?}", e);
        }

        if let Err(e) = tx.commit().await {
            println!("Error: {:#?}", e);
        }
    }

    pub async fn update_debate(&self, graph: Arc<Graph>) {
        let q = Query::new("MATCH (d:Debate {id: $id} SET d.score = $score, d.topic = $topic, d.category = $category, d.registered_speakers = $registered_speakers, d.commenters = $commenters, d.voters = $voters, d.inactive_participants = $inactive_participants, d.comments = $comments, d.responses = $responses})".to_string())
            .param("id", Uuid::new_v4().to_string())
            .param("score", self.score.to_string())
            .param("topic", self.topic.to_string())
            .param("category", self.category.to_string())
            .param("registered_speakers", self.registered_speakers.to_string())
            .param("commenters", self.commenters.to_string())
            .param("voters", self.voters.to_string())
            .param("comments", self.comments.to_string())
            .param("inactive_participants", self.inactive_participants.to_string())
            .param("responses", self.responses.to_string());

        let tx = graph.start_txn().await.unwrap();

        if let Err(e) = tx.run(q).await {
            println!("Error: {:#?}", e);
        }

        if let Err(e) = tx.commit().await {
            println!("Error: {:#?}", e);
        }
    }

    pub async fn delete_debate(&self, graph: Graph) {
        let q = Query::new("MATCH (d:Debate {id: $id}) DETACH DELETE d".to_string())
            .param("id", Uuid::new_v4().to_string());

        let tx = graph.start_txn().await.unwrap();

        if let Err(e) = tx.run(q).await {
            println!("Error: {:#?}", e);
        }

        if let Err(e) = tx.commit().await {
            println!("Error: {:#?}", e);
        }
    }

    pub async fn add_participant(&self, graph: Arc<Graph>, user: User) {
        let q = Query::new("MATCH (d:Debate {id: $id}) MATCH (u:User {id: $user_id}) CREATE (u)-[:PARTICIPATED]->(d)".to_string())
            .param("id", Uuid::new_v4().to_string())
            .param("user_id", user.id.to_string());

        let tx = graph.start_txn().await.unwrap();

        if let Err(e) = tx.run(q).await {
            println!("Error: {:#?}", e);
        }

        if let Err(e) = tx.commit().await {
            println!("Error: {:#?}", e);
        }
    }
}
