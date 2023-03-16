#[derive(Debug, Clone, Default)]
pub struct Engagements {
    pub report_harmful_to_others: i64,

    pub report_abuseof_platform: i64,

    pub hide: i64,

    pub vote_validity: i64,

    pub vote_condfidence: i64,

    pub response_distance: i64,

    pub response_timing: i64,
}

impl Engagements {
    pub fn new(
        report_harmful_to_others: i64,
        report_abuseof_platform: i64,
        hide: i64,
        vote_validity: i64,
        vote_condfidence: i64,
        response_distance: i64,
        response_timing: i64,
    ) -> Self {
        Self {
            report_harmful_to_others,
            report_abuseof_platform,
            hide,
            vote_validity,
            vote_condfidence,
            response_distance,
            response_timing,
        }
    }
}
