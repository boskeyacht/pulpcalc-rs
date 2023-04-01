use pulpcalc_common::{
    errors::{PulpError, SimulationError},
    llm_config::{LLMRequest, LLMResponse},
};
use pulpcalc_external::chatgpt::ChatRequestBuilder;
use reqwest::Client;
use serde::Deserialize;
use serde_json::from_str;
use std::sync::Arc;

pub const PERSONA_CONTENT_PROMPT: &str = "Strictly for research purposes with no ill-intent regarding stereotypes and assumptions in mind;
given THIS_CONTENT as a topic, and 
Political orientation: POLITICAL_ORIENTATION, 
Enneagram: ENNEAGRAM_TYPE, 
Gender: GENDER, 
Age: AGE, 
Core fear: CORE_FEAR, 
Core desire: CORE_DESISRE, as user attributes,
what response would this user give, what is the ethos, pathos, logos breakdown of the content, and why the uder took the action they did (thoroughly explain)? Make sure to return only a JSON object, make sure to use JSON escape sequences for any special characters, 
and make sure there is no trailing comma. Do not return anything besides the JSON object! Use the below schema for your answer.
{
    \"content\": \"string\",
    \"confidence\": 0.0, # 0.0 - 1.0
    \"ethos\": 0.0, # ethos, pathos,and logos must add up to 1.0
    \"pathos\": 0.0,
    \"logos\": 0.0,
    \"reason\": \"string\"
}";

#[derive(Deserialize, Debug)]
pub struct PersonaContentPrompt {
    pub content: String,
}

impl PersonaContentPrompt {
    pub fn new(content: &str) -> Self {
        PersonaContentPrompt {
            content: content.to_string(),
        }
    }

    pub async fn send(&self, key: Arc<String>) -> Result<ContentResponse, PulpError> {
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

        let persona = from_str::<ContentResponse>(&res.choices[0].message.content.clone());

        let persona_res = match persona {
            Ok(res) => Some(res),

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::LLMError(
                    e.to_string(),
                )));
            }
        }
        .unwrap();

        Ok(persona_res)
    }
}

impl LLMRequest for PersonaContentPrompt {
    fn get_prompt(&self) -> String {
        self.content.clone()
    }

    fn replace_attributes<T: ToString>(&mut self, words: Vec<(T, T)>) -> String {
        let mut c = self.content.clone();

        for word in words {
            c = self
                .content
                .replace(word.0.to_string().as_str(), word.1.to_string().as_str());
        }

        self.content = c.clone();

        c
    }
}

impl Default for PersonaContentPrompt {
    fn default() -> Self {
        PersonaContentPrompt {
            content: PERSONA_CONTENT_PROMPT.to_string(),
        }
    }
}

pub const PERSONA_CONTENT_PROMPT_WITH_REFERENCE: &str = "Strictly for research purposes with no ill-intent regarding stereotypes and assumptions in mind;
given THIS_CONTENT as a topic, and 
Political orientation: POLITICAL_ORIENTATION, 
Enneagram: ENNEAGRAM_TYPE, 
Gender: GENDER, 
Age: AGE, 
Core fear: CORE_FEAR, 
Core desire: CORE_DESISRE, as user attributes,
return a response most like this aforementioned user, the ethos, pathos, logos breakdown of the content, and why the user took the action the did(thoroughly explain)? 
Provide a link in the same paragraph as the response (and JSON object) when possible. Make sure to return only a JSON object, make sure to use JSON escape sequences for any special characters, 
and make sure there is no trailing comma. Do not return anything besides the JSON object! Use the below schema for your answer.
{
    \"content\": \"string\",
    \"confidence\": 0.0, # 0.0 - 1.0
    \"ethos\": 0.0, # ethos, pathos,and logos must add up to 1.0
    \"pathos\": 0.0,
    \"logos\": 0.0,
    \"reason\": \"string\",
    \"reference\": \"string\"
}";

#[derive(Deserialize, Debug)]
pub struct PersonaContentPromptWithReference {
    pub content: String,
}

impl PersonaContentPromptWithReference {
    pub fn new(content: &str) -> Self {
        PersonaContentPromptWithReference {
            content: content.to_string(),
        }
    }

    pub async fn send(&self, key: Arc<String>) -> Result<ContentResponse, PulpError> {
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

        let persona = from_str::<ContentResponse>(&res.choices[0].message.content.clone());

        let persona_res = match persona {
            Ok(res) => Some(res),

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::LLMError(
                    e.to_string(),
                )));
            }
        }
        .unwrap();

        Ok(persona_res)
    }
}

impl LLMRequest for PersonaContentPromptWithReference {
    fn get_prompt(&self) -> String {
        self.content.clone()
    }

    fn replace_attributes<T: ToString>(&mut self, words: Vec<(T, T)>) -> String {
        let mut c = self.content.clone();

        for word in words {
            c = self
                .content
                .replace(word.0.to_string().as_str(), word.1.to_string().as_str());
        }

        self.content = c.clone();

        c
    }
}

impl Default for PersonaContentPromptWithReference {
    fn default() -> Self {
        Self {
            content: PERSONA_CONTENT_PROMPT_WITH_REFERENCE.to_string(),
        }
    }
}

