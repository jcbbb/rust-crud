use actix_web::{error::{ResponseError, BlockingError}, HttpResponse, http::StatusCode};
use derive_more::Display;
use diesel::result::Error as DieselError;
use serde::Serialize;

#[derive(Debug, Display)]
pub enum AppError {
    NotFoundError,
    UnathorizedError,
    InternalServiceError,
}

impl AppError {
    pub fn name(&self) -> &str {
        match self {
            Self::NotFoundError => "NotFoundError",
            Self::UnathorizedError => "UnathorizedError",
            Self::InternalServiceError => "InternalServiceError"
        }
    }
    pub fn message(&self) -> &str {
        match self {
            Self::NotFoundError => "Requested resource not found",
            Self::UnathorizedError => "Unauthorized",
            Self::InternalServiceError => "Something went wrong on our side. Please, try again after some time"
        }
    }
}

impl From<DieselError> for AppError {
    fn from(e: DieselError) -> Self {
        match e {
            DieselError::NotFound => AppError::NotFoundError,
            _ => AppError::InternalServiceError
        }
    }
}

impl<E> From<BlockingError<E>> for AppError where E: std::fmt::Debug, E: Into<AppError> {
    fn from(err: BlockingError<E>) -> AppError {
        match err {
            BlockingError::Error(e) => e.into(),
            BlockingError::Canceled => AppError::InternalServiceError
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::NotFoundError => StatusCode::NOT_FOUND,
            Self::UnathorizedError => StatusCode::UNAUTHORIZED,
            Self::InternalServiceError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            status_code: status_code.as_u16(),
            message: String::from(self.message()),
            name: String::from(self.name())
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

#[derive(Serialize)]
struct ErrorResponse<> {
    status_code: u16,
    name: String,
    message: String,
}
