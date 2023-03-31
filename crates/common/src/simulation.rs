pub enum SimulationType {
    Enneagram,
    Age,
    Business,
}

impl From<&str> for SimulationType {
    fn from(s: &str) -> Self {
        match s {
            "enneagram" => SimulationType::Enneagram,
            "age" => SimulationType::Age,
            "business" => SimulationType::Business,
            _ => panic!("Invalid simulation type"),
        }
    }
}

pub enum Action {
    ValidVoteWithContent,
    InvalidVoteWithContent,
    ValidVote,
    InvalidVote,
    AbstainVote,
    Response,
    Report,
    Hide,
}

impl Action {
    pub fn base_point_value(&self) -> i32 {
        match self {
            Action::ValidVoteWithContent => 100,
            Action::InvalidVoteWithContent => 100,
            Action::ValidVote => 50,
            Action::InvalidVote => 50,
            Action::AbstainVote => 0,
            Action::Response => 150,
            Action::Report => 0,
            Action::Hide => 0,
        }
    }
}
