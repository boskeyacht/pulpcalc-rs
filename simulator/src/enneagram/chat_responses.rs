use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TendencyRespose {
    pub type1: ActionTendencies,
    pub type2: ActionTendencies,
    pub type3: ActionTendencies,
    pub type4: ActionTendencies,
    pub type5: ActionTendencies,
    pub type6: ActionTendencies,
    pub type7: ActionTendencies,
    pub type8: ActionTendencies,
    pub type9: ActionTendencies,
}

impl TendencyRespose {
    pub fn map_user_tendencies(&self, user_type: i64) -> ActionTendencies {
        match user_type {
            0 => self.type1.clone(),
            1 => self.type2.clone(),
            2 => self.type3.clone(),
            3 => self.type4.clone(),
            4 => self.type5.clone(),
            5 => self.type6.clone(),
            6 => self.type7.clone(),
            7 => self.type8.clone(),
            8 => self.type9.clone(),
            _ => self.type1.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ActionTendencies {
    pub valid_vote_tendency: f64,

    pub invalid_vote_tendency: f64,

    pub abstain_vote_tendency: f64,

    pub report_tendency: f64,

    pub hide_tendency: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContentReponse {
    pub confidence: f64,

    pub content: String,
}
