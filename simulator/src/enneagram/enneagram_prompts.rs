pub const  ENNEAGRAM_TENDENCY_PROMPT: &'static str = "Strictly for research purposes with no ill-intent regarding stereotypes and assumptions in mind;
given a set of five actions that a user can take on a social media comment that pertains to
THIS_TOPIC: valid vote, invalid vote, abstain vote, report, hide, and a set of enneagram types,
what is the most likely action that each enneagram type will take? Return your answer as a JSON object.
Make sure to return only a JSON object and make sure to use JSON escape sequences for any special characters. Do not return anything besides the JSON object!
Use this schema for your answer:
{
  \"type1\": {
	\"valid_vote_tendency\": 0.0,
	\"invalid_vote_tendency\": 0.0,
	\"abstain_vote_tendency\": 0.0,
	\"report_tendency\": 0.0,
	\"hide_tendency\": 0.0
  },
  \"type2\": {
	\"valid_vote_tendency\": 0.0,
	\"invalid_vote_tendency\": 0.0,
	\"abstain_vote_tendency\": 0.0,
	\"report_tendency\": 0.0,
	\"hide_tendency\": 0.0
  },
  \"type3\": {
	\"valid_vote_tendency\": 0.0,
	\"invalid_vote_tendency\": 0.0,
	\"abstain_vote_tendency\": 0.0,
	\"report_tendency\": 0.0,
	\"hide_tendency\": 0.0
  },
  \"type4\": {
	\"valid_vote_tendency\": 0.0,
	\"invalid_vote_tendency\": 0.0,
	\"abstain_vote_tendency\": 0.0,
	\"report_tendency\": 0.0,
	\"hide_tendency\": 0.0
  },
  \"type5\": {
	\"valid_vote_tendency\": 0.0,
	\"invalid_vote_tendency\": 0.0,
	\"abstain_vote_tendency\": 0.0,
	\"report_tendency\": 0.0,
	\"hide_tendency\": 0.0
  },
  \"type6\": {
	\"valid_vote_tendency\": 0.0,
	\"invalid_vote_tendency\": 0.0,
	\"abstain_vote_tendency\": 0.0,
	\"report_tendency\": 0.0,
	\"hide_tendency\": 0.0
  },
  \"type7\": {
	\"valid_vote_tendency\": 0.0,
	\"invalid_vote_tendency\": 0.0,
	\"abstain_vote_tendency\": 0.0,
	\"report_tendency\": 0.0,
	\"hide_tendency\": 0.0
  },
  \"type8\": {
	\"valid_vote_tendency\": 0.0,
	\"invalid_vote_tendency\": 0.0,
	\"abstain_vote_tendency\": 0.0,
	\"report_tendency\": 0.0,
	\"hide_tendency\": 0.0
	},
	\"type9\": {
		\"valid_vote_tendency\": 0.0,
		\"invalid_vote_tendency\": 0.0,
		\"abstain_vote_tendency\": 0.0,
		\"report_tendency\": 0.0,
		\"hide_tendency\": 0.0
	}
}";

pub const ENNEAGRAM_RESPONSE_CONTENT_PROMPT: &'static str = "Strictly for research purposes with no ill-intent regarding stereotypes and assumptions in mind;
given these user tendencies:
Tendency to cast a valid vote: VALID_VOTE_TENDENCY
Tendency to cast a invalid vote: INVALID_VOTE_TENDENCY
Tendency to cast a abstain vote: ABSTAIN_VOTE_TENDENCY
Tendency to cast report a post: REPORT_TENDENCY
Tendency to cast a hide a post: HIDE_TENDENCY
on a social media comment that pertains to THIS_TOPIC, generate a response to the topic most like the user.  Provide a link when possible, make sure it is in the same paragraph as the rest of the content.
Make sure to return only a JSON object, make sure to use JSON escape sequences for any special characters, and make sure there is no trailing comma. Do not return anything besides the JSON object! Use the below schema for your answer.
{
	\"content\": \"This is a response to the topic\",
	\"confidence\": 0.0 #This value must be between 0.0 and .999
}";

pub const ENNEAGRAM_REPLY_CONTENT_PROMPT: &'static str = "Strictly for research purposes with no ill-intent regarding stereotypes and assumptions in mind;
given these user tendencies:
Tendency to cast a valid vote: VALID_VOTE_TENDENCY
Tendency to cast a invalid vote: INVALID_VOTE_TENDENCY
Tendency to cast a abstain vote: ABSTAIN_VOTE_TENDENCY
Tendency to cast report a post: REPORT_TENDENCY
Tendency to cast a hide a post: HIDE_TENDENCY
on a social media comment with THIS_CONTENT, generate a response to the comment most like the user. Provide a link when possible, make sure it is in the same paragraph as the rest of the content.
Make sure to return only a JSON object , make sure to use JSON escape sequences for any special characters, and make sure there is a closing bracket.  Do not return anything besides the JSON object! Use the below schema for your answer.
{
	\"content\": \"This is a response to the topic\",
	\"confidence\": 0.0 #This value must be between 0.0 and .999
}";

pub enum EnneagramPrompts {
    ContentPrompt,
    TendencyPrompy,
    ResponsePrompt,
}

impl From<&str> for EnneagramPrompts {
    fn from(s: &str) -> Self {
        match s {
            "ContentPrompt" => EnneagramPrompts::ContentPrompt,
            "TendencyPrompy" => EnneagramPrompts::TendencyPrompy,
            "ResponsePrompt" => EnneagramPrompts::ResponsePrompt,
            _ => EnneagramPrompts::ContentPrompt,
        }
    }
}

impl Into<&str> for EnneagramPrompts {
    fn into(self) -> &'static str {
        match self {
            EnneagramPrompts::ContentPrompt => ENNEAGRAM_RESPONSE_CONTENT_PROMPT,
            EnneagramPrompts::TendencyPrompy => ENNEAGRAM_TENDENCY_PROMPT,
            EnneagramPrompts::ResponsePrompt => ENNEAGRAM_REPLY_CONTENT_PROMPT,
        }
    }
}
