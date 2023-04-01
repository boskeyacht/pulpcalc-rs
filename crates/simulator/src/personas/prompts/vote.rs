use pulpcalc_common::{
    errors::{PulpError, SimulationError},
    llm_config::{LLMRequest, LLMResponse},
};
use pulpcalc_external::chatgpt::ChatRequestBuilder;
use reqwest::Client;
use serde::Deserialize;
use serde_json::from_str;
use std::sync::Arc;

pub const VOTE_CONTENT_PROMPT: &str = "Strictly for research purposes with no ill-intent regarding stereotypes and assumptions in mind;
given THIS_CONTENT as a topic, and 
Political orientation: POLITICAL_ORIENTATION, 
Enneagram: ENNEAGRAM_TYPE, 
Gender: GENDER, 
Age: AGE, 
Core fear: CORE_FEAR, 
Core desire: CORE_DESISRE, as user attributes,
what vote is the user most likely to cast, and why? Make sure to return only a JSON object, make sure to use JSON escape sequences for any special characters, 
and make sure there is no trailing comma. Do not return anything besides the JSON object! Use the below schema for your answer.
{
    \"vote\": \"valid\" or \"invalid\" or \"abstain\",
    \"reason\": \"string\"
}";

#[derive(Deserialize, Debug)]
pub struct VoteContentPrompt {
    content: String,
}

impl VoteContentPrompt {
    pub async fn send(&self, key: Arc<String>) -> Result<VoteResponse, PulpError> {
        let res = ChatRequestBuilder::new()
            .messages(self.content.clone())
            .temperature(0.7)
            .max_tokens(800)
            .top_p(1.0)
            .presence_penalty(0.0)
            .frequency_penalty(0.0)
            .build()
            .send(key.clone(), Client::new())
            .await;

        let vote = from_str::<VoteResponse>(&res.choices[0].message.content.clone());

        let vote_res = match vote {
            Ok(res) => Some(res),

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::LLMError(
                    e.to_string(),
                )));
            }
        }
        .unwrap();

        Ok(vote_res)
    }
}

impl Default for VoteContentPrompt {
    fn default() -> Self {
        Self {
            content: VOTE_CONTENT_PROMPT.to_string(),
        }
    }
}

impl LLMRequest for VoteContentPrompt {
    fn get_prompt(&self) -> String {
        self.content.clone()
    }

    fn replace_attributes<T: ToString>(&mut self, words: Vec<(T, T)>) -> String {
        let mut prompt = self.content.clone();

        for (key, value) in words {
            prompt = prompt.replace(key.to_string().as_str(), value.to_string().as_str());
        }

        self.content = prompt.clone();

        prompt
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct VoteResponse {
    pub vote: String,
    pub reason: String,
}

impl LLMResponse for VoteResponse {
    fn get_response<T: std::fmt::Debug, Default, Deserialize>(&self) -> T {
        todo!()
    }
}
