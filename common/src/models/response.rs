#[derive(Debug, Default)]
pub struct Response {
    pub id: String,

    pub content: String,

    pub confidence: f64,

    pub score: i64,

    pub valid_vote_count: i64,

    pub invalid_vote_count: i64,

    pub abstain_vote_count: i64,

    pub author_id: String,

    pub replies: Vec<Response>,
}

impl Response {
    pub fn new(
        id: String,
        content: String,
        confidence: f64,
        score: i64,
        valid_vote_count: i64,
        invalid_vote_count: i64,
        abstain_vote_count: i64,
        author_id: String,
        replies: Vec<Response>,
    ) -> Self {
        Self {
            id,
            content,
            confidence,
            score,
            valid_vote_count,
            invalid_vote_count,
            abstain_vote_count,
            author_id,
            replies,
        }
    }

    pub fn generate_engagement(&self, response: &Self) {}
}
