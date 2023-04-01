use crate::personas::{
    models::{Learned, PersonasUser},
    prompts::{
        learned::{LearnedPrompt, LearnedResponse},
        response::{ContentResponse, PersonaContentPrompt, PersonaContentPromptWithReference},
        vote::{VoteContentPrompt, VoteResponse},
    },
};
use eyre::Result;
use futures::future::join_all;
use pulpcalc_common::{
    config::Config,
    errors::PulpError,
    llm_config::LLMRequest,
    models::{vote::VoteType, Debate, Reference, Response},
};
use rand::prelude::*;
use serde::Deserialize;
use tokio::task::{self, JoinHandle};

pub mod models;
mod prompts;

#[derive(Debug, Clone, Default)]
pub struct PersonasSimulation {
    pub simulation_type: String,

    pub simulation_size: i64,

    pub simulation_duration: i64,

    pub debates: Vec<Debate>,
}

impl PersonasSimulation {
    pub fn new(
        simulation_type: String,
        simulation_size: i64,
        simulation_duration: i64,
        debates: Vec<Debate>,
    ) -> Self {
        Self {
            simulation_type,
            simulation_size,
            simulation_duration,
            debates,
        }
    }

    pub fn simulation_type(&self) -> String {
        self.simulation_type.clone()
    }

    pub async fn run_simulation(
        &self,
        config: Config,
        personas_config: PersonasSimulationConfig,
    ) -> Result<(), PulpError> {
        let users = PersonasUser::get_all_users(&config.neo4j_graph).await?;
        let rint = (random::<f32>() * users.clone().len() as f32).floor() as usize;
        let rand_user = &users[rint];

        for mut debate in self.debates.clone() {
            let debate_id = debate.create(&config.neo4j_graph).await?;
            debate.id = debate_id;

            debate
                .update_commenters(
                    &config.neo4j_graph,
                    personas_config.max_commenters.unwrap().clone() as i64,
                )
                .await?;

            for _ in 0..personas_config.max_commenters.unwrap().clone() {
                let mut response = Response::default();

                let mut prompt = PersonaContentPrompt::default();
                prompt.replace_attributes(vec![
                    ("THIS_CONTENT".to_string(), response.content.clone()),
                    (
                        "POLITICAL_ORIENTATION".to_string(),
                        rand_user.political_orientation.to_string(),
                    ),
                    (
                        "ENNEAGRAM_TYPE".to_string(),
                        rand_user.personality.personality_base.enneagram.to_string(),
                    ),
                    ("GENDER".to_string(), rand_user.gender.to_string()),
                    ("AGE".to_string(), rand_user.age.to_string()),
                    (
                        "CORE_FEAR".to_string(),
                        rand_user.personality.personality_base.core_fear.clone(),
                    ),
                    (
                        "CORE_DESIRE".to_string(),
                        rand_user.personality.personality_base.core_desire.clone(),
                    ),
                ]);

                let content_res = match prompt.send(config.open_ai_key.clone()).await {
                    Ok(content) => content,

                    Err(e) => {
                        println!("{:?}", e);

                        ContentResponse::default()
                    }
                };

                response.content = content_res.content;
                response.ethos = content_res.ethos;
                response.pathos = content_res.pathos;
                response.logos = content_res.logos;

                let id = response.create(&config.neo4j_graph).await?;
                response.id = id;

                response
                    .update_ethos(&config.neo4j_graph, response.ethos)
                    .await?;
                response
                    .update_logos(&config.neo4j_graph, response.logos)
                    .await?;
                response
                    .update_pathos(&config.neo4j_graph, response.pathos)
                    .await?;

                Self::generate_votes(
                    config.clone(),
                    response.clone(),
                    debate.clone(),
                    personas_config.max_commenters.unwrap(),
                    users.clone(),
                )
                .await?;

                Self::generate_engagement(
                    config.clone(),
                    personas_config.clone(),
                    response.clone(),
                    users.clone(),
                    0,
                    3,
                    &mut debate,
                )
                .await?;

                response.score = response
                    .calculate_content_attribute_score(config.open_ai_key.clone())
                    .await?
                    + response.calculate_engagement_score();

                response
                    .update_score(&config.neo4j_graph, response.score)
                    .await?;

                response
                    .add_debate_response_relationship(&config.neo4j_graph, debate.clone())
                    .await?;

                rand_user
                    .add_user_responded(&config.neo4j_graph, response.clone())
                    .await?;
            }

            println!("Debate: {:?}", debate);

            debate
                .update_responses(&config.neo4j_graph, debate.responses)
                .await?;
        }

        Ok(())
    }

