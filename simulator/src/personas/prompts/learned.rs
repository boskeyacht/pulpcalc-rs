use serde::Deserialize;

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
