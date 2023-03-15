use anyhow::Result;
use pulpcalc_common::models::response::Response;

pub struct Debate {
    pub id: String,

    pub last_score: i64,

    pub score: i64,

    pub topic: String,

    pub category: String,

    pub registered_speakers: i64,

    pub commenters: i64,

    pub voters: i64,

    pub comments: i64,

    pub inactive_participants: i64,

    pub responses: Vec<Response>,
}

impl Debate {
    pub fn calculate(&self, response: &Response) -> Result<i64> {
        todo!();
    }
}