    pub async fn init_users() -> Vec<PersonasUser> {
        let mut users = vec![];
        for _ in 0..100 {
            users.push(PersonasUser::default());
        }

        users
    }

    /// Generates child responses to a given piece of content, uses user attributes to generate as "real"
    /// a response as possible
    pub async fn generate_engagement(
        config: Config,
        pcfg: PersonasSimulationConfig,
        response: Response,
        users: Vec<PersonasUser>,
        mut width: i64,
        mut depth: i64,
        debate: &mut Debate,
    ) -> Result<(), PulpError> {
        let key = config.open_ai_key.clone();

        let rint = (random::<f32>() * users.len() as f32).floor() as usize;
        let rand_user = &users[rint];

        let mut prompt = PersonaContentPrompt::default();
        prompt.replace_attributes(vec![
            ("THIS_CONTENT".to_string(), response.content.clone()),
            (
                "POLITICAL_ORIENTATION".to_string(),
                rand_user.political_orientation.to_string(),
            ),
            (
                "ENNEAGRAM_TYPE".to_string(),
                rand_user.personality.personality_base.enneagram.to_string(),
            ),
            ("GENDER".to_string(), rand_user.gender.to_string()),
            ("AGE".to_string(), rand_user.age.to_string()),
            (
                "CORE_FEAR".to_string(),
                rand_user.personality.personality_base.core_fear.clone(),
            ),
            (
                "CORE_DESIRE".to_string(),
                rand_user.personality.personality_base.core_desire.clone(),
            ),
        ]);

        let response_res = match prompt.send(key.clone()).await {
            Ok(content) => content,

            Err(e) => {
                println!("{:?}", e);

                ContentResponse::default()
            }
        };

        let mut response_reply = Response::default();
        response_reply.content = response_res.content.clone();
        response_reply.confidence = response_res.confidence as f64;
        response_reply.ethos = response_res.ethos;
        response_reply.pathos = response_res.pathos;
        response_reply.logos = response_res.logos;

        response_reply.score = response_reply
            .calculate_content_attribute_score(key.clone())
            .await?
            + response_reply.calculate_engagement_score();

        response
            .update_score(&config.neo4j_graph, response.score)
            .await?;

        depth -= 1;

        let response_reply_id = response_reply.create(&config.neo4j_graph).await?;
        response_reply.id = response_reply_id;

        debate.responses += 1;

        rand_user
            .add_user_responded(&config.neo4j_graph, response_reply.clone())
            .await?;

        response
            .add_reply_relationship(&config.neo4j_graph, response_reply.clone())
            .await?;

        Self::get_learned_attributes(&config, rand_user, &response, &response_reply, debate)
            .await?;

        // get reference

        let mut handles: Vec<JoinHandle<()>> = Vec::new();
        let mut res: Response = response_reply;
        while depth > 0 {
            let rint = (random::<f32>() * users.clone().len() as f32).floor() as usize;
            let rand_user = users[rint].clone();

            // TODO: references
            if pcfg.max_voters > Some(0) {
                let dh = task::spawn({
                    let config = config.clone();
                    let users = users.clone();
                    let response = response.clone();
                    let mut debate = debate.clone();
                    let key = key.clone();
                    let mut res = res.clone();

                    async move {
                        let mut prompt = PersonaContentPrompt::default();
                        prompt.replace_attributes(vec![
                            ("THIS_CONTENT".to_string(), response.content.clone()),
                            (
                                "POLITICAL_ORIENTATION".to_string(),
                                rand_user.political_orientation.to_string(),
                            ),
                            (
                                "ENNEAGRAM_TYPE".to_string(),
                                rand_user.personality.personality_base.enneagram.to_string(),
                            ),
                            ("GENDER".to_string(), rand_user.gender.to_string()),
                            ("AGE".to_string(), rand_user.age.to_string()),
                            (
                                "CORE_FEAR".to_string(),
                                rand_user.personality.personality_base.core_fear.clone(),
                            ),
                            (
                                "CORE_DESIRE".to_string(),
                                rand_user.personality.personality_base.core_desire.clone(),
                            ),
                        ]);

                        let response_res = match prompt.send(key.clone()).await {
                            Ok(content) => content,

                            Err(e) => {
                                println!("{:?}", e);

                                ContentResponse::default()
                            }
                        };

                        let mut depth_response_reply = Response::default();
                        depth_response_reply.content = response_res.content.clone();
                        depth_response_reply.confidence = response_res.confidence as f64;
                        depth_response_reply.references =
                            vec![response_res.reference.unwrap_or_default()];
                        depth_response_reply.ethos = response_res.ethos;
                        depth_response_reply.pathos = response_res.pathos;
                        depth_response_reply.logos = response_res.logos;

                        depth_response_reply.score = depth_response_reply
                            .calculate_content_attribute_score(key.clone())
                            .await
                            .expect("msg")
                            + depth_response_reply.calculate_engagement_score();

                        depth -= 1;

                        let depth_response_reply_id =
                            depth_response_reply.create(&config.neo4j_graph).await;
                        if let Ok(did) = depth_response_reply_id {
                            depth_response_reply.id = did;
                        }

                        rand_user
                            .add_user_responded(&config.neo4j_graph, depth_response_reply.clone())
                            .await;
                        res.add_reply_relationship(
                            &config.neo4j_graph,
                            depth_response_reply.clone(),
                        )
                        .await;

                        Self::get_learned_attributes(
                            &config,
                            &rand_user,
                            &res,
                            &depth_response_reply,
                            &debate,
                        )
                        .await;

                        debate.responses += 1;

                        depth_response_reply
                            .update_ethos(&config.neo4j_graph, depth_response_reply.ethos)
                            .await;
                        depth_response_reply
                            .update_logos(&config.neo4j_graph, depth_response_reply.logos)
                            .await;
                        depth_response_reply
                            .update_pathos(&config.neo4j_graph, depth_response_reply.pathos)
                            .await;

                        for reference in depth_response_reply.references.clone() {
                            let mut reff = Reference::default();

                            reff.content = reference.clone();
                            reff.internal = false;

                            let reff_id = reff.create(&config.neo4j_graph).await;
                            if let Ok(rid) = reff_id {
                                reff.id = rid;
                            }

                            reff.add_response_referenced_relationship(
                                &config.neo4j_graph,
                                depth_response_reply.clone(),
                            )
                            .await;
                        }

                        depth_response_reply
                            .update_score(&config.neo4j_graph, response.score)
                            .await;

                        rand_user
                            .add_user_responded(&config.neo4j_graph, depth_response_reply.clone())
                            .await;

                        println!("Reference Response: {:#?}", depth_response_reply.clone());

                        res.add_reply_relationship(
                            &config.neo4j_graph,
                            depth_response_reply.clone(),
                        )
                        .await;

                        Self::get_learned_attributes(
                            &config,
                            &rand_user,
                            &res,
                            &depth_response_reply,
                            &debate,
                        )
                        .await;

                        res = depth_response_reply;
                    }
                });

                handles.push(dh);
            } else {
                let dh = task::spawn({
                    let config = config.clone();
                    let users = users.clone();
                    let response = response.clone();
                    let mut debate = debate.clone();
                    let key = key.clone();
                    let mut res = res.clone();

                    async move {
                        let mut prompt = PersonaContentPrompt::default();
                        prompt.replace_attributes(vec![
                            ("THIS_CONTENT".to_string(), response.content.clone()),
                            (
                                "POLITICAL_ORIENTATION".to_string(),
                                rand_user.political_orientation.to_string(),
                            ),
                            (
                                "ENNEAGRAM_TYPE".to_string(),
                                rand_user.personality.personality_base.enneagram.to_string(),
                            ),
                            ("GENDER".to_string(), rand_user.gender.to_string()),
                            ("AGE".to_string(), rand_user.age.to_string()),
                            (
                                "CORE_FEAR".to_string(),
                                rand_user.personality.personality_base.core_fear.clone(),
                            ),
                            (
                                "CORE_DESIRE".to_string(),
                                rand_user.personality.personality_base.core_desire.clone(),
                            ),
                        ]);

                        let response_res = match prompt.send(key.clone()).await {
                            Ok(content) => content,

                            Err(e) => {
                                println!("{:?}", e);

                                ContentResponse::default()
                            }
                        };

                        let mut depth_response_reply = Response::default();
                        depth_response_reply.content = response_res.content.clone();
                        depth_response_reply.confidence = response_res.confidence as f64;
                        depth_response_reply.ethos = response_res.ethos;
                        depth_response_reply.pathos = response_res.pathos;
                        depth_response_reply.logos = response_res.logos;

                        depth_response_reply.score = depth_response_reply
                            .calculate_content_attribute_score(key.clone())
                            .await
                            .expect("msg")
                            + depth_response_reply.calculate_engagement_score();

                        depth -= 1;

                        depth_response_reply
                            .update_ethos(&config.neo4j_graph, depth_response_reply.ethos)
                            .await;
                        depth_response_reply
                            .update_logos(&config.neo4j_graph, depth_response_reply.logos)
                            .await;
                        depth_response_reply
                            .update_pathos(&config.neo4j_graph, depth_response_reply.pathos)
                            .await;

                        if let Ok(depth_response_reply_id) =
                            depth_response_reply.create(&config.neo4j_graph).await
                        {
                            depth_response_reply.id = depth_response_reply_id;
                        }

                        debate.responses += 1;

                        depth_response_reply
                            .update_score(&config.neo4j_graph, response.score)
                            .await;

                        rand_user
                            .add_user_responded(&config.neo4j_graph, depth_response_reply.clone())
                            .await;

                        println!("Response: {:?}", depth_response_reply.clone());

                        res.add_reply_relationship(
                            &config.neo4j_graph,
                            depth_response_reply.clone(),
                        )
                        .await;

                        Self::get_learned_attributes(
                            &config,
                            &rand_user,
                            &res,
                            &depth_response_reply,
                            &debate,
                        )
                        .await;

                        res = depth_response_reply;
                    }
                });

                handles.push(dh);
            }
        }

        join_all(handles).await;

        Ok(())
    }

