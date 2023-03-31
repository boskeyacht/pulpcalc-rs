use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum PulpError {
    #[error("{0}")]
    SimulationError(SimulationError),

    #[error("{0}")]
    ApiError(ApiError),
}

#[derive(Error, Debug, PartialEq)]
pub enum SimulationError {
    #[error("simulation llm error: {0}")]
    LLMError(String),

    #[error("simulation neo4j error: {0}")]
    Neo4jError(String),

    #[error("simulation error: {0}")]
    SimError(String),
}

#[derive(Error, Debug, PartialEq)]
pub enum ApiError {
    #[error("grpc error: {0}")]
    GRPCError(String),

    #[error("rest error: {0}")]
    RESTError(String),
}
