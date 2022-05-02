use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use axum::{extract, Extension, Json};
use axum_client_ip::ClientIp;
use axum_extra::extract::cookie::{Cookie, CookieJar};
use sea_orm::{prelude::*, DatabaseConnection, Set};

use entity::user;

use crate::{
    dtos::{LoginDto, RegisterDto, UserDto},
    errors::AppError,
};

pub(crate) async fn login(
    extract::Json(login_dto): extract::Json<LoginDto>,
    Extension(ref conn): Extension<DatabaseConnection>,
    jar: CookieJar,
) -> Result<CookieJar, AppError> {
    let user = user::Entity::find()
        .filter(user::Column::PhoneNumber.eq(login_dto.phone))
        .one(conn)
        .await?
        .ok_or(AppError::LoginError)?;

    let password = login_dto.password;

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

pub(crate) async fn logout(jar: CookieJar) -> CookieJar {
    jar.remove(Cookie::named("jwt"))
}

pub(crate) async fn register(
    extract::Json(register_dto): extract::Json<RegisterDto>,
    ClientIp(ip): ClientIp,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<Json<UserDto>, AppError> {
    if !ip.is_loopback() {
        return Err(AppError::Forbidden);
    }

    let name = register_dto.name;
    if name.is_empty() {
        return Err(AppError::BadInput("name can't be empty"));
    }
    if name.len() > 30 {
        return Err(AppError::BadInput("name can't be longer than 30"));
    }

    let phone_number = register_dto.phone_number;
    if !(phone_number.len() == 9 && phone_number.chars().all(|c| c.is_digit(10))) {
        return Err(AppError::BadInput("phone number must be 9 digits long"));
    }

    let password = register_dto.password;
    if password.len() < 8 {
        return Err(AppError::BadInput("password must be at least 8 characters"));
    }

    let user = user::Entity::find()
        .filter(user::Column::PhoneNumber.eq(phone_number.clone()))
        .one(conn)
        .await?;
    if user.is_some() {
        return Err(AppError::DuplicateUser);
    }

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    let user = user::ActiveModel {
        name: Set(name.to_string()),
        phone_number: Set(phone_number),
        hashed_password: Set(password_hash),
        ..Default::default()
    };

    let user = user.insert(conn).await?;

    Ok(Json(UserDto::from_entity(user)?))
}