    pub async fn get_learned_attributes(
        config: &Config,
        user: &PersonasUser,
        response: &Response,
        reply: &Response,
        debate: &Debate,
    ) -> Result<(), PulpError> {
        let mut prompt = LearnedPrompt::default();
        prompt.replace_attributes(vec![
            ("THIS_CONTENT".to_string(), response.content.clone()),
            ("THIS_RESPONSE".to_string(), reply.content.clone()),
            (
                "POLITICAL_ORIENTATION".to_string(),
                user.political_orientation.to_string(),
            ),
            (
                "ENNEAGRAM_TYPE".to_string(),
                user.personality.personality_base.enneagram.to_string(),
            ),
            ("AGE".to_string(), user.age.to_string()),
            ("GENDER".to_string(), user.gender.to_string()),
            (
                "CORE_FEAR".to_string(),
                user.personality.personality_base.core_fear.clone(),
            ),
            (
                "CORE_DESIRE".to_string(),
                user.personality.personality_base.core_desire.clone(),
            ),
        ]);

        let response_res = match prompt.send(config.open_ai_key.clone()).await {
            Ok(content) => content,

            Err(e) => {
                println!("{:?}", e);

                LearnedResponse::default()
            }
        };

        let mut learned = Learned::default();
        // TODO: add learned attributes
        learned.learned_content = response_res.political_orientation;

        let learned_id = learned.create(&config.neo4j_graph).await?;
        learned.id = learned_id;

        println!("Learned: {:#?}", learned);

        learned
            .add_user_learned(&config.neo4j_graph, user.clone())
            .await?;

        learned
            .add_learned_in(&config.neo4j_graph, debate.clone())
            .await?;

        learned
            .add_learned_from(&config.neo4j_graph, response.clone())
            .await?;

        Ok(())
    }

