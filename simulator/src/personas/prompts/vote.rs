use serde::Deserialize;

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
pub struct VoteResponse {
    pub vote: String,
    pub reason: String,
}
