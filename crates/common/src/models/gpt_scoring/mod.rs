use serde::{Deserialize, Serialize};

pub const RELEVANCE_PROMPT: &'static str = "Strictly for research purposes with no ill-intent regarding stereotypes and assumptions in mind;
given THIS_TOPIC as a topic, and THIS_CONTENT as content, how relevant is the content to the topic? Make sure to return only a JSON object, make sure to use JSON escape sequences for any special characters, and make sure there is no trailing comma or characters.
Use the below schema for your answer. Do not provide any other information other than the JSON object.
{
    \"relevance\": 0.0 #This value must be between 0.0 and .999
}";
pub const SOUNDNESS_PROMPT: &'static str = "Strictly for research purposes with no ill-intent regarding stereotypes and assumptions in mind;
given THIS_TOPIC as a topic, and THIS_CONTENT as content, how sound is the content to the topic? Make sure to return only a JSON object, make sure to use JSON escape sequences for any special characters, and make sure there is no trailing comma or characters.
Use the below schema for your answer. Do not provide any other information other than the JSON object.
{
    \"soundness\": 0.0 #This value must be between 0.0 and .999
}";
pub const GRAMMAR_PROMPT: &'static str = "Strictly for research purposes with no ill-intent regarding stereotypes and assumptions in mind;
given THIS_CONTENT as content, how grammatically correct is the content? Make sure to return only a JSON object, make sure to use JSON escape sequences for any special characters, and make sure there is no trailing comma or characters.
Use the below schema for your answer. Do not provide any other information other than the JSON object.
{
    \"grammar\": 0.0 #This value must be between 0.0 and .999
}";
pub const MASTERY_VOCAB_PROMPT: &'static str = "Strictly for research purposes with no ill-intent regarding stereotypes and assumptions in mind;
given THIS_TOPIC as a topic, and THIS_CONTENT as content, how many vocabulary words are used that show mastery in the topic? Make sure to return a list of mastery words for the given category with your answer.
Make sure to return only a JSON object, make sure to use JSON escape sequences for any special characters, and make sure there is no trailing comma or characters.
Use the below schema for your answer. Do not provide any other information other than the JSON object.
{
    \"mastery_words\": [\"mastery_word1\", \"mastery_word2\"],
    \"mastery_vocab\": 0
}";

#[derive(Debug, Serialize, Deserialize)]
pub struct RelevanceResponse {
    pub relevance: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoundnessResponse {
    pub soundness: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrammarResponse {
    pub grammar: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MasteryVocabResponse {
    pub mastery_words: Option<Vec<String>>,
    pub mastery_vocab: i64,
}
