use anyhow::Result;

struct Debate {
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
}

impl Debate {
    fn calculate(&self, response: i64) -> Result<i64> {
        todo!();
    }
}
