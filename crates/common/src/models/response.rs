use super::engagements::Engagements;
use super::reference::Reference;
use super::user::User;
use super::{attributes::Attributes, Debate};
use crate::errors::{PulpError, SimulationError};
use crate::llm_config::LLMRequest;
use crate::models::gpt_scoring::*;
use neo4rs::{Graph, Query};
use std::sync::Arc;
use tokio::{join, task, task::JoinError};
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

    pub topic_of_response: String,

    pub ethos: f64,

    pub pathos: f64,

    pub logos: f64,

    pub replies: Vec<String>,

    pub references: Vec<String>,

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
        topic_of_response: String,
        ethos: f64,
        pathos: f64,
        logos: f64,
        replies: Vec<String>,
        references: Vec<String>,
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
            topic_of_response,
            ethos,
            pathos,
            logos,
            replies,
            references,
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

    // TODO: if let some here
    #[allow(unused_mut)]
    #[allow(unused_assignments)]
    pub async fn calculate_content_attribute_score(
        &mut self,
        open_ai_key: Arc<String>,
    ) -> Result<i64, PulpError> {
        let mut init_score: i64 = self.score;
        let mut relevance = 0.0;
        let mut soundness = 0.0;
        // let mut stats_included = 0;
        let mut references = 0;
        // let mut syntax_and_grammar = 0;
        // let mut spelling = 0;
        let mut word_count: i64 = 0;
        let mut mastery_vocabulary: i64 = 0;

        let mut relevance_prompt = RelevanceContentPrompt::default();
        relevance_prompt.replace_attributes(vec![
            ("THIS_TOPIC".to_string(), self.topic_of_response.clone()),
            ("THIS_CONTENT".to_string(), self.content.clone()),
        ]);

        let mut soundness_prompt = SoundnessContentPrompt::default();
        soundness_prompt.replace_attributes(vec![
            ("THIS_TOPIC".to_string(), self.topic_of_response.clone()),
            ("THIS_CONTENT".to_string(), self.content.clone()),
        ]);

        let mut mastery_prompt = MasteryVocabContentPrompt::default();
        mastery_prompt.replace_attributes(vec![
            ("THIS_TOPIC".to_string(), self.topic_of_response.clone()),
            ("THIS_CONTENT".to_string(), self.content.clone()),
        ]);

        let (rel, mastery, sound): (
            Result<RelevanceResponse, JoinError>,
            Result<MasteryVocabResponse, JoinError>,
            Result<SoundnessResponse, JoinError>,
        ) = join!(
            task::spawn({
                let key = open_ai_key.clone();

                async move {
                    match relevance_prompt.clone().send(key).await {
                        Ok(rel) => {
                            println!("rel: {:?}", rel);

                            rel
                        }

                        Err(e) => {
                            println!("err: {:?}", e);

                            RelevanceResponse::default()
                        }
                    }
                }
            }),
            task::spawn({
                let key = open_ai_key.clone();

                async move {
                    match mastery_prompt.clone().send(key.clone()).await {
                        Ok(mast) => {
                            println!("mast: {:?}", mast);

                            mast
                        }

                        Err(e) => {
                            println!("err: {:?}", e);

                            MasteryVocabResponse::default()
                        }
                    }
                }
            }),
            task::spawn({
                let key = open_ai_key.clone();

                async move {
                    match soundness_prompt.clone().send(key).await {
                        Ok(sound) => {
                            println!("sound: {:?}", sound);

                            sound
                        }

                        Err(e) => {
                            println!("err: {:?}", e);

                            SoundnessResponse::default()
                        }
                    }
                }
            })
        );

        relevance = rel.unwrap().relevance;
        soundness = sound.unwrap().soundness;
        mastery_vocabulary = if let Some(words) = mastery.unwrap().mastery_words {
            let l = words.len() as i64;

            self.attributes = Attributes::new(
                relevance,
                soundness,
                references,
                self.content.split_whitespace().count() as i64,
                words,
            );

            l
        } else {
            self.attributes = Attributes::new(
                relevance,
                soundness,
                references,
                self.content.split_whitespace().count() as i64,
                vec!["".to_string()],
            );

            0
        };

        println!("relevance: {:?}", relevance);
        println!("soundness: {:?}", soundness);

        init_score +=
            (mastery_vocabulary + self.invalid_vote_count) * (relevance + soundness) as i64;

        println!("init_score: {:#?}", init_score);

        Ok(init_score)
    }

    pub fn calculate_reply_score(&self) -> i64 {
        0
    }

    // ==================================
    //           Neo4j Methods          =
    // ==================================

    pub async fn create(&self, graph: &Graph) -> Result<String, PulpError> {
        let id = Uuid::new_v4().to_string();
        let q = Query::new(
            "CREATE (r:Response {id: $id, content: $content, score: $score, valid_vote_count: $vvc, invalid_vote_count: $ivc, abstain_vote_count: $avc, hide_count: $hide_count, report_count: $report_count, ethos: $ethos, pathos: $pathos, logos: $logos}) RETURN (r.id)".to_string(),
        )
        .param("id", id.clone())
        .param("content", self.content.to_string())
        .param("score", 0)
        .param("vvc", 0)
        .param("ivc", 0)
        .param("avc", 0)
        .param("hide_count", 0)
        .param("report_count", 0)
        // .param("replies", vec![""])
        .param("ethos", 0.0)
        .param("pathos", 0.0)
        .param("logos", 0.0);

        match graph.start_txn().await {
            Ok(tx) => {
                match tx.execute(q).await {
                    Ok(mut res) => {
                        let row = res.next().await.unwrap().unwrap();

                        let id: Option<String> = row.get("(r.id)");

                        id
                    }

                    Err(e) => {
                        return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                            e.to_string(),
                        )));
                    }
                };

                if let Err(e) = tx.commit().await {
                    return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                        e.to_string(),
                    )));
                };
            }

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                    e.to_string(),
                )));
            }
        };

        Ok(id)
    }

    pub async fn get_response(&self, graph: &Graph) -> Result<Self, PulpError> {
        let q = Query::new("MATCH (r:Response {id: $id})".to_string()).param("id", self.id.clone());

        let response = match graph.start_txn().await {
            Ok(tx) => {
                let response = match tx.execute(q).await {
                    Ok(mut res) => {
                        let row = res.next().await.unwrap().unwrap();

                        let id: Option<String> = row.get("r.id");
                        let content: Option<String> = row.get("r.content");
                        let score: Option<i64> = row.get("r.score");
                        let valid_vote_count: Option<i64> = row.get("r.valid_vote_count");
                        let invalid_vote_count: Option<i64> = row.get("r.invalid_vote_count");
                        let abstain_vote_count: Option<i64> = row.get("r.abstain_vote_count");
                        let hide_count: Option<i64> = row.get("r.hide_count");
                        let report_count: Option<i64> = row.get("r.report_count");
                        let ethos: Option<f64> = row.get("r.ethos");
                        let pathos: Option<f64> = row.get("r.pathos");
                        let logos: Option<f64> = row.get("r.logos");

                        let mut response = Response::default();
                        response.content = content.unwrap();
                        response.id = id.unwrap();
                        response.score = score.unwrap();
                        response.valid_vote_count = valid_vote_count.unwrap();
                        response.invalid_vote_count = invalid_vote_count.unwrap();
                        response.abstain_vote_count = abstain_vote_count.unwrap();
                        response.hide_count = hide_count.unwrap();
                        response.report_count = report_count.unwrap();
                        response.ethos = ethos.unwrap();
                        response.pathos = pathos.unwrap();
                        response.logos = logos.unwrap();

                        response
                    }

                    Err(e) => {
                        return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                            e.to_string(),
                        )));
                    }
                };

                if let Err(e) = tx.commit().await {
                    return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                        e.to_string(),
                    )));
                };

                response
            }

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                    e.to_string(),
                )));
            }
        };

        Ok(response)
    }

    pub async fn update_response(&self, graph: &Graph) -> Result<(), PulpError> {
        let q = Query::new("MATCH (r:Response {id: $id}) SET r.content = $content, r.confidence = $confidence, r.score = $score, r.valid_vote_count = $vvc, r.invalid_vote_count = $ivc, r.abstain_vote_count = $avc, r.author_id = $author_id)".to_string())
            .param("id", self.id.clone())
            .param("content", self.content.clone())
            .param("confidence", self.confidence.to_string())
            .param("score", self.score)
            .param("vvc", self.valid_vote_count)
            .param("ivc", self.invalid_vote_count)
            .param("avc", self.abstain_vote_count);

        match graph.start_txn().await {
            Ok(tx) => {
                match tx.execute(q).await {
                    Ok(_) => {}

                    Err(e) => {
                        return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                            e.to_string(),
                        )));
                    }
                };

                if let Err(e) = tx.commit().await {
                    return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                        e.to_string(),
                    )));
                };
            }

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                    e.to_string(),
                )));
            }
        }

        Ok(())
    }

    pub async fn update_valid_vote_count(
        &self,
        graph: &Graph,
        count: i64,
    ) -> Result<(), PulpError> {
        let q =
            Query::new("MATCH (r:Response {id: $id}) SET r.valid_vote_count = $vvc".to_string())
                .param("id", self.id.clone())
                .param("vvc", count);

        match graph.start_txn().await {
            Ok(tx) => {
                match tx.execute(q).await {
                    Ok(_) => {}

                    Err(e) => {
                        return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                            e.to_string(),
                        )));
                    }
                };

                if let Err(e) = tx.commit().await {
                    return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                        e.to_string(),
                    )));
                };
            }

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                    e.to_string(),
                )));
            }
        }

        Ok(())
    }

    pub async fn update_invalid_vote_count(
        &self,
        graph: &Graph,
        count: i64,
    ) -> Result<(), PulpError> {
        let q =
            Query::new("MATCH (r:Response {id: $id}) SET r.invalid_vote_count = $ivc".to_string())
                .param("id", self.id.clone())
                .param("ivc", count);

        match graph.start_txn().await {
            Ok(tx) => {
                match tx.execute(q).await {
                    Ok(_) => {}

                    Err(e) => {
                        return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                            e.to_string(),
                        )));
                    }
                };

                if let Err(e) = tx.commit().await {
                    return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                        e.to_string(),
                    )));
                };
            }

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                    e.to_string(),
                )));
            }
        }

        Ok(())
    }
    pub async fn update_abstain_vote_count(
        &self,
        graph: &Graph,
        count: i64,
    ) -> Result<(), PulpError> {
        let q =
            Query::new("MATCH (r:Response {id: $id}) SET r.abstain_vote_count = $avc".to_string())
                .param("id", self.id.clone())
                .param("avc", count);

        match graph.start_txn().await {
            Ok(tx) => {
                match tx.execute(q).await {
                    Ok(_) => {}

                    Err(e) => {
                        return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                            e.to_string(),
                        )));
                    }
                };

                if let Err(e) = tx.commit().await {
                    return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                        e.to_string(),
                    )));
                };
            }

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                    e.to_string(),
                )));
            }
        }

        Ok(())
    }

    pub async fn update_score(&self, graph: &Graph, score: i64) -> Result<(), PulpError> {
        let q = Query::new("MATCH (r:Response {id: $id}) SET r.score = $score".to_string())
            .param("id", self.id.clone())
            .param("score", score);

        match graph.start_txn().await {
            Ok(tx) => {
                match tx.execute(q).await {
                    Ok(_) => {}

                    Err(e) => {
                        return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                            e.to_string(),
                        )));
                    }
                };

                if let Err(e) = tx.commit().await {
                    return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                        e.to_string(),
                    )));
                };
            }

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                    e.to_string(),
                )));
            }
        }

        Ok(())
    }

    pub async fn update_ethos(&self, graph: &Graph, val: f64) -> Result<(), PulpError> {
        let q = Query::new("MATCH (r:Response {id: $id}) SET r.ethos = $ethos".to_string())
            .param("id", self.id.clone())
            .param("ethos", val);

        match graph.start_txn().await {
            Ok(tx) => {
                match tx.execute(q).await {
                    Ok(_) => {}

                    Err(e) => {
                        return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                            e.to_string(),
                        )));
                    }
                };

                if let Err(e) = tx.commit().await {
                    return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                        e.to_string(),
                    )));
                };
            }

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                    e.to_string(),
                )));
            }
        }

        Ok(())
    }

    pub async fn update_logos(&self, graph: &Graph, val: f64) -> Result<(), PulpError> {
        let q = Query::new("MATCH (r:Response {id: $id}) SET r.logos = $logos".to_string())
            .param("id", self.id.clone())
            .param("logos", val);

        match graph.start_txn().await {
            Ok(tx) => {
                match tx.execute(q).await {
                    Ok(_) => {}

                    Err(e) => {
                        return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                            e.to_string(),
                        )));
                    }
                };

                if let Err(e) = tx.commit().await {
                    return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                        e.to_string(),
                    )));
                };
            }

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                    e.to_string(),
                )));
            }
        }

        Ok(())
    }

    pub async fn update_pathos(&self, graph: &Graph, val: f64) -> Result<(), PulpError> {
        let q = Query::new("MATCH (r:Response {id: $id}) SET r.pathos = $pathos".to_string())
            .param("id", self.id.clone())
            .param("pathos", val);

        match graph.start_txn().await {
            Ok(tx) => {
                match tx.execute(q).await {
                    Ok(_) => {}

                    Err(e) => {
                        return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                            e.to_string(),
                        )));
                    }
                };

                if let Err(e) = tx.commit().await {
                    return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                        e.to_string(),
                    )));
                };
            }

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                    e.to_string(),
                )));
            }
        }

        Ok(())
    }

    pub async fn delete_response(&self, graph: &Graph) -> Result<(), PulpError> {
        let q = Query::new("MATCH (r:Response {id: $id}) DETACH DELETE r".to_string())
            .param("id", self.id.clone());

        match graph.start_txn().await {
            Ok(tx) => {
                match tx.execute(q).await {
                    Ok(_) => {}

                    Err(e) => {
                        return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                            e.to_string(),
                        )));
                    }
                };

                if let Err(e) = tx.commit().await {
                    return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                        e.to_string(),
                    )));
                };
            }

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                    e.to_string(),
                )));
            }
        }

        Ok(())
    }

    pub async fn add_user_responded(&self, graph: &Graph, user: User) -> Result<(), PulpError> {
        let q = Query::new("MATCH (r:Response {id: $id}) MATCH (u:User {id: $user_id}) CREATE (u)-[:RESPONDED]->(r)".to_string())
            .param("id", self.id.clone())
            .param("user_id", user.id.clone());

        match graph.start_txn().await {
            Ok(tx) => {
                match tx.execute(q).await {
                    Ok(_) => {}

                    Err(e) => {
                        return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                            e.to_string(),
                        )));
                    }
                };

                if let Err(e) = tx.commit().await {
                    return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                        e.to_string(),
                    )));
                };
            }

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                    e.to_string(),
                )));
            }
        }

        Ok(())
    }

    pub async fn add_has_referecne(
        &self,
        graph: &Graph,
        reference: Reference,
    ) -> Result<(), PulpError> {
        let q = Query::new("MATCH (r:Response {id: $id}) MATCH (ref:Reference {id: $reference_id}) CREATE (r)-[:HAS_REFERENCE]->(ref)".to_string())
            .param("id", self.id.clone())
            .param("reference_id", reference.id.clone());

        match graph.start_txn().await {
            Ok(tx) => {
                match tx.execute(q).await {
                    Ok(_) => {}

                    Err(e) => {
                        return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                            e.to_string(),
                        )));
                    }
                };

                if let Err(e) = tx.commit().await {
                    return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                        e.to_string(),
                    )));
                };
            }

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                    e.to_string(),
                )));
            }
        }

        Ok(())
    }

    pub async fn add_reply_relationship(
        &self,
        graph: &Graph,
        reply: Self,
    ) -> Result<(), PulpError> {
        let q = Query::new("MATCH (r:Response {id: $id}) MATCH (rep:Response {id: $reply_id}) CREATE (r)-[:REPLIED]->(rep)".to_string())
            .param("id", self.id.clone())
            .param("reply_id", reply.id.clone());

        match graph.start_txn().await {
            Ok(tx) => {
                match tx.execute(q).await {
                    Ok(_) => {}

                    Err(e) => {
                        return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                            e.to_string(),
                        )));
                    }
                };

                if let Err(e) = tx.commit().await {
                    return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                        e.to_string(),
                    )));
                };
            }

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                    e.to_string(),
                )));
            }
        }

        Ok(())
    }

    pub async fn add_debate_response_relationship(
        &self,
        graph: &Graph,
        debate: Debate,
    ) -> Result<(), PulpError> {
        let q = Query::new("MATCH (r:Response {id: $id}) MATCH (d:Debate {id: $debate_id}) CREATE (r)-[:RESPONSE]->(d)".to_string())
            .param("id", self.id.clone())
            .param("debate_id", debate.id.clone());

        match graph.start_txn().await {
            Ok(tx) => {
                match tx.execute(q).await {
                    Ok(_) => {}

                    Err(e) => {
                        return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                            e.to_string(),
                        )));
                    }
                };

                if let Err(e) = tx.commit().await {
                    return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                        e.to_string(),
                    )));
                };
            }

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                    e.to_string(),
                )));
            }
        }

        Ok(())
    }
}
