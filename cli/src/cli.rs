use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(
    name("pulpcalc"),
    about("A CLI utility for simulating debates"),
    long_about("A CLI utility for simulating debates."),
    version("0.1.0")
)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Option<PulpCommand>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum PulpCommand {
    /// Simualte a debatee
    #[command(subcommand)]
    Sim(SimCmd),

    /// Start a debate simulation server
    #[command(subcommand)]
    Serve(ServeCmd),
}

#[derive(Subcommand, Debug, Clone)]
pub enum SimCmd {
    /// Simulate a debate with various user sets
    #[command(name = "enneagram")]
    Enneagram(EnneagramArgs),

    #[command(name = "business")]
    Business(BusinessArgs),

    #[command(name = "personas")]
    Personas(PersonasArgs),
}

#[derive(Args, Debug, Clone)]
pub struct EnneagramArgs {
    /// The config file to use for the simulation
    #[arg(short, long)]
    pub file: String,
}

#[derive(Args, Debug, Clone)]
pub struct BusinessArgs {
    /// The config file used for the simulation
    #[arg(short, long)]
    pub file: String,
}

#[derive(Args, Debug, Clone)]
pub struct PersonasArgs {
    /// The config file used for the simulation
    #[arg(short, long)]
    pub file: String,

    /// Whether or not to initialize the neo4j database with a ser of users
    #[arg(short, long)]
    pub init: Option<i64>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ServeCmd {
    /// Start the gRPC server
    #[command(name = "grpc")]
    Grpc(GrpcArgs),

    /// Start the REST server
    #[command(name = "rest")]
    Rest(RestArgs),
}

#[derive(Args, Debug, Clone)]
pub struct GrpcArgs {
    /// The port to run the gRPC server on
    #[arg(short, long)]
    pub port: String,
}

#[derive(Args, Debug, Clone)]
pub struct RestArgs {
    /// The port to run the REST server on
    #[arg(short, long)]
    pub port: String,
}
