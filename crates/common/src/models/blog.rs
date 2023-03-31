use super::user::User;
use neo4rs::{Graph, Query};
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
pub struct Blog {
    pub id: String,

    pub title: String,

    pub body: String,

    pub author: String,

    pub topic: String,

    pub category: String,

    pub publish_timestamp: i64,

    pub ethos: i64,

    pub pathos: i64,

    pub logos: i64,

    pub score: i64,

    pub engagements: i64,

    pub impressions: i64,
}

impl Business {
    pub fn new(
        id: String,
        title: String,
        body: String,
        author: String,
        topic: String,
        category: String,
        publish_timestamp: i64,
        ethos: i64,
        pathos: i64,
        logos: i64,
        score: i64,
        engagements: i64,
        impressions: i64,
    ) -> Self {
        Self {
            id,
            title,
            body,
            author,
            topic,
            category,
            publish_timestamp,
            ethos,
            pathos,
            logos,
            score,
            engagements,
            impressions,
        }
    }

    pub async fn create_blog(&self, graph: &Graph) -> String {
        let q = Query::new(
            "CREATE (b:Blog {id: $id, title: $title, body: $body, author: $author, topic: $topic, category: $category, publish_timestamp: $publish_timestamp, ethos: $ethos, pathos: $pathos, logos: $logos, score: $score, engagements: $engagements, impressions: $impressions}) RETURN(b.id)"
                .to_string(),
        )
        .param("id", Uuid::new_v4().to_string())
        .param("title", self.title.clone())
        .param("body", self.body.clone())
        .param("author", self.author.clone())
        .param("topic", self.topic.clone())
        .param("category", self.category.clone())
        .param("publish_timestamp", self.publish_timestamp)
        .param("ethos", self.ethos)
        .param("pathos", self.pathos)
        .param("logos", self.logos)
        .param("score", self.score)
        .param("engagements", self.engagements)
        .param("impressions", self.impressions);

        let tx = graph.start_txn().await.unwrap();

        let id = match tx.execute(q).await {
            Ok(mut res) => {
                let row = res.next().await.unwrap().unwrap();

                let id = row.get("(b.id)");

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
        }
    }

    pub async fn get_blog(&self, graph: &Graph) -> Self {
        let q = Query::new(
            "MATCH (b:Blog {id: $id}) RETURN b.id, b.title, b.body, b.author, b.topic, b.category, b.publish_timestamp, b.ethos, b.pathos, b.logos, b.score, b.engagements, b.impressions"
                .to_string(),
        )
        .param("id", self.id.clone());

        let tx = graph.start_txn().await.unwrap();

        let blog = match tx.execute(q).await {
            Ok(mut res) => {
                let row = res.next().await.unwrap().unwrap();

                let id = row.get("b.id");
                let title = row.get("b.title");
                let body = row.get("b.body");
                let author = row.get("b.author");
                let topic = row.get("b.topic");
                let category = row.get("b.category");
                let publish_timestamp = row.get("b.publish_timestamp");
                let ethos = row.get("b.ethos");
                let pathos = row.get("b.pathos");
                let logos = row.get("b.logos");
                let score = row.get("b.score");
                let engagements = row.get("b.engagements");
                let impressions = row.get("b.impressions");

                Self::new(
                    id,
                    title,
                    body,
                    author,
                    topic,
                    category,
                    publish_timestamp,
                    ethos,
                    pathos,
                    logos,
                    score,
                    engagements,
                    impressions,
                )
            }

            Err(e) => {
                println!("Error: {:#?}", e);

                None
            }
        }
        .unwrap();

        if let Err(e) = tx.commit().await {
            println!("Error: {:#?}", e);
        }
    }

    pub async fn update_blog(&self, graph: &Graph) {
        let q = Query::new(
            "MATCH (b:Blog {id: $id}) SET b.title = $title, b.body = $body, b.author = $author, b.topic = $topic, b.category = $category, b.publish_timestamp = $publish_timestamp, b.ethos = $ethos, b.pathos = $pathos, b.logos = $logos, b.score = $score, b.engagements = $engagements, b.impressions = $impressions"
                .to_string(),
        )
        .param("id", self.id.clone())
        .param("title", self.title.clone())
        .param("body", self.body.clone())
        .param("author", self.author.clone())
        .param("topic", self.topic.clone())
        .param("category", self.category.clone())
        .param("publish_timestamp", self.publish_timestamp)
        .param("ethos", self.ethos)
        .param("pathos", self.pathos)
        .param("logos", self.logos)
        .param("score", self.score)
        .param("engagements", self.engagements)
        .param("impressions", self.impressions);

        let tx = graph.start_txn().await.unwrap();

        if let Err(e) = tx.execute(q).await {
            println!("Error: {:#?}", e);
        }
    }

    pub async fn delete_blog(&self, graph: &Graph) {
        let q = Query::new("MATCH (b:Blog {id: $id}) DETACH DELETE b".to_string())
            .param("id", self.id.clone());

        let tx = graph.start_txn().await.unwrap();

        if let Err(e) = tx.execute(q).await {
            println!("Error: {:#?}", e);
        }
    }
}
