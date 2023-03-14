pub enum SimulationType {
    Enneagram,
    Age,
}

impl From<&str> for SimulationType {
    fn from(s: &str) -> Self {
        match s {
            "enneagram" => SimulationType::Enneagram,
            "age" => SimulationType::Age,
            _ => panic!("Invalid simulation type"),
        }
    }
}

pub trait Simulation {
    fn simulation_type(&self) -> String;

    // fn duration(&self) -> u64;

    // fn size(&self) -> u64;

    // fn depth(&self) -> u64;

    fn run_simulation(&self) -> u64;
}
