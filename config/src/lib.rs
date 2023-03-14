use pulpcalc_common::simulation::{Simulation, SimulationType};
use simulator::enneagram::EnneagramSimulation;
use std::{env, fs};
use toml;

#[derive(Debug, Default)]
pub struct Config {
    /// Reddit app Id
    pub reddit_app_id: Option<String>,

    /// Reddit secret key
    pub reddit_secret_key: Option<String>,

    /// Twitter access key
    pub twitter_access_key: Option<String>,

    /// Twitter access secret
    pub twitter_access_secret: Option<String>,

    /// Twitter api key
    pub twitter_api_key: Option<String>,

    /// Twitter api secret
    pub twitter_api_secret: Option<String>,

    /// Twitter bearer token    
    pub twitter_bearer_token: Option<String>,

    /// Neo4j database endpoint
    pub neo_endpoint: Option<String>,

    /// Neo4j database user
    pub neo_user: Option<String>,

    /// Neo4j database password
    pub neo_password: Option<String>,

    pub open_ai_key: Option<String>,
}

impl Config {
    pub fn init() -> Self {
        let mut config = Config::default();

        config.reddit_app_id = env::var("REDDIT_APP_ID").ok();
        config.reddit_secret_key = env::var("REDDIT_SECRET_KEY").ok();
        config.twitter_access_key = env::var("TWITTER_ACCESS_KEY").ok();
        config.twitter_access_secret = env::var("TWITTER_ACCESS_SECRET").ok();
        config.twitter_api_key = env::var("TWITTER_API_KEY").ok();
        config.twitter_api_secret = env::var("TWITTER_API_SECRET").ok();
        config.twitter_bearer_token = env::var("TWITTER_BEARER_TOKEN").ok();
        config.neo_endpoint = env::var("NEO_ENDPOINT").ok();
        config.neo_user = env::var("NEO_USER").ok();
        config.neo_password = env::var("NEO_PASSWORD").ok();
        config.open_ai_key = env::var("OPEN_AI_KEY").ok();

        config
    }
}

pub fn new_simulation_from_file(file: &str, sim_type: SimulationType) -> Vec<Box<dyn Simulation>> {
    let contents = match fs::read_to_string(file) {
        Ok(c) => c,

        Err(e) => {
            println!("{}", e);

            std::process::exit(1);
        }
    };

    let mut sims: Vec<Box<dyn Simulation>> = vec![];

    match sim_type {
        SimulationType::Enneagram => {
            let data: EnneagramSimulation = match toml::from_str(&contents) {
                Ok(d) => d,

                Err(e) => {
                    println!("{}", e);

                    std::process::exit(1);
                }
            };

            sims.push(Box::new(EnneagramSimulation {
                simulation_type: String::from("Enneagram"),
                simulation_size: data.simulation_size,
                distribution: data.distribution,
                depth: data.depth,
                simulation_duration: data.simulation_duration,
                topic: data.topic,
                category: data.category,
            }) as Box<dyn Simulation>);
        }

        SimulationType::Age => println!("Age unimplemented"),
    };

    sims
}
