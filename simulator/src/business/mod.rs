use crate::business::prompts::GENERATE_BLOG_PROMPT;
use pulpcalc_common::{
    config::Config,
    models::{Debate, Response, User},
};
use pulpcalc_external::chatgpt::ChatRequestBuilder;
use reqwest::Client;
use serde::Deserialize;
use serde_json::from_str;

use self::prompts::GenerateBlogResponse;
mod prompts;

pub struct BusinessData {}

#[derive(Debug, Default, Clone)]
pub struct BusinessSimulation {
    pub simulation_type: String,

    pub simulation_size: u64,

    pub topic: String,

    pub category: String,

    pub distribution: Vec<f64>,

    pub source_url: String,
}

impl BusinessSimulation {
    pub fn new(
        simulation_type: String,
        simulation_size: u64,
        distribution: Vec<f64>,
        topic: String,
        cat: String,
        source_url: String,
    ) -> Self {
        Self {
            simulation_type,
            simulation_size,
            distribution,
            topic,
            category: cat,
            source_url,
        }
    }

    pub fn simulation_type(&self) -> String {
        String::from("Business")
    }

    pub async fn run_simulation(&self, config: Config, mut debate: Debate) {
        let key = config.open_ai_key.clone();

        let content = if !self.source_url.is_empty() {
            todo!("Scrape content from link")
        } else {
            let mut blog_prompt = GENERATE_BLOG_PROMPT.to_string();
            blog_prompt = blog_prompt.replace("THIS_TOPIC", &self.topic);

            let blog_chat_res = ChatRequestBuilder::new()
                .messages(blog_prompt)
                .temperature(0.7)
                .max_tokens(850)
                .top_p(1.0)
                .presence_penalty(0.0)
                .frequency_penalty(0.0)
                .build()
                .send(key.clone(), Client::new())
                .await;

            let content =
                from_str::<GenerateBlogResponse>(&blog_chat_res.choices[0].message.content.clone());

            let cont_res = match content {
                Ok(res) => Some(res),

                Err(e) => {
                    println!(
                        "failed to unmarshal content: {:?}: {}",
                        e, blog_chat_res.choices[0].message.content
                    );

                    None
                }
            }
            .unwrap();

            cont_res
        };

        println!("Content: {}", content.body);
    }
}
