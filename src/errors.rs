use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use std::fmt;
use validator::ValidationErrors;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    ValidationError(ValidationErrors),
    DatabaseError(sqlx::Error),
    NotFound(String),
    BadRequest(String),
    InternalServerError(String),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<String>>,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ValidationError(_) => write!(f, "Validation error"),
            AppError::DatabaseError(e) => write!(f, "Database error: {}", e),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::DatabaseError(e) => match e {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                sqlx::Error::Database(db_err) => {
                    // Check for specific database errors
                    if let Some(code) = db_err.code() {
                        match code.as_ref() {
                            "23505" => StatusCode::CONFLICT,    // unique_violation
                            "23503" => StatusCode::BAD_REQUEST, // foreign_key_violation
                            "23502" => StatusCode::BAD_REQUEST, // not_null_violation
                            _ => StatusCode::INTERNAL_SERVER_ERROR,
                        }
                    } else {
                        StatusCode::INTERNAL_SERVER_ERROR
                    }
                }
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();

        let error_response = match self {
            AppError::ValidationError(errors) => {
                let details: Vec<String> = errors
                    .field_errors()
                    .iter()
                    .flat_map(|(field, errors)| {
                        errors.iter().map(move |error| {
                            format!(
                                "{}: {}",
                                field,
                                error.message.as_ref().unwrap_or(&"Invalid value".into())
                            )
                        })
                    })
                    .collect();

                log::warn!("⚠️ Validation failed: {:?}", details);

                ErrorResponse {
                    error: "Validation failed".to_string(),
                    details: Some(details),
                }
            }
            AppError::DatabaseError(e) => {
                log::error!("❌ Database error: {}", e);

                // Don't expose internal database errors to clients
                let user_message = match e {
                    sqlx::Error::RowNotFound => "Resource not found",
                    sqlx::Error::Database(db_err) => {
                        if let Some(code) = db_err.code() {
                            match code.as_ref() {
                                "23505" => "A record with this information already exists",
                                "23503" => "Referenced resource does not exist",
                                "23502" => "Required field is missing",
                                _ => "Database operation failed",
                            }
                        } else {
                            "Database operation failed"
                        }
                    }
                    _ => "Database operation failed",
                };

                ErrorResponse {
                    error: user_message.to_string(),
                    details: None,
                }
            }
            AppError::NotFound(msg) => {
                log::error!("Not found: {}", msg);
                ErrorResponse {
                    error: msg.clone(),
                    details: None,
                }
            }
            AppError::BadRequest(msg) => {
                log::error!("Bad request: {}", msg);
                ErrorResponse {
                    error: msg.clone(),
                    details: None,
                }
            }
            AppError::InternalServerError(msg) => {
                log::error!("☠️ Internal error: {}", msg);
                ErrorResponse {
                    error: "An internal error occurred".to_string(),
                    details: None,
                }
            }
        };

        HttpResponse::build(status_code).json(error_response)
    }
}

// Automatic conversion from sqlx::Error to AppError
impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        AppError::DatabaseError(error)
    }
}

// Automatic conversion from ValidationErrors to AppError
impl From<ValidationErrors> for AppError {
    fn from(errors: ValidationErrors) -> Self {
        AppError::ValidationError(errors)
    }
}
