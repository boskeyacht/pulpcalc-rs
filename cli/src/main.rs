mod cli;

use clap::Parser;
use config::{new_simulation_from_file, Config};
use pulpcalc_common::simulation::SimulationType;

#[tokio::main]
async fn main() {
    let args = cli::Cli::parse();

    match args.commands {
        Some(cli::PulpCommand::Sim(cmd)) => match cmd {
            cli::SimCmd::Generate(args) => {
                println!(
                    "Generating a debate with {} users for {} seconds",
                    args.users, args.duration
                );
            }
            cli::SimCmd::Sets(args) => {
                println!(
                    "Running a debate simulation with the config file {}",
                    args.file
                );

                let _config = Config::init();

                let simulations = new_simulation_from_file(
                    &args.file,
                    SimulationType::from(args.simulation_type.as_str()),
                );

                for sim in simulations {
                    sim.run_simulation();
                }
            }
        },
        Some(cli::PulpCommand::Serve(cmd)) => match cmd {
            cli::ServeCmd::Grpc(args) => {
                println!("Starting the gRPC server {}", args.port);
            }

            cli::ServeCmd::Rest(args) => {
                println!("Starting the REST server {}", args.port)
            }
        },
        None => {
            println!("No command provided");
        }
    }
}
