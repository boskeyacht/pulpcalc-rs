use crate::business::prompts::*;
use eyre::Result;
use pulpcalc_common::{
    config::Config,
    errors::{PulpError, SimulationError},
    models::{Debate, Reference, Response, User},
};
use pulpcalc_external::chatgpt::ChatRequestBuilder;
use reqwest::Client;
use serde::Deserialize;
use serde_json::from_str;
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

    pub async fn run_simulation(
        &self,
        config: Config,
        mut debate: Debate,
    ) -> Result<(), PulpError> {
        let key = config.open_ai_key.clone();

        let debate_id = debate.create(&config.neo4j_graph).await?;
        debate.id = debate_id;

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

            cont_res;
        };

        let mut nahco_prompt = NAHCO_PROMPT.to_string();
        nahco_prompt = nahco_prompt.replace("THIS_TOPIC", &self.topic);

        let nahco_chat_res = ChatRequestBuilder::new()
            .messages(nahco_prompt)
            .temperature(0.7)
            .max_tokens(850)
            .top_p(1.0)
            .presence_penalty(0.0)
            .frequency_penalty(0.0)
            .build()
            .send(key.clone(), Client::new())
            .await;

        let content = from_str::<NahcoResponse>(&nahco_chat_res.choices[0].message.content.clone());

        let nahco_res = match content {
            Ok(res) => Some(res),

            Err(e) => {
                println!(
                    "failed to unmarshal content: {:?}: {}",
                    e, nahco_chat_res.choices[0].message.content
                );

                None
            }
        }
        .unwrap();

        if nahco_res.answer {
            let mut nahco_reference_prompt =
                PERSONA_CONTENT_PROMPT_WITH_NAHCO_REFERENCE.to_string();
            nahco_reference_prompt = nahco_reference_prompt.replace("THIS_TOPIC", &self.topic);

            let nahco_reference_chat_res = ChatRequestBuilder::new()
                .messages(nahco_reference_prompt)
                .temperature(0.7)
                .max_tokens(850)
                .top_p(1.0)
                .presence_penalty(0.0)
                .frequency_penalty(0.0)
                .build()
                .send(key.clone(), Client::new())
                .await;

            let content = from_str::<ContentResponse>(
                &nahco_reference_chat_res.choices[0].message.content.clone(),
            );

            let nahco_reference_res = match content {
                Ok(res) => Some(res),

                Err(e) => {
                    println!(
                        "failed to unmarshal content: {:?}: {}",
                        e, nahco_reference_chat_res.choices[0].message.content
                    );

                    None
                }
            }
            .unwrap();

            let mut response = Response::default();
            response.content = nahco_reference_res.content;
            response.ethos = nahco_reference_res.ethos;
            response.pathos = nahco_reference_res.pathos;
            response.logos = nahco_reference_res.logos;

            let response_id = response.create(&config.neo4j_graph).await?;
            response.id = response_id;

            let mut reff = Reference::default();
            reff.content = nahco_reference_res.reference.unwrap_or_default();
            reff.internal = false;

            let reff_id = reff.create(&config.neo4j_graph).await?;
            reff.id = reff_id;

            reff.add_response_referenced_relationship(&config.neo4j_graph, response.clone())
                .await;
        }

        Ok(())
    }
}
