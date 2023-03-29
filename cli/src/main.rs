mod cli;

use clap::Parser;
use eyre::Result;
use futures::future::join_all;
use pulpcalc_common::prelude::*;
use rand::prelude::*;
use simulator::{
    new_business_from_file, new_enneagram_from_file, new_personas_from_file,
    personas::{
        models::{Gender, PersonasUser},
        PersonasSimulation,
    },
};
use tokio::task;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::Cli::parse();

    match args.commands {
        Some(cli::PulpCommand::Sim(cmd)) => match cmd {
            // Simulate a debate with a user pre-defined set of users (i.e. enneagram, business, personas)
            cli::SimCmd::Enneagram(args) => {
                println!("Simulating a debate using enneagram types");

                let simulations = new_enneagram_from_file(&args.file);

                let mut ts = vec![];

                let debates: Vec<Debate> = Vec::new();

                for sim in simulations {
                    let mut d = Debate::default();
                    d.topic = sim.topic.clone();
                    d.category = sim.category.clone();

                    let t = task::spawn(async move {
                        sim.run_simulation(Config::init().await, d).await;
                    });

                    ts.push(t);
                }

                join_all(ts).await;

                for debate in debates {
                    println!("Debate: {:?}", debate);
                }
            }

            cli::SimCmd::Business(args) => {
                let simulations = new_business_from_file(args.file);

                let mut ts = vec![];

                let debates: Vec<Debate> = Vec::new();

                for sim in simulations {
                    let mut d = Debate::default();
                    d.topic = sim.topic.clone();
                    d.category = sim.category.clone();

                    let t = task::spawn(async move {
                        sim.run_simulation(Config::init().await, d).await;
                    });

                    ts.push(t);
                }

                join_all(ts).await;

                for debate in debates {
                    println!("Debate: {:?}", debate);
                }
            }

            cli::SimCmd::Personas(args) => {
                println!(
                    "Running a personas simulation with the config file {}",
                    args.file
                );

                let personas_sim = new_personas_from_file(args.file);
                let cfg = Config::init().await;

                if let Some(init_count) = args.init {
                    let mut users: Vec<PersonasUser> = Vec::new();

                    for _ in 0..init_count {
                        let mut user = PersonasUser::default();

                        if personas_sim.adults_only {
                            user.age = 18;
                        } else {
                            let rint_age = (random::<f32>()
                                * personas_sim.user_restrictions.clone().unwrap().max_age as f32)
                                .floor() as i64;
                            user.age = rint_age;
                        }

                        user.vote_valid_influence = personas_sim
                            .user_restrictions
                            .as_ref()
                            .unwrap()
                            .vote_valid_reason
                            .clone();
                        user.vote_invalid_influence = personas_sim
                            .user_restrictions
                            .as_ref()
                            .unwrap()
                            .vote_invalid_reason
                            .clone();
                        user.vote_abstain_influence = personas_sim
                            .user_restrictions
                            .as_ref()
                            .unwrap()
                            .vote_abstain_reason
                            .clone();

                        user.personality.personality_base.core_desire = "money".to_string();
                        user.personality.personality_base.core_fear = "death".to_string();

                        let rint_enneagram = (random::<f32>() * 9.0).floor() as i64;

                        user.personality.personality_base.enneagram = rint_enneagram;

                        users.push(user);
                    }

                    let mut j = 0.0;
                    let mut mdu: Vec<PersonasUser> = Vec::new();
                    while j < personas_sim
                        .user_restrictions
                        .as_ref()
                        .unwrap()
                        .male_distribution
                        * (personas_sim.simulation_size - 1) as f64
                    {
                        let mut user = users.pop().unwrap();
                        user.gender = Gender::from("male");

                        let user_id = user
                            .create(&cfg.neo4j_graph)
                            .await
                            .expect("failed to create user");
                        user.base_user.id = user_id;

                        mdu.push(user);

                        j += 1.0;
                    }

                    let mut z = 0.0;
                    let mut fdu: Vec<PersonasUser> = Vec::new();
                    while z
                        < (personas_sim
                            .user_restrictions
                            .as_ref()
                            .unwrap()
                            .female_distribution
                            * (personas_sim.simulation_size - 1) as f64)
                            .floor()
                    {
                        let mut user = users.pop().unwrap();
                        user.gender = Gender::from("female");

                        let user_id = user
                            .create(&cfg.neo4j_graph)
                            .await
                            .expect("failed to create user");
                        user.base_user.id = user_id;

                        fdu.push(user);

                        z += 1.0;
                    }

                    let mut k = 0.0;
                    let mut odu: Vec<PersonasUser> = Vec::new();
                    while k
                        < (personas_sim
                            .user_restrictions
                            .as_ref()
                            .unwrap()
                            .other_distribution
                            * (personas_sim.simulation_size - 1) as f64)
                            .floor()
                    {
                        let mut user = users.pop().unwrap();
                        user.gender = Gender::from("other");

                        let user_id = user
                            .create(&cfg.neo4j_graph)
                            .await
                            .expect("failed to create user");
                        user.base_user.id = user_id;

                        odu.push(user);

                        k += 1.0;
                    }

                    let mut l = 0.0;
                    let mut rnsdu: Vec<PersonasUser> = Vec::new();
                    while l < personas_sim
                        .user_restrictions
                        .as_ref()
                        .unwrap()
                        .not_saying_gender_distribution
                        * (personas_sim.simulation_size - 1) as f64
                    {
                        let mut user = users.pop().unwrap();
                        user.gender = Gender::from("rather not say");

                        let user_id = user
                            .create(&cfg.neo4j_graph)
                            .await
                            .expect("failed to create user");
                        user.base_user.id = user_id;

                        rnsdu.push(user);

                        l += 1.0;
                    }
                }

                let mut debates: Vec<Debate> = Vec::new();
                let tl = personas_sim.debate_topics.as_ref().unwrap();
                let cl = personas_sim.debate_categories.as_ref().unwrap();
                if personas_sim.exclusive_debate {
                    if cl.clone().len() == tl.clone().len() {
                        for i in 0..tl.len() {
                            let mut d = Debate::default();

                            d.topic = tl[i].clone();
                            d.category = cl[i].clone();

                            debates.push(d);
                        }
                    } else {
                        println!(
                            "Categories and topics must be the same length {} {}",
                            tl.len(),
                            cl.len()
                        );

                        return Ok(());
                    }
                } else {
                    todo!("Simulate an open debate")
                }

                let mut ps = PersonasSimulation::default();
                ps.debates = debates;

                ps.run_simulation(cfg, personas_sim).await;
            }
        },

        Some(cli::PulpCommand::Serve(cmd)) => match cmd {
            // Start the gRPC server
            cli::ServeCmd::Grpc(args) => {
                println!("Starting the gRPC server {}", args.port);
            }

            // Start the REST api
            cli::ServeCmd::Rest(args) => {
                println!("Starting the REST server {}", args.port)
            }
        },
        None => {
            println!("No command given");
        }
    };

    Ok(())
}
