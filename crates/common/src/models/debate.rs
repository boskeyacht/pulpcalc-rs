use crate::errors::{PulpError, SimulationError};

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

    pub init_timestamp: i64,

    pub registration_timestamps: (i64, i64),

    pub competition_timestamps: (i64, i64),

    pub rewards_timestamps: (i64, i64),
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
        init_timestamp: i64,
        registration_timestamps: (i64, i64),
        competition_timestamps: (i64, i64),
        rewards_timestamps: (i64, i64),
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
            init_timestamp,
            registration_timestamps,
            competition_timestamps,
            rewards_timestamps,
        }
    }

    pub fn register_users(&self) {
        todo!("Fetch users from the database based on ... and register them to the debate")
    }

    pub fn choose_nominees(&self) -> Vec<User> {
        todo!("Choose nominees based on score")
    }

    // ----------------------------------
    //           Neo4j Methods          -
    // ----------------------------------

    pub async fn create(&self, graph: &Graph) -> Result<String, PulpError> {
        let id = Uuid::new_v4().to_string();

        let q = Query::new("CREATE(d: Debate {id: $id, score: $score, topic: $topic, category: $category, registered_speakers: $registered_speakers, commenters: $commenters, voters: $voters, inactive_participants: $inactive_participants, comments: $comments, responses: $responses}) RETURN (d.id)".to_string())
            .param("id", id.clone())
            .param("score", self.score.to_string())
            .param("topic", self.topic.to_string())
            .param("category", self.category.to_string())
            .param("registered_speakers", self.registered_speakers.to_string())
            .param("commenters", self.commenters.to_string())
            .param("voters", 1.to_string())
            .param("comments", self.comments.to_string())
            .param("inactive_participants", self.inactive_participants.to_string())
            .param("responses", self.responses.to_string());

        match graph.start_txn().await {
            Ok(tx) => {
                match tx.execute(q).await {
                    Ok(mut res) => {
                        let row = res.next().await.unwrap().unwrap();

                        let id: Option<String> = row.get("(d.id)");

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
        }

        Ok(id)
    }

    pub async fn get_debate(&self, graph: &Graph) -> Result<Self, PulpError> {
        let q = Query::new("MATCH (d:Debate {id: $id}) RETURN (d)".to_string())
            .param("id", self.id.clone());

        let debate = match graph.start_txn().await {
            Ok(tx) => {
                let debate = match tx.execute(q).await {
                    Ok(mut res) => {
                        let row = res.next().await.unwrap().unwrap();

                        let mut d = Debate::default();

                        d.voters = row.get("voters").unwrap_or_default();
                        d.responses = row.get("responses").unwrap_or_default();
                        d.comments = row.get("comments").unwrap_or_default();
                        d.inactive_participants =
                            row.get("inactive_participants").unwrap_or_default();
                        d.registered_speakers = row.get("registered_speakers").unwrap_or_default();
                        d.commenters = row.get("commenters").unwrap_or_default();
                        d.score = row.get("score").unwrap_or_default();
                        d.topic = row.get("topic").unwrap_or_default();
                        d.category = row.get("category").unwrap_or_default();

                        d
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

                debate
            }

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                    e.to_string(),
                )));
            }
        };

        Ok(debate)
    }

    pub async fn update_debate(&self, graph: &Graph) -> Result<(), PulpError> {
        let q = Query::new("MATCH (d:Debate {id: $id} SET d.topic = $topic, d.category = $category, d.commenters = $commenters, d.voters = $voters, d.responses = $responses})".to_string())
            .param("id", self.id.clone())
            .param("topic", self.topic.to_string())
            .param("category", self.category.to_string())
            .param("commenters", self.commenters.to_string())
            .param("voters", self.voters.to_string())
            .param("responses", self.responses.to_string());

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

    // pub async fn update_score(&self, graph: &Graph) {
    //     let q = Query::new("MATCH (d:Debate {id: $id} SET d.score = $score)".to_string())
    //         .param("id", self.id.clone())
    //         .param("score", self.score.to_string());

    //     let tx = graph.start_txn().await.unwrap();

    //     if let Err(e) = tx.run(q).await {
    //         println!("Error: {:#?}", e);
    //     }

    //     if let Err(e) = tx.commit().await {
    //         println!("Error: {:#?}", e);
    //     }
    // }

    pub async fn update_registered_speakers(&self, graph: &Graph) -> Result<(), PulpError> {
        let q = Query::new(
            "MATCH (d:Debate {id: $id} SET d.registered_speakers = $registered_speakers)"
                .to_string(),
        )
        .param("id", self.id.clone())
        .param("registered_speakers", self.registered_speakers.to_string());

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

    pub async fn update_commenters(&self, graph: &Graph, count: i64) -> Result<(), PulpError> {
        let q = Query::new("MATCH (d:Debate {id: $id}) SET d.commenters = $commenters".to_string())
            .param("id", self.id.clone())
            .param("commenters", count);

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

    pub async fn update_voters(&self, graph: &Graph, count: i64) -> Result<(), PulpError> {
        let q = Query::new("MATCH (d:Debate {id: $id}) SET d.voters = $voters".to_string())
            .param("id", self.id.clone())
            .param("voters", count);

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

    // pub async fn update_comments(&self, graph: &Graph) {
    //     let q = Query::new("MATCH (d:Debate {id: $id} SET d.comments = $comments)".to_string())
    //         .param("id", self.id.clone())
    //         .param("comments", self.score.to_string());

    //     let tx = graph.start_txn().await.unwrap();

    //     if let Err(e) = tx.run(q).await {
    //         println!("Error: {:#?}", e);
    //     }

    //     if let Err(e) = tx.commit().await {
    //         println!("Error: {:#?}", e);
    //     }
    // }

    pub async fn update_responses(&self, graph: &Graph, count: i64) -> Result<(), PulpError> {
        let q = Query::new("MATCH (d:Debate {id: $id}) SET d.responses = $responses".to_string())
            .param("id", self.id.clone())
            .param("responses", count);

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

    // pub async fn update_inactive_participants(&self, graph: &Graph) {
    //     let q = Query::new(
    //         "MATCH (d:Debate {id: $id} SET d.inactive_participants = $inactive_participants)"
    //             .to_string(),
    //     )
    //     .param("id", self.id.clone())
    //     .param(
    //         "inactive_participants",
    //         self.inactive_participants.to_string(),
    //     );

    //     let tx = graph.start_txn().await.unwrap();

    //     if let Err(e) = tx.run(q).await {
    //         println!("Error: {:#?}", e);
    //     }

    //     if let Err(e) = tx.commit().await {
    //         println!("Error: {:#?}", e);
    //     }
    // }

    pub async fn update_registration_start(&self, graph: &Graph) -> Result<(), PulpError> {
        let (start, _) = self.registration_timestamps;
        let q = Query::new(
            "MATCH (d:Debate {id: $id}) SET d.registration_start = $registration_start".to_string(),
        )
        .param("id", self.id.clone())
        .param("registration_start", start.to_string());

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

    pub async fn update_registration_end(&self, graph: &Graph) -> Result<(), PulpError> {
        let (_, end) = self.registration_timestamps;
        let q = Query::new(
            "MATCH (d:Debate {id: $id}) SET d.registration_end = $registration_end".to_string(),
        )
        .param("id", self.id.clone())
        .param("registration_end", end.to_string());

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

    pub async fn update_competition_start(&self, graph: &Graph) -> Result<(), PulpError> {
        let (start, _) = self.competition_timestamps;
        let q = Query::new(
            "MATCH (d:Debate {id: $id}) SET d.competition_start = $competition_start".to_string(),
        )
        .param("id", self.id.clone())
        .param("competition_start", start.to_string());

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

    pub async fn update_competition_end(&self, graph: &Graph) -> Result<(), PulpError> {
        let (_, end) = self.competition_timestamps;
        let q = Query::new(
            "MATCH (d:Debate {id: $id}) SET d.competition_end = $competition_end".to_string(),
        )
        .param("id", self.id.clone())
        .param("competition_end", end.to_string());

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

    pub async fn update_rewards_start(&self, graph: &Graph) -> Result<(), PulpError> {
        let (start, _) = self.rewards_timestamps;
        let q = Query::new(
            "MATCH (d:Debate {id: $id}) SET d.rewards_start = $rewards_start)".to_string(),
        )
        .param("id", self.id.clone())
        .param("rewards_start", start.to_string());

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

    pub async fn update_rewards_end(&self, graph: &Graph) -> Result<(), PulpError> {
        let (_, end) = self.rewards_timestamps;
        let q =
            Query::new("MATCH (d:Debate {id: $id}) SET d.rewards_end = $rewards_end".to_string())
                .param("id", self.id.clone())
                .param("rewards_end", end.to_string());

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

    pub async fn delete_debate(&self, graph: &Graph) -> Result<(), PulpError> {
        let q = Query::new("MATCH (d:Debate {id: $id}) DETACH DELETE d".to_string())
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

    pub async fn add_participant(&self, graph: &Graph, user: User) -> Result<(), PulpError> {
        let q = Query::new("MATCH (d:Debate {id: $id}) MATCH (u:User {id: $user_id}) CREATE (u)-[:PARTICIPATED]->(d)".to_string())
            .param("id", self.id.clone())
            .param("user_id", user.id.to_string());

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
