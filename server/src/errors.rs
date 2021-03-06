use argon2::password_hash::errors::Error as PwHashError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::error::DbErr;

pub(crate) enum AppError {
    BadInput(&'static str),
    NoSuchUser,
    LoginError,
    DuplicateUser,
    NoSuchProduct,
    NoSuchPurchase,
    NotEnoughStock,
    PurchaseAlreadyPaid,
    BulkCountMismatch,
    Unauthorized,
    Forbidden,
    JwtError(jwt::error::Error),
    PwhError(PwHashError),
    DbError(DbErr),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::BadInput(message) => (StatusCode::BAD_REQUEST, message),
            AppError::NoSuchUser => (StatusCode::BAD_REQUEST, "no such user"),
            AppError::DuplicateUser => (StatusCode::CONFLICT, "that user already exists"),
            AppError::NoSuchProduct => (StatusCode::NOT_FOUND, "no such product"),
            AppError::NoSuchPurchase => (StatusCode::NOT_FOUND, "no such purchase"),
            AppError::NotEnoughStock => (StatusCode::CONFLICT, "not enough stock"),
            AppError::PurchaseAlreadyPaid => {
                (StatusCode::CONFLICT, "purchase has already been paid")
            }
            AppError::BulkCountMismatch => (
                StatusCode::CONFLICT,
                "affected count is different than expected",
            ),
            AppError::PwhError(PwHashError::Password) | AppError::LoginError => {
                (StatusCode::UNAUTHORIZED, "wrong password")
            }
            AppError::PwhError(_) | AppError::DbError(_) | AppError::JwtError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal servor error")
            }
            // TODO: how to clear jar **and** return StatusCode?
            // maybe UNAUTHORIZED redirects to login page?
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "login required"),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "not allowed to access this"),
        };

        (status, error_message).into_response()
    }
}

impl From<DbErr> for AppError {
    fn from(inner: DbErr) -> Self {
        Self::DbError(inner)
    }
}

impl From<PwHashError> for AppError {
    fn from(inner: PwHashError) -> Self {
        Self::PwhError(inner)
    }
}

impl From<jwt::error::Error> for AppError {
    fn from(inner: jwt::error::Error) -> Self {
        Self::JwtError(inner)
    }
}
