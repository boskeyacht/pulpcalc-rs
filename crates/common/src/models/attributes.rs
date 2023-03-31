#[derive(Debug, Clone, Default)]
pub struct Attributes {
    pub relevance: f64,

    pub soundness: f64,

    pub references: i64,

    pub word_count: i64,

    pub mastery_vocab_words: Option<Vec<String>>,
}

impl Attributes {
    pub fn new(
        relevance: f64,
        soundness: f64,
        references: i64,
        word_count: i64,
        mastery_vocab_words: Option<Vec<String>>,
    ) -> Self {
        Self {
            relevance,
            soundness,
            references,
            word_count,
            mastery_vocab_words,
        }
    }
}
