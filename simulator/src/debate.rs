use anyhow::Result;
use pulpcalc_common::models::response::Response;

pub struct Debate {
    id: String,

    last_score: i64,

    score: i64,

    topic: String,

    category: String,

    registered_speakers: i64,

    commenters: i64,

    voters: i64,

    comments: i64,

    inactive_participants: i64,

    responses: Vec<Response>,
}

impl Debate {
    pub fn calculate(&self, response: &Response) -> Result<i64> {
        todo!();
    }
}
