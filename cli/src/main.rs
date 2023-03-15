mod cli;

use clap::Parser;
use futures::future::join_all;
use pulpcalc_common::{config::Config, simulation::SimulationType};
use pulpcalc_external::chatgpt::ChatRequestBuilder;
use reqwest::Client;
use serde_json::{json, Value};
use simulator::new_enneagram_from_file;
use tokio::task;

#[tokio::main]
async fn main() {
    let args = cli::Cli::parse();

    // let config = Config::init();

    // let cl = ChatRequestBuilder::new()
    //     .messages("Who was the oldest man to ever live?".to_string())
    //     .temperature(0.7)
    //     .max_tokens(100)
    //     .top_p(1.0)
    //     .presence_penalty(0.0)
    //     .frequency_penalty(0.0)
    //     .build();

    // println!("{:#?}", json!(cl));

    // let res = Client::new()
    //     .post("https://api.openai.com/v1/chat/completions")
    //     .json(&cl)
    //     .bearer_auth(config.open_ai_key.as_ref().unwrap().as_str())
    //     .send()
    //     .await
    //     .map_err(|e| e.to_string());

    // println!(
    //     "REQ Builder: {:#?}",
    //     res.unwrap().json::<Value>().await.unwrap()
    // );

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

                for sim in simulations {
                    let t = task::spawn(async move {
                        sim.run_simulation(Config::init()).await;
                    });

                    ts.push(t);
                }

                let scores = join_all(ts).await;

                println!("{:?}", scores);
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
