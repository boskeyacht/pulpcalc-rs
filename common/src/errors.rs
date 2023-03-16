pub enum PulpError {
    SimulationError(SimulationError),
    ApiError(ApiError),
}

pub enum ApiError {
    UnmarshalErorr(String),
}

pub enum SimulationError {
    UnmarshalError(String),
}