    /// Uses user attribtues to genereat votes for a given piece of content
    pub async fn generate_votes(
        config: Config,
        response: Response,
        debate: Debate,
        votes: u64,
        users: Vec<PersonasUser>,
    ) -> Result<(), PulpError> {
        debate
            .update_voters(&config.neo4j_graph, votes as i64)
            .await?;

        for _ in 0..votes {
            let rint = (random::<f32>() * users.clone().len() as f32).floor() as usize;
            let rand_user = users[rint].clone();

            // task::spawn({
            // let config = config.clone();
            // let response = response.clone();
            // let debate = debate.clone();
            // let rand_user = rand_user.clone();

            // async move {
            let mut vote = VoteContentPrompt::default();
            vote.replace_attributes(vec![
                ("THIS_CONTENT".to_string(), response.clone().content.clone()),
                (
                    "POLITICAL_ORIENTATION".to_string(),
                    rand_user.political_orientation.to_string(),
                ),
                (
                    "ENNEAGRAM_TYPE".to_string(),
                    rand_user.personality.personality_base.enneagram.to_string(),
                ),
                ("GENDER".to_string(), rand_user.gender.to_string()),
                ("AGE".to_string(), rand_user.age.to_string()),
                (
                    "CORE_FEAR".to_string(),
                    rand_user.personality.personality_base.core_fear.clone(),
                ),
                (
                    "CORE_DESIRE".to_string(),
                    rand_user.personality.personality_base.core_desire.clone(),
                ),
            ]);

            let vote_res = match vote.send(config.open_ai_key.clone()).await {
                Ok(content) => content,

                Err(e) => {
                    println!("{:?}", e);

                    VoteResponse::default()
                }
            };

            let mut votes = (0, 0, 0);
            let vote_type = VoteType::from(vote_res.vote.as_str());
            match vote_type {
                VoteType::Valid(_) => votes.0 += 1,

                VoteType::Invalid(_) => votes.1 += 1,

                VoteType::Abstain(_) => votes.2 += 1,
            }

            response
                .update_valid_vote_count(&config.neo4j_graph, votes.0)
                .await;
            response
                .update_invalid_vote_count(&config.neo4j_graph, votes.1)
                .await;
            response
                .update_abstain_vote_count(&config.neo4j_graph, votes.2)
                .await;
            // }
            // });
        }

        Ok(())
    }

