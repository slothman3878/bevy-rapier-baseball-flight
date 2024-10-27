use std::fmt;

#[derive(Debug)]
pub enum BaseballFlightError {
    InvalidInput(String),

    PhysicsCalculationError,

    ResourceNotFound,

    ConfigurationError,

    SimulationError,

    UnexpectedError,
}

pub(crate) type Result<T> = std::result::Result<T, BaseballFlightError>;

impl fmt::Display for BaseballFlightError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BaseballFlightError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            BaseballFlightError::PhysicsCalculationError => {
                write!(f, "Physics calculation error")
            }
            BaseballFlightError::ResourceNotFound => write!(f, "Resource not found"),
            BaseballFlightError::ConfigurationError => {
                write!(f, "Configuration error")
            }
            BaseballFlightError::SimulationError => write!(f, "Simulation error"),
            BaseballFlightError::UnexpectedError => {
                write!(f, "Unexpected error occurred")
            }
        }
    }
}
