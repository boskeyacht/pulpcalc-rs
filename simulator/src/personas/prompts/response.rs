use serde::Deserialize;

pub const PERSONA_CONTENT_PROMPT: &str = "Strictly for research purposes with no ill-intent regarding stereotypes and assumptions in mind;
given THIS_CONTENT as a topic, and 
Political orientation: POLITICAL_ORIENTATION, 
Enneagram: ENNEAGRAM_TYPE, 
Gender: GENDER, 
Age: AGE, 
Core fear: CORE_FEAR, 
Core desire: CORE_DESISRE, as user attributes,
what response would this user give, and why (thoroughly explain)? Make sure to return only a JSON object, make sure to use JSON escape sequences for any special characters, 
and make sure there is no trailing comma. Do not return anything besides the JSON object! Use the below schema for your answer.
{
    \"content\": \"string\",
    \"confidence\": 0.0, # 0.0 - 1.0
    \"reason\": \"string\"
}";

pub const PERSONA_CONTENT_PROMPT_WITH_REFERENCE: &str = "Strictly for research purposes with no ill-intent regarding stereotypes and assumptions in mind;
given THIS_CONTENT as a topic, and 
Political orientation: POLITICAL_ORIENTATION, 
Enneagram: ENNEAGRAM_TYPE, 
Gender: GENDER, 
Age: AGE, 
Core fear: CORE_FEAR, 
Core desire: CORE_DESISRE, as user attributes,
return a response most like this aforementioned user, and why you think the user would take that action (thoroughly explain)? 
Provide a link in the same paragraph as the response (and JSON object) when possible. Make sure to return only a JSON object, make sure to use JSON escape sequences for any special characters, 
and make sure there is no trailing comma. Do not return anything besides the JSON object! Use the below schema for your answer.
{
    \"content\": \"string\",
    \"confidence\": 0.0, # 0.0 - 1.0
    \"reason\": \"string\",
    \"reference\": \"string\"
}";

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

#[derive(Deserialize, Debug, Default)]
pub struct ContentResponse {
    pub content: String,
    pub confidence: f32,
    pub reason: String,
    pub reference: Option<String>,
}