    pub async fn simulate_debate() {
        todo!()
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Restrictions {
    pub min_user_score: u64,

    pub max_user_score: u64,

    pub male_distribution: f64,

    pub female_distribution: f64,

    pub other_distribution: f64,

    pub not_saying_gender_distribution: f64,

    pub min_age: u64,

    pub max_age: u64,

    pub right_leaning_distribution: f64,

    pub left_leaning_distribution: f64,

    pub center_leaning_distribution: f64,

    pub logos_level: u64,

    pub ethos_level: u64,

    pub pathos_level: u64,

    pub vote_valid_reason: Vec<f64>,

    pub vote_invalid_reason: Vec<f64>,

    pub vote_abstain_reason: Vec<f64>,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct DebateRules {
    pub vote_limit: Option<u64>,

    pub references_allowed: bool,

    pub must_include_reference_count: Option<u64>,

    pub banned_publications: Option<Vec<String>>,

    pub supported_publications: Option<Vec<String>>,

    pub banned_words: Option<Vec<String>>,

    pub response_time_limit: Option<u64>,
}

#[derive(Default, Deserialize, Debug, Clone)]
pub struct PersonasSimulationConfig {
    pub adults_only: bool,

    pub exclusive_debate: bool,

    pub user_restrictions: Option<Restrictions>,

    pub debate_rules: Option<DebateRules>,

    pub debate_topics: Option<Vec<String>>,

    pub debate_categories: Option<Vec<String>>,

    pub simulation_size: i64,

    pub max_keynote_speakers: Option<u64>,

    pub max_voters: Option<u64>,

    pub max_commenters: Option<u64>,

    pub max_references: Option<u64>,
}
