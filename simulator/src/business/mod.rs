use pulpcalc_common::{
    config::Config,
    models::{Debate, Response, User},
};
use std::sync::Arc;

pub struct BusinessData {}

#[derive(Debug, Default)]
pub struct BusinessSimulation {
    pub simulation_type: String,

    pub simulation_size: u64,

    pub distribution: Vec<f64>,
}

impl BusinessSimulation {
    pub fn new(simulation_type: String, simulation_size: u64, distribution: Vec<f64>) -> Self {
        Self {
            simulation_type,
            simulation_size,
            distribution,
        }
    }

    pub fn simulation_type(&self) -> String {
        String::from("Business")
    }

    pub async fn run_simulation(&self, config: Config, mut debate: Debate) {}
}
