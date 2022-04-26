use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{extract, Extension};
use axum_extra::extract::cookie::CookieJar;
use sea_orm::{prelude::*, DatabaseConnection};

use entity::user;

use crate::{dtos::LoginDto, errors::AppError};

pub(crate) async fn login(
    extract::Json(login_dto): extract::Json<LoginDto>,
    Extension(ref conn): Extension<DatabaseConnection>,
    jar: CookieJar,
) -> Result<CookieJar, AppError> {
    let user = user::Entity::find()
        .filter(user::Column::PhoneNumber.contains(&login_dto.phone))
        .one(conn)
        .await?
        .ok_or(AppError::NoSuchUser)?;

    let password = dbg!(login_dto.password);

    let hashed_password = user.hashed_password;
    let password_hash =
        PasswordHash::new(&hashed_password).expect("saved password hash must be valid");
    Argon2::default().verify_password(password.as_bytes(), &password_hash)?;

    let user_cookie = crate::jwt_helpers::new_cookie(user.id)?;
    Ok(jar.add(user_cookie))
}
