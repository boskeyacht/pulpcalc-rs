use crate::errors::{PulpError, SimulationError};

use super::vote::Vote;
use neo4rs::{Graph, Node, Query};
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

    /// Initializes a set of users for use across multiple debates.
    /// User data is updated across said debates which simulates users "learning", as they enter
    /// and engage in different debates.
    pub async fn initialize_users(&self, graph: &Graph, users: i64) {
        for _ in 0..users {
            println!("users")
        }
    }

    pub async fn create(&self, graph: &Graph) -> Result<String, PulpError> {
        let id = Uuid::new_v4().to_string();
        let q = Query::new(
            "CREATE (u:User {id: $id,  debates: $debates, votes: $votes, simulation_data: $simulation_data}) RETURN(u.id)"
                .to_string(),
        )
        .param("id", id.clone())
        .param("debates", vec![""])
        .param("votes", vec![""])
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

        Ok(id)
    }

    pub async fn get_user(&self, graph: &Graph) -> Result<User, PulpError> {
        let q = Query::new("MATCH (u:User {id: $id})".to_string())
            .param("id", Uuid::new_v4().to_string());

        let user = match graph.start_txn().await {
            Ok(tx) => {
                let user = match tx.execute(q).await {
                    Ok(mut res) => {
                        let row = res.next().await.unwrap().unwrap();

                        let user_node: Node = row.get("u").unwrap();
                        let mut user = User::default();

                        user.id = user_node.get::<String>("id").unwrap();
                        user.simulation_data = user_node.get::<String>("simulation_data").unwrap();

                        user
                    }

                    Err(e) => {
                        println!("Error: {:#?}", e);

                        User::default()
                    }
                };

                if let Err(e) = tx.commit().await {
                    println!("Error: {:#?}", e);
                };

                user
            }

            Err(e) => {
                println!("Error: {:#?}", e);

                User::default()
            }
        };

        Ok(user)
    }

    pub async fn get_all_users(graph: &Graph) -> Result<Vec<User>, PulpError> {
        let q = Query::new("MATCH (u:User) RETURN u".to_string());

        let tx = graph.start_txn().await.unwrap();

        let users = match tx.execute(q).await {
            Ok(mut res) => {
                let mut users: Vec<User> = Vec::new();

                while let Some(row) = res.next().await.unwrap() {
                    let user_node: Node = row.get("u").unwrap();
                    let mut user = User::default();

                    println!("{:#?}", user_node.get::<String>("id"));
                    user.id = user_node.get::<String>("id").unwrap();
                    user.simulation_data = user_node.get::<String>("simulation_data").unwrap();

                    users.push(user);
                }

                if let Err(e) = tx.commit().await {
                    println!("Error: {:#?}", e);
                };

                users
            }

            Err(e) => {
                println!("Error: {:#?}", e);

                vec![]
            }
        };

        Ok(users)
    }

    // TODO: string vecs
    pub async fn update_user(&self, graph: &Graph) -> Result<(), PulpError> {
        let q = Query::new(
            "MATCH (u:User {id: $id} SET u.debates = $debates, u.votes = $votes)".to_string(),
        )
        .param("id", Uuid::new_v4().to_string())
        .param("debates", "self.debates".to_string())
        .param("votes", "self.votes".to_string());

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

    /// Updates the debates the user has participated in.
    /// This function will overwrite whatever is currently in the debates field.
    pub async fn update_user_debates(&self, graph: &Graph) -> Result<(), PulpError> {
        let q = Query::new("MATCH (u:User {id: $id} SET u.debates = $debates)".to_string())
            .param("id", Uuid::new_v4().to_string())
            .param("debates", self.debates.clone());

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

    /// Updates the debates the user has participated in.
    /// This function will add to the current set of debates (in other words, it will not overwrite)
    pub async fn add_user_debate(&self, graph: &Graph, debate_id: String) -> Result<(), PulpError> {
        let q =
            Query::new("MATCH (u:User {id: $id} SET u.debates = u.debates + $debate)".to_string())
                .param("id", Uuid::new_v4().to_string())
                .param("debates", debate_id);

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

    pub async fn add_user_votes(&self, graph: Graph) -> Result<(), PulpError> {
        let mut vs: Vec<String> = Vec::new();
        for v in self.votes.clone() {
            vs.push(v.id)
        }

        let q = Query::new("MATCH (u:User {id: $id} SET u.votes = $votes)".to_string())
            .param("id", Uuid::new_v4().to_string())
            .param("votes", vs);

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

    pub async fn delete_user(&self, graph: &Graph) -> Result<(), PulpError> {
        let q = Query::new("MATCH (u:User {id: $id})".to_string())
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
}
