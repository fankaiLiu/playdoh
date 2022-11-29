use crate::custom_response::ResultExt;
use crate::pagination::Pagination;
use crate::request_query::Page;
use crate::request_query::PageTurnReq;
use crate::request_query::PageTurnResponse;
use crate::utils;
use crate::utils::jwt::AuthBody;
use crate::utils::jwt::AuthPayload;
use crate::Error;
use crate::Result;
use anyhow::Context;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash};
use async_trait::async_trait;

use db::DB;
use db::db_conn;
use hyper::HeaderMap;
use serde::Serialize;
use sqlx::types::Uuid;
use sqlx::{Pool, Postgres};
pub async fn create_user(db: &Pool<Postgres>, req: NewUser) -> Result<CreateUser> {
    let password_hash = hash_password(req.password).await?;

    // I personally prefer using queries inline in request handlers as it's easier to understand the
    // query's semantics in the wider context of where it's invoked.
    //
    // Sometimes queries just get too darn big, though. In that case it may be a good idea
    // to move the query to a separate module.
    let _user_id = sqlx::query_scalar!(
        // language=PostgreSQL
        r#"insert into "user" (username, email, password_hash) values ($1, $2, $3) returning user_id"#,
        req.username,
        req.email,
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
        email: req.email,
        // token: AuthUser { user_id }.to_jwt(&db),
        token: "token".to_string(),
        username: req.username,
        bio: "".to_string(),
        image: None,
    })
}

pub async fn update_user(db: &Pool<Postgres>, req: UpdateUser) -> Result<String> {
    let user_id = Uuid::parse_str(&req.id)?;
    let user_id = sqlx::query_scalar!(
        // language=PostgreSQL
        r#"update "user" set username = $1, email = $2, bio = $3, image = $4 where user_id = $5 returning user_id"#,
        req.username,
        req.email,
        req.bio,
        req.image,
        user_id
    )
    .fetch_one(db)
    .await
    .on_constraint("user_username_key", |_| {
        Error::unprocessable_entity([("username", "username taken")])
    })
    .on_constraint("user_email_key", |_| {
        Error::unprocessable_entity([("email", "email taken")])
    })?;
    Ok("ok".to_string())
}

pub async fn delete(db: &Pool<Postgres>, id: String) -> Result<String> {
    let user_id = Uuid::parse_str(&id)?;
    let _user_id = sqlx::query_scalar!(
        // language=PostgreSQL
        r#"delete from "user" where user_id = $1 returning user_id"#,
        user_id
    )
    .fetch_one(db)
    .await?;
    Ok("ok".to_string())
}

pub async fn login(db: &Pool<Postgres>, req: LoginUser, header: HeaderMap) -> Result<AuthBody> {
    let msg = "登录成功".to_string();
    let status = "1".to_string();
    let user = sqlx::query!(
        r#"
            select user_id, email, username, bio, image, password_hash 
            from "user" where email = $1
        "#,
        req.email,
    )
    .fetch_optional(db)
    .await?
    .ok_or(Error::unprocessable_entity([("email", "does not exist")]))?;

    verify_password(req.password, user.password_hash).await?;

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
    _user: String,
    _msg: String,
    status: String,
    token_id: Option<String>,
    token: Option<AuthBody>,
) {
    let u = utils::get_client_info(header).await;
    // 写入登录日志
    let _u2 = u.clone();
    let _status2 = status.clone();
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
#[derive(serde::Deserialize)]
pub struct SearchResult {
    pub username: String,
    pub email: String,
}
#[derive(serde::Deserialize)]
pub struct OrdersRequest {}
pub struct UserPageClient {}

impl UserPageClient {
    pub fn new() -> Self {
        Self {}
    }
}

pub type UserRequest = PageTurnReq<SearchResult, OrdersRequest>;
pub type UserPageResponse = PageTurnResponse<User>;

#[async_trait]
impl Page<UserRequest, UserPageResponse> for UserPageClient {
    async fn page(&self, req: UserRequest) -> Result<UserPageResponse> {
    let db = DB.get_or_init(db_conn).await;
    let pagination = Pagination::build_from_request_query(req.page_turn).count(1).build();
    //Paging queries
    let users = sqlx::query_as!(
        User,
        r#"select cast(user_id as varchar), email, username, bio, image from "user" order by user_id limit $1 offset $2"#,
        pagination.limit,
        pagination.offset,
    ).fetch_all(db).await?;
    //Query the total number of
    let tatal_count= sqlx::query_scalar!(
        r#"select count(*) from "user""#,
    ).fetch_one(db).await?.unwrap_or(0);
    
    return Ok(UserPageResponse::new(tatal_count, users));
    }
}

#[derive(serde::Deserialize)]
pub struct NewUser {
    username: String,
    email: String,
    password: String,
}

#[derive(serde::Deserialize)]
pub struct UpdateUser {
    id: String,
    username: String,
    email: String,
    password: String,
    bio: String,
    image: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateUser {
    email: String,
    token: String,
    username: String,
    bio: String,
    image: Option<String>,
}
#[derive(serde::Deserialize, Serialize)]
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
