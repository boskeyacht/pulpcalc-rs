use pulpcalc_common::llm_config::{LLMRequest, LLMResponse};
use serde::{Deserialize, Serialize};

pub const GENERATE_BLOG_PROMPT: &'static str = "Strictly for research purposes with no ill-intent regarding stereotypes and assumptions in mind;
Given THIS_TOPIC as a topic, generate a blog post about the topic similar to one that a company would publish on their blog.
Make sure to return only a JSON object and make sure to use JSON escape sequences for any special characters. Do not return anything besides the JSON object!
Use this schema for your answer:
{
  \"title\": \"\",
  \"body\": \"\"
}";

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GenerateBlogResponse {
    pub title: String,
    pub body: String,
}

pub const NAHCO_PROMPT: &'static str = "Does this topic: THIS_TOPIC, relate to any of the following? Entrepreneurship, Business, Marketing, Sales, Investing, Venture Capital, Startups, or Technology?
Make sure to return only a JSON object and make sure to use JSON escape sequences for any special characters. Do not return anything besides the JSON object!
Use this schema for your answer:
{
  \"answer\": \"true\" or \"false\"
}";

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NahcoResponse {
    pub answer: bool,
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

pub const PERSONA_CONTENT_PROMPT_WITH_NAHCO_REFERENCE: &str = "Strictly for research purposes with no ill-intent regarding stereotypes and assumptions in mind;
given THIS_CONTENT as a topic, and 
Political orientation: POLITICAL_ORIENTATION, 
Enneagram: ENNEAGRAM_TYPE, 
Gender: GENDER, 
Age: AGE, 
Core fear: CORE_FEAR, 
Core desire: CORE_DESISRE, as user attributes,
return a response most like this aforementioned user, the ethos, pathos, logos breakdown of the content, and why the user took the action the did(thoroughly explain)? 
Use THIS_LINK as a reference in your answer. Make sure to return only a JSON object, make sure to use JSON escape sequences for any special characters, 
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

impl LLMResponse for ContentResponse {
    fn get_response<T: std::fmt::Debug, Default, Deserialize>(&self) -> T {
        todo!();
    }
}
