use crate::personas::models::{Learned, PersonasUser};
use crate::personas::prompts::*;
use pulpcalc_common::{
    config::Config,
    models::{vote::VoteType, Debate, Response},
};
use pulpcalc_external::chatgpt::ChatRequestBuilder;
use rand::prelude::*;
use reqwest::Client;
use serde::Deserialize;
use serde_json::from_str;

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
    pub fn new() {}

    pub fn simulation_type(&self) -> String {
        self.simulation_type.clone()
    }

    pub async fn run_simulation(&self, config: Config, personas_config: PerosnasSimulationConfig) {
        let users = PersonasUser::get_all_users(&config.neo4j_graph).await;
        let rint = (random::<f32>() * users.clone().len() as f32).floor() as usize;
        let mut rand_user = &users[rint];

        println!("User: {:#?}", rand_user);

        let mut debates: Vec<Debate> = Vec::new();
        for mut debate in self.debates.clone() {
            let debate_id = debate.create(&config.neo4j_graph).await;
            debate.id = debate_id;

            rand_user
                .add_user_participated(&config.neo4j_graph, debate.clone())
                .await;

            debates.push(debate);
        }

        // let mut dh = vec![];
        let vl = personas_config
            .clone()
            .debate_rules
            .unwrap()
            .vote_limit
            .unwrap();
        let must_include_reference_count = personas_config
            .clone()
            .debate_rules
            .unwrap()
            .must_include_reference_count;
        let banned = personas_config
            .clone()
            .debate_rules
            .unwrap()
            .banned_publications;
        let supported = personas_config
            .clone()
            .debate_rules
            .unwrap()
            .supported_publications
            .unwrap();

        for mut debate in debates {
            for _ in 0..personas_config.max_voters.unwrap().clone() {
                let mut response = Response::default();
                debate.commenters += 1;

                let mut p = response::PERSONA_CONTENT_PROMPT.to_string();
                p = p.replace("THIS_CONTENT", response.content.clone().as_str());
                p = p.replace(
                    "POLITICAL_ORIENTATION",
                    rand_user.political_orientation.to_string().as_str(),
                );
                p = p.replace(
                    "ENNEAGRAM_TYPE",
                    rand_user
                        .personality
                        .personality_base
                        .enneagram
                        .to_string()
                        .as_str(),
                );
                p = p.replace("GENDER", rand_user.gender.to_string().as_str());
                p = p.replace("AGE", rand_user.age.to_string().as_str());
                p = p.replace(
                    "CORE_FEAR",
                    rand_user.personality.personality_base.core_fear.as_str(),
                );
                p = p.replace(
                    "CORE_DESIRE",
                    rand_user.personality.personality_base.core_desire.as_str(),
                );

                let persona_chat_res = ChatRequestBuilder::new()
                    .messages(p)
                    .temperature(0.7)
                    .max_tokens(800)
                    .top_p(1.0)
                    .presence_penalty(0.0)
                    .frequency_penalty(0.0)
                    .build()
                    .send(config.open_ai_key.clone().to_string(), Client::new())
                    .await;

                let persona = from_str::<response::ContentResponse>(
                    &persona_chat_res.choices[0].message.content.clone(),
                );

                let persona_res = match persona {
                    Ok(res) => Some(res),

                    Err(e) => {
                        println!(
                            "failed to unmarshal tendencies: {:?}: {}",
                            e, persona_chat_res.choices[0].message.content
                        );

                        None
                    }
                }
                .unwrap();

                println!("Persona: {:#?}", persona_res);
                response.content = persona_res.content;

                response.score = response
                    .calculate_content_attribute_score(config.open_ai_key.clone())
                    .await
                    + response.calculate_engagement_score();

                response
                    .update_score(&config.neo4j_graph, response.score)
                    .await;

                let mut vv: i64 = 0;
                let mut iv: i64 = 0;
                let mut av: i64 = 0;
                for _ in 0..vl {
                    let mut v = vote::VOTE_CONTENT_PROMPT.to_string();
                    v = v.replace("THIS_CONTENT", response.clone().content.clone().as_str());
                    v = v.replace(
                        "POLITICAL_ORIENTATION",
                        rand_user.political_orientation.to_string().as_str(),
                    );
                    v = v.replace(
                        "ENNEAGRAM_TYPE",
                        rand_user
                            .personality
                            .personality_base
                            .enneagram
                            .to_string()
                            .as_str(),
                    );
                    v = v.replace("GENDER", rand_user.gender.to_string().as_str());
                    v = v.replace("AGE", rand_user.age.to_string().as_str());
                    v = v.replace(
                        "CORE_FEAR",
                        rand_user.personality.personality_base.core_fear.as_str(),
                    );
                    v = v.replace(
                        "CORE_DESIRE",
                        rand_user.personality.personality_base.core_desire.as_str(),
                    );

                    let vote_chat_res = ChatRequestBuilder::new()
                        .messages(v)
                        .temperature(0.7)
                        .max_tokens(800)
                        .top_p(1.0)
                        .presence_penalty(0.0)
                        .frequency_penalty(0.0)
                        .build()
                        .send(config.open_ai_key.clone().to_string(), Client::new())
                        .await;

                    let vote = from_str::<vote::VoteResponse>(
                        &vote_chat_res.choices[0].message.content.clone(),
                    );

                    let vote_res = match vote {
                        Ok(res) => Some(res),

                        Err(e) => {
                            println!(
                                "failed to unmarshal tendencies: {:?}: {}",
                                e, vote_chat_res.choices[0].message.content
                            );

                            None
                        }
                    }
                    .unwrap();

                    let vote_type = VoteType::from(vote_res.vote.as_str());
                    match vote_type {
                        VoteType::Valid(_) => vv += 1,

                        VoteType::Invalid(_) => iv += 1,

                        VoteType::Abstain(_) => av += 1,
                    }
                }

                println!("Valid Votes: {}", vv);
                println!("Invalid Votes: {}", iv);
                println!("Abstain Votes: {}", av);

                response
                    .update_valid_vote_count(&config.neo4j_graph, vv)
                    .await;
                response
                    .update_invalid_vote_count(&config.neo4j_graph, iv)
                    .await;
                response
                    .update_abstain_vote_count(&config.neo4j_graph, av)
                    .await;

                let id = response.create(&config.neo4j_graph).await;
                response.id = id;

                Self::generate_engagement(
                    &config,
                    &personas_config,
                    response.clone(),
                    &users,
                    3,
                    &debate,
                )
                .await;

                rand_user
                    .add_user_responded(&config.neo4j_graph, response.clone())
                    .await;

                response
                    .add_debate_response_relationship(&config.neo4j_graph, debate.clone())
                    .await;

                let rint = (random::<f32>() * users.clone().len() as f32).floor() as usize;
                rand_user = &users[rint];

                println!("Debate: {:?}", debate);
            }
        }

        for debate in self.debates.clone() {
            let nominees = debate.choose_nominees();

            println!("Nominees: {:?}", nominees);
        }
    }

    pub async fn init_users() -> Vec<PersonasUser> {
        let mut users = vec![];
        for _ in 0..100 {
            users.push(PersonasUser::default());
        }

        users
    }

    pub async fn generate_engagement(
        config: &Config,
        pcfg: &PerosnasSimulationConfig,
        response: Response,
        users: &Vec<PersonasUser>,
        mut depth: i64,
        debate: &Debate,
    ) {
        let key = config.open_ai_key.clone();

        let rint = (random::<f32>() * users.len() as f32).floor() as usize;
        let rand_user = &users[rint];

        let mut p = response::PERSONA_CONTENT_PROMPT.to_string();
        p = p.replace("THIS_CONTENT", response.content.clone().as_str());
        p = p.replace(
            "POLITICAL_ORIENTATION",
            rand_user.political_orientation.to_string().as_str(),
        );
        p = p.replace(
            "ENNEAGRAM_TYPE",
            rand_user
                .personality
                .personality_base
                .enneagram
                .to_string()
                .as_str(),
        );
        p = p.replace("GENDER", rand_user.gender.to_string().as_str());
        p = p.replace("AGE", rand_user.age.to_string().as_str());
        p = p.replace(
            "CORE_FEAR",
            rand_user.personality.personality_base.core_fear.as_str(),
        );
        p = p.replace(
            "CORE_DESIRE",
            rand_user.personality.personality_base.core_desire.as_str(),
        );

        let persona_chat_res = ChatRequestBuilder::new()
            .messages(p)
            .temperature(0.7)
            .max_tokens(800)
            .top_p(1.0)
            .presence_penalty(0.0)
            .frequency_penalty(0.0)
            .build()
            .send(key.clone(), Client::new())
            .await;

        let persona = from_str::<response::ContentResponse>(
            &persona_chat_res.choices[0].message.content.clone(),
        );

        let persona_res = match persona {
            Ok(res) => Some(res),

            Err(e) => {
                println!(
                    "failed to unmarshal tendencies: {:?}: {}",
                    e, persona_chat_res.choices[0].message.content
                );

                None
            }
        }
        .unwrap();

        let mut response_reply = Response::default();
        response_reply.content = persona_res.content.clone();
        response_reply.confidence = persona_res.confidence as f64;

        response_reply.score = response_reply
            .calculate_content_attribute_score(key.clone())
            .await
            + response_reply.calculate_engagement_score();

        response
            .update_score(&config.neo4j_graph, response.score)
            .await;

        depth -= 1;

        let response_reply_id = response_reply.create(&config.neo4j_graph).await;
        response_reply.id = response_reply_id;

        rand_user
            .add_user_responded(&config.neo4j_graph, response_reply.clone())
            .await;

        response
            .add_reply_relationship(&config.neo4j_graph, response_reply.clone())
            .await;

        Self::get_learned_attributes(&config, rand_user, &response, &response_reply, debate).await;

        // get reference

        let mut res: Response = response_reply;
        while depth > 0 {
            let rint = (random::<f32>() * users.len() as f32).floor() as usize;
            let rand_user = &users[rint];

            let mut p = response::PERSONA_CONTENT_PROMPT.to_string();
            p = p.replace("THIS_CONTENT", response.content.clone().as_str());
            p = p.replace(
                "POLITICAL_ORIENTATION",
                rand_user.political_orientation.to_string().as_str(),
            );
            p = p.replace(
                "ENNEAGRAM_TYPE",
                rand_user
                    .personality
                    .personality_base
                    .enneagram
                    .to_string()
                    .as_str(),
            );
            p = p.replace("GENDER", rand_user.gender.to_string().as_str());
            p = p.replace("AGE", rand_user.age.to_string().as_str());
            p = p.replace(
                "CORE_FEAR",
                rand_user.personality.personality_base.core_fear.as_str(),
            );
            p = p.replace(
                "CORE_DESIRE",
                rand_user.personality.personality_base.core_desire.as_str(),
            );

            let reply_chat_res = ChatRequestBuilder::new()
                .messages(p)
                .temperature(0.7)
                .max_tokens(850)
                .top_p(1.0)
                .presence_penalty(0.0)
                .frequency_penalty(0.0)
                .build()
                .send(key.clone(), Client::new())
                .await;

            let content = from_str::<response::ContentResponse>(
                &reply_chat_res.choices[0].message.content.clone(),
            );

            let cont_res = match content {
                Ok(res) => Some(res),

                Err(e) => {
                    println!(
                        "failed to unmarshal content: {:?}: {}",
                        e, reply_chat_res.choices[0].message.content
                    );

                    None
                }
            }
            .unwrap_or_default();

            let mut depth_response_reply = Response::default();
            depth_response_reply.content = cont_res.content.clone();
            depth_response_reply.confidence = cont_res.confidence as f64;

            depth_response_reply.score = depth_response_reply
                .calculate_content_attribute_score(key.clone())
                .await
                + depth_response_reply.calculate_engagement_score();

            depth_response_reply
                .update_score(&config.neo4j_graph, response.score)
                .await;

            depth -= 1;

            let depth_response_reply_id = depth_response_reply.create(&config.neo4j_graph).await;
            depth_response_reply.id = depth_response_reply_id;

            rand_user
                .add_user_responded(&config.neo4j_graph, depth_response_reply.clone())
                .await;

            println!("Response: {:?}", depth_response_reply.clone());

            res.add_reply_relationship(&config.neo4j_graph, depth_response_reply.clone())
                .await;

            Self::get_learned_attributes(&config, rand_user, &res, &depth_response_reply, debate)
                .await;

            res = depth_response_reply;
        }
    }

    pub async fn get_learned_attributes(
        config: &Config,
        user: &PersonasUser,
        response: &Response,
        reply: &Response,
        debate: &Debate,
    ) {
        let mut l = learned::LEARNED_PROMPT.to_string();
        l = l.replace("THIS_CONTENT", response.content.clone().as_str());
        l = l.replace("THIS_RESPONSE", reply.content.clone().as_str());
        l = l.replace(
            "POLITICAL_ORIENTATION",
            user.political_orientation.to_string().as_str(),
        );
        l = l.replace(
            "ENNEAGRAM_TYPE",
            user.personality
                .personality_base
                .enneagram
                .to_string()
                .as_str(),
        );
        l = l.replace("AGE", user.age.to_string().as_str());
        l = l.replace("GENDER", user.gender.to_string().as_str());
        l = l.replace(
            "CORE_DESIRE",
            user.personality.personality_base.core_desire.as_str(),
        );
        l = l.replace(
            "CORE_FEAR",
            user.personality.personality_base.core_fear.as_str(),
        );

        let learned_chat_res = ChatRequestBuilder::new()
            .messages(l)
            .temperature(0.7)
            .max_tokens(800)
            .top_p(1.0)
            .presence_penalty(0.0)
            .frequency_penalty(0.0)
            .build()
            .send(config.open_ai_key.clone(), Client::new())
            .await;

        let mut learned = Learned::default();
        learned.learned_content = learned_chat_res.choices[0].message.content.clone();

        let learned_id = learned.create(&config.neo4j_graph).await;
        learned.id = learned_id;

        println!("Learned: {:?}", learned);

        learned
            .add_user_learned(&config.neo4j_graph, user.clone())
            .await;

        learned
            .add_learned_in(&config.neo4j_graph, debate.clone())
            .await;

        learned
            .add_learned_from(&config.neo4j_graph, response.clone())
            .await;
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
pub struct PerosnasSimulationConfig {
    pub adults_only: bool,

    pub exclusive_debate: bool,

    pub user_restrictions: Option<Restrictions>,

    pub debate_rules: Option<DebateRules>,

    pub debate_topics: Option<Vec<String>>,

    pub debate_categories: Option<Vec<String>>,

    pub simulation_size: i64,

    pub max_keynote_speakers: Option<u64>,

    pub max_voters: Option<u64>,

    pub max_references: Option<u64>,
}
