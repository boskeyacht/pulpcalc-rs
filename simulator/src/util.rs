use crate::enneagram::EnneagramSimulation;
use std::fs;
use toml;

pub fn new_enneagram_from_file(file: &str) -> Vec<EnneagramSimulation> {
    let contents = match fs::read_to_string(file) {
        Ok(c) => c,

        Err(e) => {
            println!("{}", e);

            std::process::exit(1);
        }
    };

    let mut sims: Vec<EnneagramSimulation> = Vec::new();

    let data: EnneagramSimulation = match toml::from_str(&contents) {
        Ok(d) => d,

        Err(e) => {
            println!("{}", e);

            std::process::exit(1);
        }
    };

    sims.push(EnneagramSimulation {
        simulation_type: String::from("Enneagram"),
        simulation_size: data.simulation_size,
        distribution: data.distribution,
        depth: data.depth,
        simulation_duration: data.simulation_duration,
        topic: data.topic,
        category: data.category,
    });

    sims
}
