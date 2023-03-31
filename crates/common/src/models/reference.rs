use super::response::Response;
use crate::errors::{PulpError, SimulationError};
use neo4rs::{Graph, Query};
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
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

    pub async fn create(&self, graph: &Graph) -> Result<String, PulpError> {
        let id = Uuid::new_v4().to_string();
        let q = Query::new("CREATE (ref:Reference {id: $id, internal: $internal, trust: $trust, distrust: $distrust, content: $content}) RETURN(ref.id)".to_string())
            .param("id", id.clone())
            .param("internal", self.internal.to_string())
            .param("trust", 0)
            .param("distrust", 0)
            .param("content", self.content.clone());

        match graph.start_txn().await {
            Ok(tx) => {
                let id = match tx.execute(q).await {
                    Ok(mut res) => {
                        let row = res.next().await.unwrap().unwrap();

                        let id: String = row.get("(ref.id)").unwrap();

                        Some(id)
                    }

                    Err(e) => {
                        return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                            e.to_string(),
                        )));
                    }
                }
                .unwrap();

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

    pub async fn get_reference(&self, graph: &Graph) -> Result<Self, PulpError> {
        let q = Query::new("MATCH (ref:Reference {id: $id})".to_string())
            .param("id", Uuid::new_v4().to_string());

        let reference = match graph.start_txn().await {
            Ok(tx) => {
                let reference = match tx.execute(q).await {
                    Ok(mut res) => {
                        let row = res.next().await.unwrap().unwrap();

                        let id: String = row.get("(ref.id)").unwrap();
                        let internal: bool = row.get("(ref.internal)").unwrap();
                        let trust: i64 = row.get("(ref.trust)").unwrap();
                        let distrust: i64 = row.get("(ref.distrust)").unwrap();
                        let content: String = row.get("(ref.content)").unwrap();

                        Self {
                            id,
                            internal,
                            trust,
                            distrust,
                            content,
                        }
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

                reference
            }

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                    e.to_string(),
                )));
            }
        };

        Ok(reference)
    }

    pub async fn update_reference(&self, graph: &Graph) -> Result<(), PulpError> {
        let q = Query::new("MATCH (ref:Reference {id: $id})".to_string())
            .param("id", Uuid::new_v4().to_string());

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

    pub async fn delete_reference(&self, graph: &Graph) -> Result<(), PulpError> {
        let q = Query::new("MATCH (ref:Reference {id: $id})".to_string())
            .param("id", Uuid::new_v4().to_string());

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

    pub async fn add_response_referenced_relationship(
        &self,
        graph: &Graph,
        response: Response,
    ) -> Result<(), PulpError> {
        let q = Query::new("MATCH (ref:Reference {id: $id}), (r:Response {id: $response_id}) CREATE (r)-[:REFERENCED]->(ref)".to_string())
            .param("id", self.id.clone())
            .param("response_id", response.id);

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
