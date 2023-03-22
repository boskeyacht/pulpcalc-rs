use pulpcalc_common::{
    config::Config,
    models::{Debate, Reference, Response, User},
};
use pulpcalc_external::chatgpt::ChatRequestBuilder;
use rand::prelude::*;
use regex::Regex;
use reqwest::Client;
use serde::Deserialize;
use serde_json::from_str;

use crate::enneagram::chat_responses::{ActionTendencies, ContentReponse, TendencyRespose};
use crate::enneagram::prompts::{
    ENNEAGRAM_REPLY_CONTENT_PROMPT, ENNEAGRAM_RESPONSE_CONTENT_PROMPT, ENNEAGRAM_TENDENCY_PROMPT,
};

mod chat_responses;
mod prompts;

#[derive(Debug, Default)]
pub struct EnneagramData {
    pub enneagram_type: i64,
}

#[derive(Debug, Default, Clone)]
pub struct EnneagramUser {
    pub base_user: User,

    pub tendencies: ActionTendencies,
}

impl EnneagramUser {
    pub fn new(user: User, tendencies: ActionTendencies) -> Self {
        EnneagramUser {
            base_user: user,
            tendencies,
        }
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct EnneagramSimulation {
    /// The simulation type (enneagram)
    pub simulation_type: String,

    /// The amount of users in the simulation
    pub simulation_size: u64,

    pub distribution: Vec<f64>,

    /// The depth of the simulation. The higher the number,
    /// the more replies will be created for any response
    pub depth: u64,

    /// The duration of the simulation
    pub simulation_duration: u64,

    /// The topic of the debate
    pub topic: String,

    /// The category of the dbeate
    pub category: String,
}

#[allow(dead_code)]
impl EnneagramSimulation {
    fn new(
        size: u64,
        depth: u64,
        duration: u64,
        topic: String,
        cat: String,
        dist: Vec<f64>,
    ) -> Self {
        let mut simulation = EnneagramSimulation::default();

        simulation.simulation_type = String::from("Enneagram");
        simulation.simulation_size = size;
        simulation.depth = depth;
        simulation.simulation_duration = duration;
        simulation.topic = topic;
        simulation.category = cat;
        simulation.distribution = dist;

        simulation
    }

    pub fn simulation_type(&self) -> String {
        String::from("Enneagram")
    }

    pub async fn run_simulation(&self, config: Config, mut debate: Debate) {
        let debate_id = debate.create(&config.neo4j_graph).await;
        debate.id = debate_id;

        let mut users: Vec<EnneagramUser> = Vec::new();

        let mut t = ENNEAGRAM_TENDENCY_PROMPT.to_string();
        t = t.replace("THIS_TOPIC", &self.topic.clone());

        let key = config.open_ai_key.clone();
        let tendency_chat_res = ChatRequestBuilder::new()
            .messages(t)
            .temperature(0.7)
            .max_tokens(850)
            .top_p(1.0)
            .presence_penalty(0.0)
            .frequency_penalty(0.0)
            .build()
            .send(key.clone(), Client::new())
            .await;

        let tendencies =
            from_str::<TendencyRespose>(&tendency_chat_res.choices[0].message.content.clone());

        let t_res = match tendencies {
            Ok(res) => Some(res),

            Err(e) => {
                println!(
                    "failed to unmarshal tendencies: {:?}: {}",
                    e, tendency_chat_res.choices[0].message.content
                );

                None
            }
        }
        .unwrap();

        // generate responses to that content
        for d in self.distribution.iter() {
            let mut i: f64 = 0.0;

            while i < d * self.simulation_size as f64 {
                let mut user = EnneagramUser::default();
                user.tendencies = t_res.clone().map_user_tendencies(i as i64);

                let user_id = user.base_user.create(&config.neo4j_graph).await;
                user.base_user.id = user_id;

                debate
                    .add_participant(&config.neo4j_graph, user.base_user.clone())
                    .await;

                users.push(user);

                i += 1.0;
            }
        }

        for _ in 1..self.simulation_size {
            let rint = (random::<f32>() * users.clone().len() as f32).floor() as usize;
            let rand_user = &users[rint];

            let mut cont_prompt = ENNEAGRAM_RESPONSE_CONTENT_PROMPT.to_string();
            cont_prompt = cont_prompt.replace(
                "VALID_VOTE_TENDENCY",
                &rand_user.tendencies.valid_vote_tendency.to_string(),
            );
            cont_prompt = cont_prompt.replace(
                "INVALID_VOTE_TENDENCY",
                &rand_user.tendencies.invalid_vote_tendency.to_string(),
            );
            cont_prompt = cont_prompt.replace(
                "ABSTAIN_VOTE_TENDENCY",
                &rand_user.tendencies.invalid_vote_tendency.to_string(),
            );
            cont_prompt = cont_prompt.replace(
                "REPORT_TENDENCY",
                &rand_user.tendencies.invalid_vote_tendency.to_string(),
            );
            cont_prompt = cont_prompt.replace(
                "HIDE_TENDENCY",
                &rand_user.tendencies.invalid_vote_tendency.to_string(),
            );
            cont_prompt = cont_prompt.replace("THIS_TOPIC", &self.topic.clone());

            let response_chat_res = ChatRequestBuilder::new()
                .messages(cont_prompt)
                .temperature(0.7)
                .max_tokens(850)
                .top_p(1.0)
                .presence_penalty(0.0)
                .frequency_penalty(0.0)
                .build()
                .send(key.clone(), Client::new())
                .await;

            let content =
                from_str::<ContentReponse>(&response_chat_res.choices[0].message.content.clone());

            let cont_res = match content {
                Ok(res) => Some(res),

                Err(e) => {
                    println!(
                        "failed to unmarshal content: {:?}: {}",
                        e, response_chat_res.choices[0].message.content
                    );

                    None
                }
            }
            .unwrap();

            let mut debate_response = Response::default();
            debate_response.content = cont_res.content.clone();
            debate_response.confidence = cont_res.confidence;

            debate_response.score = debate_response
                .calculate_content_attribute_score(key.clone())
                .await
                + debate_response.calculate_engagement_score();

            let debate_response_id = debate_response.create(&config.neo4j_graph).await;
            debate_response.id = debate_response_id;

            let re = Regex::new(r"/(?:(?:https?|ftp|file):\\|www\\.|ftp\\.)(?:\\([-A-Z0-9+&@#\\%=~_|$?!:,.]*\\)|[-A-Z0-9+&@#\\%=~_|$?!:,.])*(?:\\([-A-Z0-9+&@#\\%=~_|$?!:,.]*\\)|[A-Z0-9+&@#\\%=~_|$])/igm").unwrap();
            for link in re.find_iter(cont_res.content.as_str()) {
                let mut reference = Reference::default();
                reference.content = link.as_str().to_string();

                reference.create(&config.neo4j_graph).await;

                debate_response
                    .clone()
                    .add_has_referecne(&config.neo4j_graph, reference)
                    .await;
            }

            generate_engagement(&config, debate_response.clone(), self.depth, users.clone()).await;

            debate_response
                .add_user_responded(&config.neo4j_graph, rand_user.base_user.to_owned())
                .await;

            debate_response
                .add_debate_response_relationship(&config.neo4j_graph, debate.clone())
                .await;
        }
    }
}

pub async fn generate_engagement(
    config: &Config,
    response: Response,
    mut depth: u64,
    users: Vec<EnneagramUser>,
) {
    let key = config.open_ai_key.clone();

    let rint = (random::<f32>() * users.len() as f32).floor() as usize;
    let rand_user = &users[rint];

    let mut reply_prompt = ENNEAGRAM_REPLY_CONTENT_PROMPT.to_string();
    reply_prompt = reply_prompt.replace("THIS_CONTENT", &response.content.clone());
    reply_prompt = reply_prompt.replace(
        "VALID_VOTE_TENDENCY",
        &rand_user.tendencies.valid_vote_tendency.to_string(),
    );
    reply_prompt = reply_prompt.replace(
        "INVALID_VOTE_TENDENCY",
        &rand_user.tendencies.invalid_vote_tendency.to_string(),
    );
    reply_prompt = reply_prompt.replace(
        "ABSTAIN_VOTE_TENDENCY",
        &rand_user.tendencies.invalid_vote_tendency.to_string(),
    );
    reply_prompt = reply_prompt.replace(
        "REPORT_TENDENCY",
        &rand_user.tendencies.invalid_vote_tendency.to_string(),
    );
    reply_prompt = reply_prompt.replace(
        "HIDE_TENDENCY",
        &rand_user.tendencies.invalid_vote_tendency.to_string(),
    );

    let reply_chat_res = ChatRequestBuilder::new()
        .messages(reply_prompt)
        .temperature(0.7)
        .max_tokens(850)
        .top_p(1.0)
        .presence_penalty(0.0)
        .frequency_penalty(0.0)
        .build()
        .send(key.clone(), Client::new())
        .await;

    let content = from_str::<ContentReponse>(&reply_chat_res.choices[0].message.content.clone());

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
    .unwrap();

    let mut response_reply = Response::default();
    response_reply.content = cont_res.content.clone();
    response_reply.confidence = cont_res.confidence;

    response_reply.score = response_reply
        .calculate_content_attribute_score(key.clone())
        .await
        + response_reply.calculate_engagement_score();

    depth -= 1;

    let response_reply_id = response_reply.create(&config.neo4j_graph).await;
    response_reply.id = response_reply_id;

    let re = Regex::new(r"/(?:(?:https?|ftp|file):\\|www\\.|ftp\\.)(?:\\([-A-Z0-9+&@#\\%=~_|$?!:,.]*\\)|[-A-Z0-9+&@#\\%=~_|$?!:,.])*(?:\\([-A-Z0-9+&@#\\%=~_|$?!:,.]*\\)|[A-Z0-9+&@#\\%=~_|$])/igm").unwrap();
    for link in re.find_iter(cont_res.content.as_str()) {
        let mut reference = Reference::default();
        reference.content = link.as_str().to_string();

        reference.create(&config.neo4j_graph).await;

        response_reply
            .clone()
            .add_has_referecne(&config.neo4j_graph, reference)
            .await;
    }

    response_reply
        .add_user_responded(&config.neo4j_graph, rand_user.base_user.to_owned())
        .await;

    response
        .add_reply_relationship(&config.neo4j_graph, response_reply.clone())
        .await;

    // get reference

    let mut res: Response = response_reply;
    while depth > 0 {
        let rint = (random::<f32>() * users.len() as f32).floor() as usize;
        let rand_user = &users[rint];

        let mut reply_prompt = ENNEAGRAM_REPLY_CONTENT_PROMPT.to_string();
        reply_prompt = reply_prompt.replace("THIS_CONTENT", &res.content.clone());
        reply_prompt = reply_prompt.replace(
            "VALID_VOTE_TENDENCY",
            &rand_user.tendencies.valid_vote_tendency.to_string(),
        );
        reply_prompt = reply_prompt.replace(
            "INVALID_VOTE_TENDENCY",
            &rand_user.tendencies.invalid_vote_tendency.to_string(),
        );
        reply_prompt = reply_prompt.replace(
            "ABSTAIN_VOTE_TENDENCY",
            &rand_user.tendencies.invalid_vote_tendency.to_string(),
        );
        reply_prompt = reply_prompt.replace(
            "REPORT_TENDENCY",
            &rand_user.tendencies.invalid_vote_tendency.to_string(),
        );
        reply_prompt = reply_prompt.replace(
            "HIDE_TENDENCY",
            &rand_user.tendencies.invalid_vote_tendency.to_string(),
        );

        let reply_chat_res = ChatRequestBuilder::new()
            .messages(reply_prompt)
            .temperature(0.7)
            .max_tokens(850)
            .top_p(1.0)
            .presence_penalty(0.0)
            .frequency_penalty(0.0)
            .build()
            .send(key.clone(), Client::new())
            .await;

        let content =
            from_str::<ContentReponse>(&reply_chat_res.choices[0].message.content.clone());

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
        .unwrap();

        let mut depth_response_reply = Response::default();
        depth_response_reply.content = cont_res.content.clone();
        depth_response_reply.confidence = cont_res.confidence;

        depth_response_reply.score = depth_response_reply
            .calculate_content_attribute_score(key.clone())
            .await
            + depth_response_reply.calculate_engagement_score();

        depth -= 1;

        let depth_response_reply_id = depth_response_reply.create(&config.neo4j_graph).await;
        depth_response_reply.id = depth_response_reply_id;

        let re = Regex::new(r"/(?:(?:https?|ftp|file):\\|www\\.|ftp\\.)(?:\\([-A-Z0-9+&@#\\%=~_|$?!:,.]*\\)|[-A-Z0-9+&@#\\%=~_|$?!:,.])*(?:\\([-A-Z0-9+&@#\\%=~_|$?!:,.]*\\)|[A-Z0-9+&@#\\%=~_|$])/igm").unwrap();
        for link in re.find_iter(cont_res.content.as_str()) {
            let mut reference = Reference::default();
            reference.content = link.as_str().to_string();

            reference.create(&config.neo4j_graph).await;

            depth_response_reply
                .clone()
                .add_has_referecne(&config.neo4j_graph, reference)
                .await;
        }

        depth_response_reply
            .add_user_responded(&config.neo4j_graph, rand_user.base_user.to_owned())
            .await;

        println!("Response: {:?}", depth_response_reply);

        res.add_reply_relationship(&config.neo4j_graph, depth_response_reply.clone())
            .await;

        res = depth_response_reply;
    }
}

pub struct EnneagramSimulationBuilder {
    pub size: u64,
    pub depth: u64,
    pub duration: u64,
    pub topic: String,
    pub category: String,
    pub distribution: Vec<f64>,
}

impl EnneagramSimulationBuilder {
    pub fn new() -> Self {
        EnneagramSimulationBuilder {
            size: 0,
            depth: 0,
            duration: 0,
            topic: String::from(""),
            category: String::from(""),
            distribution: Vec::new(),
        }
    }

    pub fn size(mut self, size: u64) -> Self {
        self.size = size;
        self
    }

    pub fn depth(mut self, depth: u64) -> Self {
        self.depth = depth;
        self
    }

    pub fn duration(mut self, duration: u64) -> Self {
        self.duration = duration;
        self
    }

    pub fn topic(mut self, topic: String) -> Self {
        self.topic = topic;
        self
    }

    pub fn category(mut self, category: String) -> Self {
        self.category = category;
        self
    }

    pub fn distribution(mut self, distribution: Vec<f64>) -> Self {
        self.distribution = distribution;
        self
    }

    pub fn build(self) -> EnneagramSimulation {
        EnneagramSimulation::new(
            self.size,
            self.depth,
            self.duration,
            self.topic,
            self.category,
            self.distribution,
        )
    }
}
