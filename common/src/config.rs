use neo4rs::Graph;
use std::{env, sync::Arc};

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

    pub neo4j_graph: Arc<Graph>,
}

impl Config {
    pub async fn default() -> Self {
        let g = Graph::new("", "", "").await.unwrap();

        Self {
            reddit_app_id: None,
            reddit_secret_key: None,
            twitter_access_key: None,
            twitter_access_secret: None,
            twitter_api_key: None,
            twitter_api_secret: None,
            twitter_bearer_token: None,
            neo_endpoint: None,
            neo_user: None,
            neo_password: None,
            open_ai_key: None,
            neo4j_graph: Arc::new(g),
        }
    }

    pub async fn init() -> Self {
        let mut config = Config::default().await;

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
        config.open_ai_key = env::var("OPENAI_KEY").ok();

        let g = Graph::new(
            &config.neo_endpoint.clone().unwrap(),
            &config.neo_user.clone().unwrap(),
            &config.neo_password.clone().unwrap(),
        )
        .await;

        match g {
            Ok(g) => {
                config.neo4j_graph = Arc::new(g);
            }

            Err(e) => {
                println!("Error: {:#?}", e);
            }
        };

        config
    }
}
