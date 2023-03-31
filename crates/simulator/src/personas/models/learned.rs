use super::super::PersonasUser;
use neo4rs::{Graph, Query};
use pulpcalc_common::{
    errors::{PulpError, SimulationError},
    models::{Debate, Response},
};
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

    pub async fn create(&self, graph: &Graph) -> Result<String, PulpError> {
        let id = Uuid::new_v4().to_string();
        let q = Query::new(
            "CREATE (l:Learned {id: $id, learned_content: $learned_content, reason: $reason}) RETURN(l.id)"
                .to_string(),
        )
        .param("id", id.clone())
        .param("learned_content", self.learned_content.to_string())
        .param("reason", self.reason.to_string());

        match graph.start_txn().await {
            Ok(tx) => {
                match tx.execute(q).await {
                    Ok(mut res) => {
                        let row = res.next().await.unwrap().unwrap();

                        let id: Option<String> = row.get("(l.id)");

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

    pub async fn get_learned(&self, graph: &Graph) -> Result<Self, PulpError> {
        let q = Query::new("MATCH (l:Learned {id: $id}) RETURN (l)".to_string())
            .param("id", self.id.clone());

        let learned = match graph.start_txn().await {
            Ok(tx) => {
                let learned = match tx.execute(q).await {
                    Ok(mut res) => {
                        let row = res.next().await.unwrap().unwrap();

                        let mut l = Learned::default();

                        l.id = row.get("(l.id)").unwrap_or_default();
                        l.learned_content = row.get("(l.learned_content)").unwrap_or_default();
                        l.reason = row.get("(l.reason)").unwrap_or_default();

                        l
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

                learned
            }

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                    e.to_string(),
                )));
            }
        };

        Ok(learned)
    }

    pub async fn update_learned(&self, graph: Graph) -> Result<(), PulpError> {
        let q = Query::new(
            "MATCH (l:Learned {id: $id}) SET l.learned_content = $learned_content, l.reason = $reason RETURN (l.id)"
                .to_string(),
        )
        .param("id", self.id.clone())
        .param("learned_content", self.learned_content.clone())
        .param("reason", self.reason.clone());

        match graph.start_txn().await {
            Ok(tx) => {
                if let Err(e) = tx.execute(q).await {
                    return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                        e.to_string(),
                    )));
                }

                if let Err(e) = tx.commit().await {
                    return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                        e.to_string(),
                    )));
                };

                Ok(())
            }

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                    e.to_string(),
                )));
            }
        }
    }

    pub async fn delete_learned(&self, graph: Graph) -> Result<(), PulpError> {
        let q = Query::new("MATCH (l:Learned {id: $id}) DETACH DELETE l".to_string())
            .param("id", self.id.clone());

        match graph.start_txn().await {
            Ok(tx) => {
                if let Err(e) = tx.execute(q).await {
                    return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                        e.to_string(),
                    )));
                }

                if let Err(e) = tx.commit().await {
                    return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                        e.to_string(),
                    )));
                };

                Ok(())
            }

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                    e.to_string(),
                )));
            }
        }
    }

    pub async fn add_user_learned(
        &self,
        graph: &Graph,
        user: PersonasUser,
    ) -> Result<(), PulpError> {
        let q = Query::new(
            "MATCH (l:Learned {id: $id}), (pu:PersonaUser {id: $user_id}) CREATE (pu)-[:LEARNED]->(l)"
                .to_string(),
        )
        .param("id", self.id.clone())
        .param("user_id", user.base_user.id);

        match graph.start_txn().await {
            Ok(tx) => {
                if let Err(e) = tx.execute(q).await {
                    return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                        e.to_string(),
                    )));
                }

                if let Err(e) = tx.commit().await {
                    return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                        e.to_string(),
                    )));
                };

                Ok(())
            }

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                    e.to_string(),
                )));
            }
        }
    }

    pub async fn add_learned_in(&self, graph: &Graph, debate: Debate) -> Result<(), PulpError> {
        let q = Query::new(
            "MATCH (l:Learned {id: $id}), (d:Debate {id: $debate_id}) CREATE (l)-[:LEARNED_IN]->(d)"
                .to_string(),
        )
        .param("id", self.id.clone())
        .param("debate_id", debate.id);

        match graph.start_txn().await {
            Ok(tx) => {
                if let Err(e) = tx.execute(q).await {
                    return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                        e.to_string(),
                    )));
                }

                if let Err(e) = tx.commit().await {
                    return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                        e.to_string(),
                    )));
                };

                Ok(())
            }

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                    e.to_string(),
                )));
            }
        }
    }

    pub async fn add_learned_from(
        &self,
        graph: &Graph,
        response: Response,
    ) -> Result<(), PulpError> {
        let q = Query::new(
            "MATCH (l:Learned {id: $id}), (r:Response {id: $response_id}) CREATE (l)-[:LEARNED_FROM]->(r)"
                .to_string(),
        )
        .param("id", self.id.clone())
        .param("response_id", response.id);

        match graph.start_txn().await {
            Ok(tx) => {
                if let Err(e) = tx.execute(q).await {
                    return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                        e.to_string(),
                    )));
                }

                if let Err(e) = tx.commit().await {
                    return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                        e.to_string(),
                    )));
                };

                Ok(())
            }

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                    e.to_string(),
                )));
            }
        }
    }
}
