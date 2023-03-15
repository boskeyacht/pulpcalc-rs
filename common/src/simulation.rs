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
