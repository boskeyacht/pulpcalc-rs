use neo4rs::{Graph, Node, Query};
use pulpcalc_common::{
    errors::{PulpError, SimulationError},
    models::{Debate, Response, User},
};
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub enum Gender {
    Male,
    Female,
    #[default]
    Other,
    RatherNotSay,
}

impl From<&str> for Gender {
    fn from(value: &str) -> Self {
        match value {
            "male" => Gender::Male,
            "female" => Gender::Female,
            "other" => Gender::Other,
            "rather not say" => Gender::RatherNotSay,
            "rather_not_say" => Gender::RatherNotSay,
            _ => Gender::RatherNotSay,
        }
    }
}

impl ToString for Gender {
    fn to_string(&self) -> String {
        match self {
            Gender::Male => "male".to_string(),
            Gender::Female => "female".to_string(),
            Gender::Other => "other".to_string(),
            Gender::RatherNotSay => "rather_not_say".to_string(),
            _ => "rather_not_say".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum PoliticalOrientation {
    Right,
    Left,
    #[default]
    Center,
}

impl From<&str> for PoliticalOrientation {
    fn from(value: &str) -> Self {
        match value {
            "right" => PoliticalOrientation::Right,
            "left" => PoliticalOrientation::Left,
            "center" => PoliticalOrientation::Center,
            _ => PoliticalOrientation::Center,
        }
    }
}

impl ToString for PoliticalOrientation {
    fn to_string(&self) -> String {
        match self {
            PoliticalOrientation::Right => "right".to_string(),
            PoliticalOrientation::Left => "left".to_string(),
            PoliticalOrientation::Center => "center".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct EPL {
    pub ethos: f64,
    pub pathos: f64,
    pub logos: f64,
}

#[derive(Debug, Clone, Default)]
pub struct PersonasUser {
    pub age: i64,

    pub gender: Gender,

    pub political_orientation: PoliticalOrientation,

    pub vote_valid_influence: Vec<f64>,

    pub vote_invalid_influence: Vec<f64>,

    pub vote_abstain_influence: Vec<f64>,

    pub knoweledge: Knowledge,

    pub network: Network,

    pub personality: Personality,

    pub base_user: User,
}

#[derive(Debug, Clone, Default)]
pub struct Knowledge {
    pub knowledge_references: f64,
}

#[derive(Debug, Clone, Default)]
pub struct Network {
    pub network_size: NetworkSize,

    pub network_activity: NetworkActivity,

    pub network_composition: NetworkComposition,
}

#[derive(Debug, Clone, Default)]
pub struct Personality {
    pub personality_content: PersonalityContent,

    pub personality_base: PersonalityBase,

    pub personality_engagement: PersonalityEngagement,
}

#[derive(Debug, Clone, Default)]
pub struct NetworkSize {
    pub followers: i64,

    pub following: i64,
}

#[derive(Debug, Clone, Default)]
pub struct NetworkActivity {
    pub impressions: i64,

    pub engagements: i64,
}

#[derive(Debug, Clone, Default)]
pub struct NetworkComposition {
    pub network_personality: Vec<Personality>,
}

#[derive(Debug, Clone, Default)]
pub struct PersonalityContent {
    pub input: f64,

    pub output: f64,
}

#[derive(Debug, Clone, Default)]
pub struct PersonalityBase {
    pub core_fear: String,

    pub enneagram: i64,

    pub core_desire: String,
}

#[derive(Debug, Clone, Default)]
pub struct PersonalityEngagement {
    pub voting_tendency: (i64, i64, i64),

    pub hide_tendency: f64,

    pub report_tendency: f64,

    pub engagement_type: i64,
}

impl PersonasUser {
    pub fn new(
        age: i64,
        gender: &str,
        political_orientation: &str,
        vote_valid_influence: Vec<f64>,
        vote_invalid_influence: Vec<f64>,
        vote_abstain_influence: Vec<f64>,
        knowledge: Knowledge,
        network: Network,
        personality: Personality,
        base_user: User,
    ) -> Self {
        let g = Gender::from(gender);
        let po = PoliticalOrientation::from(political_orientation);

        Self {
            age,
            gender: g,
            political_orientation: po,
            vote_valid_influence,
            vote_invalid_influence,
            vote_abstain_influence,
            knoweledge: knowledge,
            network,
            personality,
            base_user,
        }
    }

    pub async fn create(&self, graph: &Graph) -> Result<String, PulpError> {
        let id = Uuid::new_v4().to_string();
        let (vv, iv, av) = self.personality.personality_engagement.voting_tendency;

        let q = Query::new("CREATE (pu:PersonaUser {id: $id, age: $age, gender: $gender, followers: $followers, following: $following, impressions: $impressions, engagements: $engagements, network_personality: $network_personality, input: $input, output: $output, core_fear: $core_fear, enneagram: $enneagram, core_desire: $core_desire, valid_voting_tendency: $valid_voting_tendency, invalid_voting_tendency: $invalid_voting_tendency, abstain_voting_tendency: $abstain_voting_tendency, hide_tendency: $hide_tendency, report_tendency: $report_tendency, engagement_type: $engagement_type, knowledge_references: $knowledge_references}) RETURN(pu.id)".to_string())
            .param("id", id.clone())
            .param("age", self.age)
            .param("gender", self.gender.to_string())
            .param("followers", self.network.network_size.followers)
            .param("following", self.network.network_size.following)
            .param("impressions", self.network.network_activity.impressions)
            .param("engagements", self.network.network_activity.engagements)
            .param("network_personality", "")
            .param("input", self.personality.personality_content.input)
            .param("output", self.personality.personality_content.output)
            .param("core_fear", self.personality.personality_base.core_fear.clone())
            .param("enneagram", self.personality.personality_base.enneagram)
            .param("core_desire", self.personality.personality_base.core_desire.clone())
            .param("valid_voting_tendency", vv)
            .param("invalid_voting_tendency", iv)
            .param("abstain_voting_tendency", av)
            .param("hide_tendency", self.personality.personality_engagement.hide_tendency)
            .param("report_tendency", self.personality.personality_engagement.report_tendency)
            .param("engagement_type", self.personality.personality_engagement.engagement_type)
            .param("knowledge_references", self.knoweledge.knowledge_references);

        match graph.start_txn().await {
            Ok(tx) => {
                let id = match tx.execute(q).await {
                    Ok(mut res) => {
                        let row = res.next().await.unwrap().unwrap();

                        let id: Option<String> = row.get("(pu.id)");

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

    pub async fn get_personas_user(&self, graph: &Graph) -> Result<Self, PulpError> {
        let q = Query::new("MATCH (pu:PersonaUser {id: $id}) RETURN(pu)".to_string())
            .param("id", self.base_user.id.clone());

        let personas_user = match graph.start_txn().await {
            Ok(tx) => {
                let user = match tx.execute(q).await {
                    Ok(mut res) => {
                        let row = res.next().await.unwrap().unwrap();

                        let mut p = PersonasUser::default();

                        let vt = (
                            row.get::<i64>("valid_voting_tendency").unwrap(),
                            row.get::<i64>("invalid_voting_tendency").unwrap(),
                            row.get::<i64>("abstain_voting_tendency").unwrap(),
                        );

                        p.base_user.id = row.get("id").unwrap();
                        p.network.network_size.followers = row.get("followers").unwrap();
                        p.network.network_size.following = row.get("following").unwrap();
                        p.network.network_activity.impressions = row.get("impressions").unwrap();
                        p.network.network_activity.engagements = row.get("engagements").unwrap();
                        // p.network.network_composition.network_personality = row.get("network_personality").unwrap();
                        p.personality.personality_content.input = row.get("input").unwrap();
                        p.personality.personality_content.output = row.get("output").unwrap();
                        p.personality.personality_base.core_fear = row.get("core_fear").unwrap();
                        p.personality.personality_base.enneagram = row.get("enneagram").unwrap();
                        p.personality.personality_base.core_desire =
                            row.get("core_desire").unwrap();
                        p.personality.personality_engagement.voting_tendency = vt;
                        p.personality.personality_engagement.hide_tendency =
                            row.get("hide_tendency").unwrap();
                        p.personality.personality_engagement.report_tendency =
                            row.get("report_tendency").unwrap();
                        p.personality.personality_engagement.engagement_type =
                            row.get("engagement_type").unwrap();
                        p.knoweledge.knowledge_references =
                            row.get("knowledge_references").unwrap();

                        p
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

                user
            }

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                    e.to_string(),
                )));
            }
        };

        Ok(personas_user)
    }

    pub async fn get_all_users(graph: &Graph) -> Result<Vec<PersonasUser>, PulpError> {
        let q = Query::new("MATCH (pu:PersonaUser) RETURN (pu)".to_string());

        let users = match graph.start_txn().await {
            Ok(tx) => {
                let users = match tx.execute(q).await {
                    Ok(mut res) => {
                        let mut users: Vec<PersonasUser> = Vec::new();

                        while let Some(row) = res.next().await.unwrap() {
                            let user_node: Node = row.get("pu").unwrap();
                            let mut user = PersonasUser::default();

                            println!("{:?}", row);

                            let vt = (
                                user_node.get::<i64>("valid_voting_tendency").unwrap(),
                                user_node.get::<i64>("invalid_voting_tendency").unwrap(),
                                user_node.get::<i64>("abstain_voting_tendency").unwrap(),
                            );

                            user.base_user.id = user_node.get("id").unwrap();
                            user.network.network_size.followers =
                                user_node.get("followers").unwrap();
                            user.network.network_size.following =
                                user_node.get("following").unwrap();
                            user.network.network_activity.impressions =
                                user_node.get("impressions").unwrap();
                            user.network.network_activity.engagements =
                                user_node.get("engagements").unwrap();
                            // user.network.network_composition.network_personality = row.get("network_personality").unwrap();
                            user.personality.personality_content.input =
                                user_node.get("input").unwrap();
                            user.personality.personality_content.output =
                                user_node.get("output").unwrap();
                            user.personality.personality_base.core_fear =
                                user_node.get("core_fear").unwrap();
                            user.personality.personality_base.enneagram =
                                user_node.get("enneagram").unwrap();
                            user.personality.personality_base.core_desire =
                                user_node.get("core_desire").unwrap();
                            user.personality.personality_engagement.voting_tendency = vt;
                            user.personality.personality_engagement.hide_tendency =
                                user_node.get("hide_tendency").unwrap();
                            user.personality.personality_engagement.report_tendency =
                                user_node.get("report_tendency").unwrap();
                            user.personality.personality_engagement.engagement_type =
                                user_node.get("engagement_type").unwrap();
                            user.knoweledge.knowledge_references =
                                user_node.get("knowledge_references").unwrap();

                            users.push(user);
                        }

                        users
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

                users
            }

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                    e.to_string(),
                )));
            }
        };

        Ok(users)
    }

    pub async fn update_personas_user(&self, graph: &Graph) -> Result<(), PulpError> {
        let (vv, iv, av) = self.personality.personality_engagement.voting_tendency;

        let q = Query::new("MACTH(pu:PersonaUser {id: $id}) SET pu.followers = $followers, pu.following = $following, pu.impressions = $impressions, pu.engagements = $engagements, pu.network_personality = $network_personality, pu.input = $input, pu.output = $output, pu.core_fear = $core_fear, pu.enneagram = $enneagram, pu.core_desire = $core_desire, pu.voting_tendency = $voting_tendency, pu.hide_tendency = $hide_tendency, pu.report_tendency = $report_tendency, pu.engagement_type = $engagement_type, pu.knowledge_references = $knowledge_references RETURN(pu.id)".to_string())
                .param("id", self.base_user.id.clone())
                .param("followers", self.network.network_size.followers)
                .param("following", self.network.network_size.following)
                .param("impressions", self.network.network_activity.impressions)
                .param("engagements", self.network.network_activity.engagements)
                // .param("network_personality", self.network.network_composition.network_personality)
                .param("input", self.personality.personality_content.input)
                .param("output", self.personality.personality_content.output)
                .param("core_fear", self.personality.personality_base.core_fear.clone())
                .param("enneagram", self.personality.personality_base.enneagram)
                .param("core_desire", self.personality.personality_base.core_desire.clone())
                .param("valid_voting_tendency", vv)
                .param("invalid_voting_tendency", iv)
                .param("abstain_voting_tendency", av)
                .param("hide_tendency", self.personality.personality_engagement.hide_tendency)
                .param("report_tendency", self.personality.personality_engagement.report_tendency)
                .param("engagement_type", self.personality.personality_engagement.engagement_type)
                .param("knowledge_references", self.knoweledge.knowledge_references);

        match graph.start_txn().await {
            Ok(tx) => {
                if let Err(e) = tx.run(q).await {
                    return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                        e.to_string(),
                    )));
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

    pub async fn delete_personas_user(&self, graph: &Graph) -> Result<(), PulpError> {
        let q = Query::new("MATCH (pu:PersonaUser {id: $id}) DETACH DELETE pu".to_string())
            .param("id", self.base_user.id.clone());

        match graph.start_txn().await {
            Ok(tx) => {
                if let Err(e) = tx.run(q).await {
                    return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                        e.to_string(),
                    )));
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

    pub async fn add_user_responded(
        &self,
        graph: &Graph,
        response: Response,
    ) -> Result<(), PulpError> {
        let q = Query::new("MATCH (pu:PersonaUser {id: $id}) MATCH (r:Response {id: $response_id}) CREATE (pu)-[:RESPONDED]->(r)".to_string())
                .param("id", self.base_user.id.clone())
                .param("response_id", response.id.clone());

        match graph.start_txn().await {
            Ok(tx) => {
                if let Err(e) = tx.run(q).await {
                    return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                        e.to_string(),
                    )));
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

    pub async fn add_user_participated(
        &self,
        graph: &Graph,
        debate: Debate,
    ) -> Result<(), PulpError> {
        let q = Query::new("MATCH (pu:PersonaUser {id: $id}) MATCH (d:Debate {id: $debate_id}) CREATE (d)-[r:PARTICIPATED]->(pu)".to_string())
                .param("id", self.base_user.id.clone())
                .param("debate_id", debate.id.clone());

        match graph.start_txn().await {
            Ok(tx) => {
                if let Err(e) = tx.run(q).await {
                    return Err(PulpError::SimulationError(SimulationError::Neo4jError(
                        e.to_string(),
                    )));
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
