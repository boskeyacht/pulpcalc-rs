use pulpcalc_common::{
    config::Config,
    models::{Response, User},
};
use pulpcalc_external::chatgpt::ChatRequestBuilder;
use rand::prelude::*;
use rand::rngs::OsRng;
use reqwest::Client;
use serde::Deserialize;
use std::{
    sync::mpsc::channel,
    thread,
    time::{Duration, Instant},
};
use tokio::sync::oneshot;

mod enneagram_prompts;

#[derive(Debug, Default)]
pub struct EnneagramData {
    pub enneagram_type: i64,
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

    pub async fn run_simulation(&self, config: Config) -> u64 {
        let mut users: Vec<User<EnneagramData>> = Vec::new();

        println!("config {}", config.neo_endpoint.unwrap());

        // generate random users based on the distribution, generate tendencies for each type
        // pick a random user and generate random content w references
        // generate responses to that content
        for d in self.distribution.iter() {
            let mut i: f64 = 0.0;

            while i < d * self.simulation_size as f64 {
                let user: User<EnneagramData> = User::default();

                users.push(user);

                i += 1.0;
            }
        }

        let rint = (rand::random::<f32>() * users.len() as f32).floor() as usize;

        let content = ChatRequestBuilder::new()
            .messages("Who was the oldest man to ever live?".to_string())
            .temperature(0.7)
            .max_tokens(100)
            .top_p(1.0)
            .presence_penalty(0.0)
            .frequency_penalty(0.0)
            .build()
            .send(config.open_ai_key.unwrap(), Client::new())
            .await;

        let mut debate_response = Response::default();
        debate_response.content = content.choices[0].message.content.clone();

        debate_response.create(config.neo4j_graph.unwrap()).await;

        println!("Response: {:?}", debate_response);

        0
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
