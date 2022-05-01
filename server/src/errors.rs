use argon2::password_hash::errors::Error as PwHashError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::error::DbErr;
use serde_json::json;

pub(crate) enum AppError {
    BadInput(&'static str),
    NoSuchUser,
    NoSuchProduct,
    NoSuchPurchase,
    NotEnoughStock,
    PurchaseAlreadyPaid,
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
            AppError::NoSuchProduct => (StatusCode::NOT_FOUND, "no such product"),
            AppError::NoSuchPurchase => (StatusCode::NOT_FOUND, "no such purchase"),
            AppError::NotEnoughStock => (StatusCode::CONFLICT, "not enough stock"),
            AppError::PurchaseAlreadyPaid => {
                (StatusCode::CONFLICT, "purchase has already been paid")
            }
            AppError::PwhError(PwHashError::Password) => {
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

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
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
