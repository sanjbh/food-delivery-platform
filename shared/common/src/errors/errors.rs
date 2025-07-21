use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Not found: {0}")]
    NotfoundError(String),

    #[error("Unauthorized error: {0}")]
    UnauthorizedError(String),

    #[error("Forbidden error: {0}")]
    ForbiddenError(String),

    #[error("Conflict error: {0}")]
    ConflictError(String),

    #[error("Internal server error: {0}")]
    InternalError(String),

    #[error("External service error: {0}")]
    ExternalServiceError(String),
}
