use pulpcalc_common::simulation::Simulation;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct EnneagramSimulation {
    /// The simulation type (enneagram)
    pub simulation_type: String,

    /// The amount of users in the simulation
    pub simulation_size: i64,

    pub distribution: Vec<f64>,

    /// The depth of the simulation. The higher the number,
    /// the more replies will be created for any response
    pub depth: i64,

    /// The duration of the simulation
    pub simulation_duration: i64,

    /// The topic of the debate
    pub topic: String,

    /// The category of the dbeate
    pub category: String,
}

impl Simulation for EnneagramSimulation {
    fn simulation_type(&self) -> String {
        String::from("Enneagram")
    }

    fn run_simulation(&self) -> u64 {
        // Create channels for sending and receieving
        // let (one_tx, one_rx) = channel();

        // // Spawn one second timer
        // thread::spawn(move || loop {
        //     thread::sleep(Duration::from_secs(1));
        //     one_tx.send("tick").unwrap();
        // });

        // loop {
        //     thread::sleep(Duration::from_millis(50));
        //     let _ = one_rx.try_recv().map(|res| println!("{}", res));
        // }

        todo!();
    }
}

#[allow(dead_code)]
impl EnneagramSimulation {
    fn new(size: i64, depth: i64, duration: i64, topic: &str, cat: &str, dist: Vec<f64>) -> Self {
        let mut simulation = EnneagramSimulation::default();

        simulation.simulation_type = String::from("Enneagram");
        simulation.simulation_size = size;
        simulation.depth = depth;
        simulation.simulation_duration = duration;

        simulation
    }

    fn duration(&self) -> u64 {
        todo!()
    }

    fn size(&self) -> u64 {
        todo!()
    }

    fn depth(&self) -> u64 {
        todo!()
    }

    fn topic(&self) -> String {
        todo!()
    }

    fn category(&self) -> String {
        todo!()
    }

    fn distribution(&self) -> Vec<f64> {
        todo!()
    }
}

struct EnneagramSimulationBuilder {
    size: i64,
    depth: i64,
    duration: i64,
    topic: String,
    category: String,
    distribution: Vec<f64>,
}

impl EnneagramSimulationBuilder {
    fn new() -> Self {
        EnneagramSimulationBuilder {
            size: 0,
            depth: 0,
            duration: 0,
            topic: String::from(""),
            category: String::from(""),
            distribution: Vec::new(),
        }
    }

    fn size(mut self, size: i64) -> Self {
        self.size = size;
        self
    }

    fn depth(mut self, depth: i64) -> Self {
        self.depth = depth;
        self
    }

    fn duration(mut self, duration: i64) -> Self {
        self.duration = duration;
        self
    }

    fn topic(mut self, topic: String) -> Self {
        self.topic = topic;
        self
    }

    fn category(mut self, category: String) -> Self {
        self.category = category;
        self
    }

    fn distribution(mut self, distribution: Vec<f64>) -> Self {
        self.distribution = distribution;
        self
    }

    fn build(self) -> EnneagramSimulation {
        EnneagramSimulation::new(
            self.size,
            self.depth,
            self.duration,
            self.topic.as_str(),
            self.category.as_str(),
            self.distribution,
        )
    }
}
