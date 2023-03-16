mod cli;

use clap::Parser;
use futures::future::join_all;
use pulpcalc_common::{config::Config, models::Debate};
use simulator::new_enneagram_from_file;
use tokio::task;

#[tokio::main]
async fn main() {
    let args = cli::Cli::parse();

    match args.commands {
        Some(cli::PulpCommand::Sim(cmd)) => match cmd {
            // Simulate a debate with a random user distribution
            cli::SimCmd::Generate(args) => {
                println!(
                    "Generating a debate with {} users for {} seconds",
                    args.users, args.duration
                );
            }

            // Simulate a debate with a user pre-defined set of users (i.e. enneagram)
            cli::SimCmd::Enneagram(args) => {
                println!(
                    "Running a debate simulation with the config file {}",
                    args.file
                );

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
                println!(
                    "Running a business simulation with the config file {}",
                    args.file
                );
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
            println!("No command provided");
        }
    }
}
