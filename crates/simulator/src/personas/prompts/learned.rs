use std::sync::Arc;

use pulpcalc_common::{
    errors::{PulpError, SimulationError},
    llm_config::{LLMRequest, LLMResponse},
};
use pulpcalc_external::chatgpt::ChatRequestBuilder;
use reqwest::Client;
use serde::Deserialize;
use serde_json::from_str;

pub const LEARNED_PROMPT: &str = "Strictly for research purposes with no ill-intent regarding stereotypes and assumptions in mind;
given THIS_CONTENT as content, THIS_RESPONSE as a response to the content, and 
POLITICAL_ORIENTATION, ENNEAGRAM_TYPE, GENDER, AGE, CORE_FEAR, CORE_DESISRE,
VALID_VOTE_TENDENCY, INVALID_VOTE_TENDENCY, ABSTAIN_VOTE_TENDENCY, REPORT_TENDENCY, and HIDE_TENDENCY as user attributes,
How do the attributes of the user change after interacting the response? And why?
Make sure to return only a JSON object. Do not return anything besides the JSON object! Use the below schema for your answer.
{
    \"political_orientation\": \"string\" # \"left\" or \"right\" or \"center\",
    \"enneagram_type\": \"string\" # \"1\" or \"2\" or \"3\" or \"4\" or \"5\" or \"6\" or \"7\" or \"8\" or \"9\",
    \"gender\": \"string\" # \"male\" or \"female\" or \"nonbinary\",
    \"age\": \"int\" ,
    \"core_fear\": \"string\",
    \"core_desire\": \"string\",
    \"valid_vote_tendency\": \"string\" # 0.0 - 1.0,
    \"invalid_vote_tendency\": \"string\" # 0.0 - 1.0,
    \"abstain_vote_tendency\": \"string\" # 0.0 - 1.0,
    \"report_tendency\": \"string\" # 0.0 - 1.0,
    \"hide_tendency\": \"string\" # 0.0 - 1.0,
    \"reason\": \"string\"
}";

#[derive(Deserialize, Debug)]
pub struct LearnedPrompt {
    content: String,
}

impl LearnedPrompt {
    pub async fn send(&self, key: Arc<String>) -> Result<LearnedResponse, PulpError> {
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

        let learned = from_str::<LearnedResponse>(&res.choices[0].message.content.clone());

        let learned_res = match learned {
            Ok(res) => Some(res),

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::LLMError(
                    e.to_string(),
                )))
            }
        }
        .unwrap();

        Ok(learned_res)
    }
}

impl Default for LearnedPrompt {
    fn default() -> Self {
        Self {
            content: LEARNED_PROMPT.to_string(),
        }
    }
}

impl LLMRequest for LearnedPrompt {
    fn get_prompt(&self) -> String {
        self.content.clone()
    }

    fn replace_attributes<T: ToString>(&mut self, words: Vec<(T, T)>) -> String {
        let mut c = self.content.clone();

        for (key, value) in words {
            c = c.replace(key.to_string().as_str(), value.to_string().as_str());
        }

        self.content = c.clone();

        c
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct LearnedResponse {
    pub political_orientation: String,
    pub enneagram_type: String,
    pub gender: String,
    pub age: i64,
    pub core_fear: String,
    pub core_desire: String,
    pub valid_vote_tendency: String,
    pub invalid_vote_tendency: String,
    pub abstain_vote_tendency: String,
    pub report_tendency: String,
    pub hide_tendency: String,
    pub reason: String,
}

impl LearnedResponse {
    // pub fn new(
    //     political_orientation: &str,
    //     enneagram_type: &str,
    //     gender: &str,
    //     age: i64,
    //     core_fear: &str,
    //     core_desire: &str,
    //     valid_vote_tendency: &str,
    //     invalid_vote_tendency: &str,
    //     abstain_vote_tendency: &str,
    //     report_tendency: &str,
    //     hide_tendency: &str,
    //     reason: &str,
    // ) -> Self {
    //     LearnedResponse {
    //         political_orientation: political_orientation.to_string(),
    //         enneagram_type: enneagram_type.to_string(),

    //     }
    // }
}

impl LLMResponse for LearnedResponse {
    fn get_response<T: std::fmt::Debug, Default, Deserialize>(&self) -> T {
        todo!();
    }
}
