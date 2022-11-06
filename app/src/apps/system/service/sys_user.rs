use crate::custom_response::ResultExt;
use crate::Error;
use crate::Result;
use anyhow::Context;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash};
use axum::{
    Json,
};
use sqlx::{Pool, Postgres};
pub async fn create_user(
    db: &Pool<Postgres>,
    req: UserBody<NewUser>,
) -> Result<Json<UserBody<User>>> {
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

    Ok(Json(UserBody {
        user: User {
            email: req.user.email,
            // token: AuthUser { user_id }.to_jwt(&db),
            token: "token".to_string(),
            username: req.user.username,
            bio: "".to_string(),
            image: None,
        },
    }))
}

pub async fn login_user(
    db: &Pool<Postgres>,
    req: UserBody<LoginUser>,
) -> Result<Json<UserBody<User>>> {
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

    Ok(Json(UserBody {
        user: User {
            email: user.email,
            token: "token".to_string(),
            username: user.username,
            bio: user.bio,
            image: user.image,
        },
    }))
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
pub struct User {
    email: String,
    token: String,
    username: String,
    bio: String,
    image: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct LoginUser {
    email: String,
    password: String,
}
