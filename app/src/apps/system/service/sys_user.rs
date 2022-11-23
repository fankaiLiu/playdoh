use crate::custom_response::ResultExt;
use crate::utils;
use crate::utils::jwt::AuthBody;
use crate::utils::jwt::AuthPayload;
use crate::Error;
use crate::Result;
use anyhow::Context;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash};
use axum::Json;

use hyper::HeaderMap;
use sqlx::types::Uuid;
use sqlx::{Pool, Postgres};
pub async fn create_user(db: &Pool<Postgres>, req: UserBody<NewUser>) -> Result<CreateUser> {
    let password_hash = hash_password(req.user.password).await?;

    // I personally prefer using queries inline in request handlers as it's easier to understand the
    // query's semantics in the wider context of where it's invoked.
    //
    // Sometimes queries just get too darn big, though. In that case it may be a good idea
    // to move the query to a separate module.
    let _user_id = sqlx::query_scalar!(
        // language=PostgreSQL
        r#"insert into "user" (username, email, password_hash) values ($1, $2, $3) returning user_id"#,
        req.user.username,
        req.user.email,
        password_hash
    )
    .fetch_one(db)
    .await
    .on_constraint("user_username_key", |_| {
        Error::unprocessable_entity([("username", "username taken")])
    })
    .on_constraint("user_email_key", |_| {
        Error::unprocessable_entity([("email", "email taken")])
    })?;

    Ok(CreateUser {
        email: req.user.email,
        // token: AuthUser { user_id }.to_jwt(&db),
        token: "token".to_string(),
        username: req.user.username,
        bio: "".to_string(),
        image: None,
    })
}

pub async fn login(
    db: &Pool<Postgres>,
    req: UserBody<LoginUser>,
    header: HeaderMap,
) -> Result<AuthBody> {
    let mut msg = "登录成功".to_string();
    let mut status = "1".to_string();
    let user = sqlx::query!(
        r#"
            select user_id, email, username, bio, image, password_hash 
            from "user" where email = $1
        "#,
        req.user.email,
    )
    .fetch_optional(db)
    .await?
    .ok_or(Error::unprocessable_entity([("email", "does not exist")]))?;

    verify_password(req.user.password, user.password_hash).await?;

    let claims = AuthPayload {
        user_id: user.user_id.to_string(),
        name: user.username.clone(),
    };
    let token_id = scru128::new_string();
    let token = utils::authorize(claims.clone(), token_id.clone())
        .await
        .unwrap();
    set_login_info(
        header,
        user.user_id.to_string(),
        user.username.clone(),
        msg.clone(),
        status.clone(),
        Some(token_id),
        Some(token.clone()),
    )
    .await;
    Ok(token)
}
pub async fn get_by_id(db: &Pool<Postgres>, u_id: &Uuid) -> Result<User> {
    let user = sqlx::query_as!(
        User,
        r#"
            select cast(user_id as varchar), email, username, bio, image 
            from "user" where user_id = $1
        "#,
        u_id.clone(),
    )
    .fetch_optional(db)
    .await?;
    Ok(user.unwrap())
}

async fn hash_password(password: String) -> Result<String> {
    // Argon2 hashing is designed to be computationally intensive,
    // so we need to do this on a blocking thread.
    tokio::task::spawn_blocking(move || -> Result<String> {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(
            PasswordHash::generate(Argon2::default(), password, salt.as_str())
                .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
                .to_string(),
        )
    })
    .await
    .context("panic in generating password hash")?
}

async fn verify_password(password: String, password_hash: String) -> Result<()> {
    tokio::task::spawn_blocking(move || -> Result<()> {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;

        hash.verify_password(&[&Argon2::default()], password)
            .map_err(|e| match e {
                argon2::password_hash::Error::Password => Error::Unauthorized,
                _ => anyhow::anyhow!("failed to verify password hash: {}", e).into(),
            })
    })
    .await
    .context("panic in verifying password hash")?
}

pub async fn set_login_info(
    header: HeaderMap,
    u_id: String,
    user: String,
    msg: String,
    status: String,
    token_id: Option<String>,
    token: Option<AuthBody>,
) {
    let u = utils::get_client_info(header).await;
    // 写入登录日志
    let u2 = u.clone();
    let status2 = status.clone();
    // 如果成功，写入在线日志
    if status == "1" {
        if let (Some(token_id), Some(token)) = (token_id, token) {
            super::sys_user_online::add(u, u_id, token_id, token.clone().exp).await;
        }
    };
    // tokio::spawn(async move {
    //     super::sys_login_log::add(u2, user, msg, status2).await;
    // });
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserBody<T> {
    user: T,
}

#[derive(serde::Deserialize)]
pub struct NewUser {
    username: String,
    email: String,
    password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateUser {
    email: String,
    token: String,
    username: String,
    bio: String,
    image: Option<String>,
}
#[derive(serde::Deserialize)]
pub struct User {
    user_id: Option<String>,
    pub email: String,
    pub username: String,
    bio: String,
    image: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct LoginUser {
    email: String,
    password: String,
}
