use std::sync::{Arc, Mutex};

use crate::{
    errors::{PulpError, SimulationError},
    llm_config::{LLMRequest, LLMResponse},
};
use pulpcalc_external::chatgpt::ChatRequestBuilder;
use reqwest::Client;
use serde::Deserialize;
use serde_json::from_str;

pub const RELEVANCE_PROMPT: &'static str = "Strictly for research purposes with no ill-intent regarding stereotypes and assumptions in mind;
given THIS_TOPIC as a topic, and THIS_CONTENT as content, how relevant is the content to the topic? Make sure to return only a JSON object, make sure to use JSON escape sequences for any special characters, and make sure there is no trailing comma or characters.
Use the below schema for your answer. Do not provide any other information other than the JSON object.
{
    \"relevance\": 0.0 #This value must be between 0.0 and .999
}";

#[derive(Debug, Deserialize, Clone)]
pub struct RelevanceContentPrompt {
    pub content: String,
}

impl RelevanceContentPrompt {
    pub fn new(content: String) -> Self {
        Self { content }
    }

    pub async fn send(&self, key: Arc<String>) -> Result<RelevanceResponse, PulpError> {
        let res = ChatRequestBuilder::new()
            .messages(self.content.clone())
            .temperature(0.7)
            .max_tokens(800)
            .top_p(1.0)
            .presence_penalty(0.0)
            .frequency_penalty(0.0)
            .build()
            .send(key, Client::new())
            .await;

        let relevance = from_str::<RelevanceResponse>(&res.choices[0].message.content.clone());

        let relevance_res = match relevance {
            Ok(res) => Some(res),

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::LLMError(
                    e.to_string(),
                )));
            }
        }
        .unwrap();

        Ok(relevance_res)
    }
}

impl LLMRequest for RelevanceContentPrompt {
    fn get_prompt(&self) -> String {
        self.content.clone()
    }

    fn replace_attributes<T: ToString>(&mut self, words: Vec<(T, T)>) -> String {
        let mut new_content = self.content.clone();

        for (word, replacement) in words {
            new_content = new_content.replace(&word.to_string(), &replacement.to_string());
        }

        new_content
    }
}

impl Default for RelevanceContentPrompt {
    fn default() -> Self {
        Self {
            content: RELEVANCE_PROMPT.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct RelevanceResponse {
    pub relevance: f64,
}

impl LLMResponse for RelevanceResponse {
    fn get_response<T: std::fmt::Debug, Default, Deserialize>(&self) -> T {
        todo!();
    }
}

pub const SOUNDNESS_PROMPT: &'static str = "Strictly for research purposes with no ill-intent regarding stereotypes and assumptions in mind;
given THIS_TOPIC as a topic, and THIS_CONTENT as content, how sound is the content to the topic? Make sure to return only a JSON object, make sure to use JSON escape sequences for any special characters, and make sure there is no trailing comma or characters.
Use the below schema for your answer. Do not provide any other information other than the JSON object.
{
    \"soundness\": 0.0 #This value must be between 0.0 and .999
}";

#[derive(Debug, Deserialize, Clone)]
pub struct SoundnessContentPrompt {
    pub soundness: String,
}

impl SoundnessContentPrompt {
    pub fn new(soundness: String) -> Self {
        Self { soundness }
    }

    pub async fn send(&self, key: Arc<String>) -> Result<SoundnessResponse, PulpError> {
        let res = ChatRequestBuilder::new()
            .messages(self.soundness.clone())
            .temperature(0.7)
            .max_tokens(800)
            .top_p(1.0)
            .presence_penalty(0.0)
            .frequency_penalty(0.0)
            .build()
            .send(key, Client::new())
            .await;

        let soundness = from_str::<SoundnessResponse>(&res.choices[0].message.content.clone());

        let soundness_res = match soundness {
            Ok(res) => Some(res),

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::LLMError(
                    e.to_string(),
                )));
            }
        }
        .unwrap();

        Ok(soundness_res)
    }
}

impl LLMRequest for SoundnessContentPrompt {
    fn get_prompt(&self) -> String {
        self.soundness.clone()
    }

    fn replace_attributes<T: ToString>(&mut self, words: Vec<(T, T)>) -> String {
        let mut new_content = self.soundness.clone();

        for (word, replacement) in words {
            new_content = new_content.replace(&word.to_string(), &replacement.to_string());
        }

        new_content
    }
}

