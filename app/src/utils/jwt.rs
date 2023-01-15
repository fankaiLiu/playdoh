use crate::apps::system::check_user_online;
use axum::{
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json, RequestPartsExt,
};
use chrono::{Duration, Local};
use db::{common::ctx::UserInfo, db_conn, DB};
use jsonwebtoken::{
    decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = &CFG.jwt.jwt_secret;
    Keys::new(secret.as_bytes())
});
use configs::CFG;
use tower_cookies::{Cookie, Cookies};

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthPayload {
    pub user_id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub id: String,
    pub token_id: String,
    pub name: String,
    pub exp: i64,
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;
    /// 将用户信息注入request
    async fn from_request_parts(req: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let token_v = get_cookie_token(req).await?;
        //获取 cookie
        let token_data = match decode::<Claims>(&token_v, &KEYS.decoding, &Validation::default()) {
            Ok(token) => {
                let token_id = token.claims.token_id.clone();
                let db = DB.get_or_init(db_conn).await;

                let (x, _) = check_user_online(db, token_id.clone()).await;
                print!("================================{}", token_id);
                print!("================================{}", x);
                if x {
                    token
                } else {
                    dbg!(1);
                    return Err(AuthError::CheckOutToken);
                }
            }
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => {
                    return Err(AuthError::InvalidToken);
                }
                ErrorKind::ExpiredSignature => {
                    return Err(AuthError::MissingCredentials);
                }
                _ => {
                    return Err(AuthError::WrongCredentials);
                }
            },
        };
        let user = token_data.claims;
        req.extensions.insert(UserInfo {
            id: user.id.clone(),
            token_id: user.token_id.clone(),
            name: user.name.clone(),
        });
        Ok(user)
    }
}

pub async fn get_cookie_token(parts: &mut Parts) -> Result<String, AuthError>
where
{
    let cookie = parts
        .extract::<Cookies>()
        .await
        .map_err(|_| AuthError::InvalidToken)?;
    let token_data = &cookie
        .get("token")
        .ok_or(AuthError::InvalidToken)?
        .to_string();

    Ok(token_data[6..].to_string())
}

pub async fn authorize(
    cookies: Cookies,
    payload: AuthPayload,
    token_id: String,
) -> Result<AuthBody, AuthError> {
    if payload.user_id.is_empty() || payload.name.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    let iat = Local::now();
    let exp = iat + Duration::minutes(CFG.jwt.jwt_exp);
    let claims = Claims {
        id: payload.user_id.to_owned(),
        token_id: token_id.clone(),
        name: payload.name,
        exp: exp.timestamp(),
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::WrongCredentials)?;
    dbg!(&token);
    // Send the authorized token
    // Build the cookie
    let mut cookie = Cookie::new("token", token.clone());
    cookie.set_http_only(true);
    cookies.add(cookie);
    Ok(AuthBody::new(token, claims.exp, CFG.jwt.jwt_exp, token_id))
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
    CheckOutToken,
}
impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
            AuthError::CheckOutToken => (StatusCode::UNAUTHORIZED, "该账户已经退出"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthBody {
    pub token: String,
    token_type: String,
    pub exp: i64,
    exp_in: i64,
}
impl AuthBody {
    fn new(access_token: String, exp: i64, exp_in: i64, token_id: String) -> Self {
        Self {
            token: access_token + &token_id,
            token_type: "Bearer".to_string(),
            exp,
            exp_in,
        }
    }
}
