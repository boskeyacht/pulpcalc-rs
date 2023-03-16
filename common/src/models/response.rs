use std::sync::Arc;

use super::engagements::Engagements;
use super::reference::Reference;
use super::user::User;
use super::{attributes::Attributes, Debate};
use crate::models::gpt_scoring::*;
use neo4rs::{Graph, Query};
use pulpcalc_external::chatgpt::ChatRequestBuilder;
use reqwest::Client;
use serde_json::from_str;
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

    pub report_count: i64,

    pub hide_count: i64,

    pub author_id: String,

    pub topic_of_response: String,

    pub replies: Vec<Response>,

    pub attributes: Attributes,

    pub engagements: Engagements,
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
        report_count: i64,
        hide_count: i64,
        author_id: String,
        topic_of_response: String,
        replies: Vec<Response>,
        attributes: Attributes,
        engagements: Engagements,
    ) -> Self {
        Self {
            id,
            content,
            confidence,
            score,
            valid_vote_count,
            invalid_vote_count,
            abstain_vote_count,
            report_count,
            hide_count,
            author_id,
            topic_of_response,
            replies,
            attributes,
            engagements,
        }
    }

    // State variables for readbility

    pub fn calculate_engagement_score(&mut self) -> i64 {
        // let mut report_harmful_to_others = 0;
        // let mut report_abuse_of_platform = 0;
        // let mut hide = 0;
        // let mut vote_validity = 0;
        // let mut vote_confidence = 0;
        // let mut response_distance = 0;
        // let response_timing = 0;

        self.engagements = Engagements::new(0, 0, 0, 0, 0, 0, 0);

        0
    }

    pub async fn calculate_content_attribute_score(&mut self, open_ai_key: String) -> i64 {
        let mut init_score: i64 = self.score;
        let mut relevance = 0.0;
        let mut soundness = 0.0;
        // let mut stats_included = 0;
        let mut references = 0;
        // let mut syntax_and_grammar = 0;
        // let mut spelling = 0;
        let mut word_count: i64 = 0;
        let mut mastery_vocabulary: i64 = 0;

        let mut relevance_prompt = RELEVANCE_PROMPT.to_string();
        relevance_prompt =
            relevance_prompt.replace("THIS_TOPIC", &self.topic_of_response.to_string());
        relevance_prompt = relevance_prompt.replace("THIS_CONTENT", &self.content.to_string());

        let relevance_chat_res = ChatRequestBuilder::new()
            .messages(relevance_prompt)
            .temperature(0.7)
            .max_tokens(850)
            .top_p(1.0)
            .presence_penalty(0.0)
            .frequency_penalty(0.0)
            .build()
            .send(open_ai_key.clone(), Client::new())
            .await;

        let relevance_res =
            from_str::<RelevanceResponse>(&relevance_chat_res.choices[0].message.content.clone());

        let rel = match relevance_res {
            Ok(res) => Some(res),

            Err(e) => {
                println!("failed to unmarshal content: {:?}", e);

                None
            }
        };

        relevance = rel.unwrap().relevance;

        println!("relevance: {:?}", relevance);

        let mut soundness_prompt = SOUNDNESS_PROMPT.to_string();
        soundness_prompt =
            soundness_prompt.replace("THIS_TOPIC", &self.topic_of_response.to_string());
        soundness_prompt = soundness_prompt.replace("THIS_CONTENT", &self.content.to_string());

        let soundness_chat_res = ChatRequestBuilder::new()
            .messages(soundness_prompt)
            .temperature(0.7)
            .max_tokens(850)
            .top_p(1.0)
            .presence_penalty(0.0)
            .frequency_penalty(0.0)
            .build()
            .send(open_ai_key.clone(), Client::new())
            .await;

        let soundness_res =
            from_str::<SoundnessResponse>(&soundness_chat_res.choices[0].message.content.clone());

        let snd = match soundness_res {
            Ok(res) => Some(res),

            Err(e) => {
                println!("failed to unmarshal content: {:?}", e);

                None
            }
        };

        soundness = snd.unwrap().soundness;

        println!("soundness: {:?}", soundness);

        let mut mastery_prompt = MASTERY_VOCAB_PROMPT.to_string();
        mastery_prompt = mastery_prompt.replace("THIS_TOPIC", &self.topic_of_response.to_string());
        mastery_prompt = mastery_prompt.replace("THIS_CONTENT", &self.content.to_string());

        let mastery_chat_res = ChatRequestBuilder::new()
            .messages(mastery_prompt)
            .temperature(0.7)
            .max_tokens(850)
            .top_p(1.0)
            .presence_penalty(0.0)
            .frequency_penalty(0.0)
            .build()
            .send(open_ai_key, Client::new())
            .await;

        let mastery_res =
            from_str::<MasteryVocabResponse>(&mastery_chat_res.choices[0].message.content.clone());

        let mst = match mastery_res {
            Ok(res) => Some(res),

            Err(e) => {
                println!("failed to unmarshal content: {:?}", e);

                None
            }
        };

        if let Some(mast) = mst {
            let mastery_vocabulary = if let Some(wrds) = mast.mastery_words.clone() {
                wrds.len() as i64
            } else {
                0
            };

            word_count = self.content.split_whitespace().count() as i64;

            init_score += (mastery_vocabulary + word_count) * (relevance + soundness) as i64;

            self.attributes = Attributes::new(
                relevance,
                soundness,
                references,
                word_count,
                mast.mastery_words,
            );
        } else {
        }

        println!("init_score: {:#?}", init_score);

        init_score
    }

    pub fn calculate_reply_score(&self) -> i64 {
        0
    }

    // ----------------------------------
    //           Neo4j Methods          -
    // ----------------------------------
    pub async fn create(&self, graph: Arc<Graph>) -> String {
        let q = Query::new(
            "CREATE (r:Response {id: $id, content: $content}) RETURN (r.id)".to_string(),
        )
        .param("id", Uuid::new_v4().to_string())
        .param("content", self.content.to_string());

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

    pub async fn get_response(&self, graph: Graph) -> Self {
        let q = Query::new("MATCH (r:Response {id: $id})".to_string()).param("id", self.id.clone());

        let tx = graph.start_txn().await.unwrap();

        let response = match tx.execute(q).await {
            Ok(mut res) => {
                let row = res.next().await.unwrap().unwrap();

                let mut r = Response::default();

                r.content = row.get("r.content").unwrap();

                Some(r)
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

        response
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

    pub async fn add_reply_relationship(&self, graph: Arc<Graph>, reply: Self) {
        let q = Query::new("MATCH (r:Response {id: $id}) MATCH (rep:Response {id: $reply_id}) CREATE (r)-[:REPLIED]->(rep)".to_string())
            .param("id", self.id.clone())
            .param("reply_id", reply.id.clone());

        let tx = graph.start_txn().await.unwrap();

        if let Err(e) = tx.run(q).await {
            println!("Error: {:#?}", e);
        }

        if let Err(e) = tx.commit().await {
            println!("Error: {:#?}", e);
        }
    }

    pub async fn add_debate_response_relationship(&self, graph: Arc<Graph>, debate: Debate) {
        let q = Query::new("MATCH (r:Response {id: $id}) MATCH (d:Debate {id: $debate_id}) CREATE (r)-[:RESPONSE]->(d)".to_string())
            .param("id", self.id.clone())
            .param("debate_id", debate.id.clone());

        let tx = graph.start_txn().await.unwrap();

        if let Err(e) = tx.run(q).await {
            println!("Error: {:#?}", e);
        }

        if let Err(e) = tx.commit().await {
            println!("Error: {:#?}", e);
        }
    }
}
