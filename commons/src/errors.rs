use log::error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Constraint violation: `{0}`")]
    ConstraintViolation(
        String,
        #[source] Box<dyn std::error::Error>,
        //std::backtrace::Backtrace
    ),
    #[error("Persistence error: `{0}`")]
    PersistenceError(String),
}

impl ServiceError {
    pub fn from_validation_error<E: std::error::Error + 'static>(
        e: E,
        backtrace: std::backtrace::Backtrace,
    ) -> Self {
        error!("{} {:?}", e.to_string(), backtrace);
        ServiceError::ConstraintViolation(e.to_string(), Box::new(e))
    }
}