impl Default for SoundnessContentPrompt {
    fn default() -> Self {
        Self {
            soundness: SOUNDNESS_PROMPT.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct SoundnessResponse {
    pub soundness: f64,
}

impl LLMResponse for SoundnessResponse {
    fn get_response<T: std::fmt::Debug, Default, Deserialize>(&self) -> T {
        todo!();
    }
}

pub const GRAMMAR_PROMPT: &'static str = "Strictly for research purposes with no ill-intent regarding stereotypes and assumptions in mind;
given THIS_CONTENT as content, how grammatically correct is the content? Make sure to return only a JSON object, make sure to use JSON escape sequences for any special characters, and make sure there is no trailing comma or characters.
Use the below schema for your answer. Do not provide any other information other than the JSON object.
{
    \"grammar\": 0.0 #This value must be between 0.0 and .999
}";

#[derive(Debug, Deserialize, Clone)]
pub struct GrammarContentPrompt {
    pub grammar: String,
}

impl GrammarContentPrompt {
    pub fn new(grammar: String) -> Self {
        Self { grammar }
    }

    pub async fn send(&self, key: Arc<String>) -> Result<GrammarResponse, PulpError> {
        let res = ChatRequestBuilder::new()
            .messages(self.grammar.clone())
            .temperature(0.7)
            .max_tokens(800)
            .top_p(1.0)
            .presence_penalty(0.0)
            .frequency_penalty(0.0)
            .build()
            .send(key, Client::new())
            .await;

        let grammar = from_str::<GrammarResponse>(&res.choices[0].message.content.clone());

        let grammar_res = match grammar {
            Ok(res) => Some(res),

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::LLMError(
                    e.to_string(),
                )));
            }
        }
        .unwrap();

        Ok(grammar_res)
    }
}

impl LLMRequest for GrammarContentPrompt {
    fn get_prompt(&self) -> String {
        self.grammar.clone()
    }

    fn replace_attributes<T: ToString>(&mut self, words: Vec<(T, T)>) -> String {
        let mut new_content = self.grammar.clone();

        for (word, replacement) in words {
            new_content = new_content.replace(&word.to_string(), &replacement.to_string());
        }

        new_content
    }
}

impl Default for GrammarContentPrompt {
    fn default() -> Self {
        Self {
            grammar: GRAMMAR_PROMPT.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct GrammarResponse {
    pub grammar: f64,
}

impl LLMResponse for GrammarResponse {
    fn get_response<T: std::fmt::Debug, Default, Deserialize>(&self) -> T {
        todo!();
    }
}

pub const MASTERY_VOCAB_PROMPT: &'static str = "Strictly for research purposes with no ill-intent regarding stereotypes and assumptions in mind;
given THIS_TOPIC as a topic, and THIS_CONTENT as content, how many vocabulary words are used that show mastery in the topic? Make sure to return a list of mastery words for the given category with your answer.
Make sure to return only a JSON object, make sure to use JSON escape sequences for any special characters, and make sure there is no trailing comma or characters.
Use the below schema for your answer. Do not provide any other information other than the JSON object.
{
    \"mastery_words\": [\"mastery_word1\", \"mastery_word2\"],
    \"mastery_vocab\": 0
}";

#[derive(Debug, Deserialize, Clone)]
pub struct MasteryVocabContentPrompt {
    pub content: String,
}

impl MasteryVocabContentPrompt {
    pub fn new(content: String) -> Self {
        Self { content }
    }

    pub async fn send(&self, key: Arc<String>) -> Result<MasteryVocabResponse, PulpError> {
        let res = ChatRequestBuilder::new()
            .messages(self.content.clone())
            .temperature(0.7)
            .max_tokens(800)
            .top_p(1.0)
            .presence_penalty(0.0)
            .frequency_penalty(0.0)
            .build()
            .send(key, Client::new())
            .await;

        let mastery_vocab =
            from_str::<MasteryVocabResponse>(&res.choices[0].message.content.clone());

        let mastery_vocab_res = match mastery_vocab {
            Ok(res) => Some(res),

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::LLMError(
                    e.to_string(),
                )));
            }
        }
        .unwrap();

        Ok(mastery_vocab_res)
    }
}

impl LLMRequest for MasteryVocabContentPrompt {
    fn get_prompt(&self) -> String {
        self.content.clone()
    }

    fn replace_attributes<T: ToString>(&mut self, words: Vec<(T, T)>) -> String {
        let mut new_content = self.content.clone();

        for (word, replacement) in words {
            new_content = new_content.replace(&word.to_string(), &replacement.to_string());
        }

        new_content
    }
}

impl Default for MasteryVocabContentPrompt {
    fn default() -> Self {
        Self {
            content: MASTERY_VOCAB_PROMPT.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct MasteryVocabResponse {
    pub mastery_words: Option<Vec<String>>,
    pub mastery_vocab: i64,
}

impl LLMResponse for MasteryVocabResponse {
    fn get_response<T: std::fmt::Debug, Default, Deserialize>(&self) -> T {
        todo!();
    }
}
