#[derive(Debug, Clone)]
pub enum VoteType {
    Valid(Option<String>),
    Invalid(Option<String>),
    Abstain(Option<String>),
}

impl Default for VoteType {
    fn default() -> Self {
        VoteType::Abstain(None)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Vote {
    /// The type of vote
    pub vote_type: VoteType,

    /// The id of the debate upon which the vote was cast
    pub debate_id: String,

    /// The id of the response upon which the vote was cast
    pub response_id: String,
}

impl Vote {
    pub fn new(vote_type: VoteType, debate_id: String, response_id: String) -> Self {
        Self {
            vote_type,
            debate_id,
            response_id,
        }
    }
}
