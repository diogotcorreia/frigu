use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{extract, Extension, Json};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use sea_orm::{prelude::*, DatabaseConnection};

use entity::user;

use crate::{
    dtos::{LoginDto, UserDto},
    errors::AppError,
};

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

pub(crate) async fn user_info(
    Extension(ref conn): Extension<DatabaseConnection>,
    jar: CookieJar,
) -> Result<Json<UserDto>, AppError> {
    let user_id = crate::jwt_helpers::get_login(&jar)?;

    let user = user::Entity::find_by_id(user_id)
        .one(conn)
        .await?
        .ok_or(AppError::NoSuchUser)?;

    Ok(Json(UserDto::from_entity(user)?))
}

pub(crate) async fn logout(jar: CookieJar) -> Result<CookieJar, AppError> {
    Ok(jar.remove(Cookie::named("jwt")))
}