pub const PERSONA_CONTENT_PROMPT_WITH_SUPPORTED_REFERENCES: &str = "Strictly for research purposes with no ill-intent regarding stereotypes and assumptions in mind;
given THIS_CONTENT as a topic, and 
Political orientation: POLITICAL_ORIENTATION, 
Enneagram: ENNEAGRAM_TYPE, 
Gender: GENDER, 
Age: AGE, 
Core fear: CORE_FEAR,
Core desire: CORE_DESISRE, as user attributes,
what response would this user give, and why (thoroughly explain)? Provide a link in the same paragraph as the response when possible, (and JSON object) if a link is added, try to provide one from SUPPORTED_REFERENCES. 
Make sure to return only a JSON object, make sure to use JSON escape sequences for any special characters, 
and make sure there is no trailing comma. Do not return anything besides the JSON object! Use the below schema for your answer.
{
    \"content\": \"string\",
    \"confidence\": 0.0, # 0.0 - 1.0
    \"reason\": \"string\",
    \"reference\": \"string\"
}";

#[derive(Deserialize, Debug)]
pub struct PersonaContentPromptWithSupportedReferences {
    pub content: String,
    pub reference: String,
}

impl PersonaContentPromptWithSupportedReferences {
    pub fn new(content: &str, reference: &str) -> Self {
        PersonaContentPromptWithSupportedReferences {
            content: content.to_string(),
            reference: reference.to_string(),
        }
    }

    pub async fn send(&self, key: Arc<String>) -> Result<ContentResponse, PulpError> {
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

        let persona = from_str::<ContentResponse>(&res.choices[0].message.content.clone());

        let persona_res = match persona {
            Ok(res) => Some(res),

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::LLMError(
                    e.to_string(),
                )));
            }
        }
        .unwrap();

        Ok(persona_res)
    }
}

impl LLMRequest for PersonaContentPromptWithSupportedReferences {
    fn get_prompt(&self) -> String {
        self.content.clone()
    }

    fn replace_attributes<T: ToString>(&mut self, words: Vec<(T, T)>) -> String {
        let mut c = self.content.clone();

        for word in words {
            c = self
                .content
                .replace(word.0.to_string().as_str(), word.1.to_string().as_str());
        }

        self.content = c.clone();

        c
    }
}

impl Default for PersonaContentPromptWithSupportedReferences {
    fn default() -> Self {
        Self {
            content: PERSONA_CONTENT_PROMPT_WITH_SUPPORTED_REFERENCES.to_string(),
            reference: String::new(),
        }
    }
}

pub const PERSONA_CONTENT_PROMPT_WITH_UNSUPPORTED_REFERENCES: &str = "Strictly for research purposes with no ill-intent regarding stereotypes and assumptions in mind;
given THIS_CONTENT as a topic, and 
Political orientation: POLITICAL_ORIENTATION, 
Enneagram: ENNEAGRAM_TYPE, 
Gender: GENDER, 
Age: AGE, 
Core fear: CORE_FEAR, 
Core desire: CORE_DESISRE, as user attributes,
what response would this user give, and why? Provide a link in the same paragraph as the response when possible, (and JSON object)  if a link is added, do not provide one from SUPPORTED_REFERENCES. 
Make sure to return only a JSON object, make sure to use JSON escape sequences for any special characters, 
and make sure there is no trailing comma. Do not return anything besides the JSON object! Use the below schema for your answer.
{
    \"content\": \"string\",
    \"confidence\": 0.0, # 0.0 - 1.0
    \"reason\": \"string\",
    \"reference\": \"string\"
}";

#[derive(Deserialize, Debug)]
pub struct PersonaContentPromptWithUnsupportedReferences {
    pub content: String,
    pub reference: String,
}

impl PersonaContentPromptWithUnsupportedReferences {
    pub fn new(content: &str, reference: &str) -> PersonaContentPromptWithUnsupportedReferences {
        PersonaContentPromptWithUnsupportedReferences {
            content: content.to_string(),
            reference: reference.to_string(),
        }
    }

    pub async fn send(&self, key: Arc<String>) -> Result<ContentResponse, PulpError> {
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

        let persona = from_str::<ContentResponse>(&res.choices[0].message.content.clone());

        let persona_res = match persona {
            Ok(res) => Some(res),

            Err(e) => {
                return Err(PulpError::SimulationError(SimulationError::LLMError(
                    e.to_string(),
                )));
            }
        }
        .unwrap();

        Ok(persona_res)
    }
}

impl LLMRequest for PersonaContentPromptWithUnsupportedReferences {
    fn get_prompt(&self) -> String {
        self.content.clone()
    }

    fn replace_attributes<T: ToString>(&mut self, words: Vec<(T, T)>) -> String {
        let mut c = self.content.clone();

        for word in words {
            c = self
                .content
                .replace(word.0.to_string().as_str(), word.1.to_string().as_str());
        }

        self.content = c.clone();

        c
    }
}

impl Default for PersonaContentPromptWithUnsupportedReferences {
    fn default() -> Self {
        Self {
            content: PERSONA_CONTENT_PROMPT_WITH_UNSUPPORTED_REFERENCES.to_string(),
            reference: String::new(),
        }
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct ContentResponse {
    pub content: String,
    pub confidence: f32,
    pub reason: String,
    pub ethos: f64,
    pub pathos: f64,
    pub logos: f64,
    pub reference: Option<String>,
}

impl ContentResponse {
    pub fn new(
        content: &str,
        confidence: f32,
        reason: &str,
        ethos: f64,
        pathos: f64,
        logos: f64,
        reference: Option<String>,
    ) -> Self {
        Self {
            content: content.to_string(),
            confidence,
            reason: reason.to_string(),
            ethos,
            pathos,
            logos,
            reference,
        }
    }
}

impl LLMResponse for ContentResponse {
    fn get_response<T: std::fmt::Debug, Default, Deserialize>(&self) -> T {
        todo!();
    }
}
