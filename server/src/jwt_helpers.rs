use axum_extra::extract::{cookie::Cookie, CookieJar};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use jwt::VerifyWithKey;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::errors::AppError;

#[derive(Serialize, Deserialize)]
struct Claim {
    sub: u32,
    exp: i64,
}

pub(crate) fn new_cookie(sub: u32) -> Result<Cookie<'static>, AppError> {
    let key: Hmac<Sha256> =
        // TODO: generate and persist hmac secret
        Hmac::new_from_slice(b"some-secret").expect("HMAC can take key of any size");
    let exp = (chrono::offset::Utc::now() + chrono::Duration::hours(24)).timestamp();
    let claim = Claim { sub, exp };
    let token_str = claim.sign_with_key(&key)?;
    Ok(Cookie::new("jwt", token_str).into_owned())
}

pub(crate) fn get_login(jar: &CookieJar) -> Result<u32, AppError> {
    let key: Hmac<Sha256> =
        // TODO: generate and persist hmac secret
        Hmac::new_from_slice(b"some-secret").expect("HMAC can take key of any size");
    let jwt_cookie = jar.get("jwt").ok_or(AppError::Unauthorized)?;
    let jwt_str = jwt_cookie.value();
    let claim: Claim = jwt_str.verify_with_key(&key)?;
    let now = chrono::offset::Utc::now().timestamp();
    if now < claim.exp {
        Ok(claim.sub)
    } else {
        Err(AppError::Unauthorized)
    }
}
