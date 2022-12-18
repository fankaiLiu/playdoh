use crate::pagination::PageParams;
use crate::pagination::PageTurnResponse;
use crate::pagination::Pagination;
use crate::utils;
use crate::utils::jwt::AuthBody;
use crate::utils::jwt::AuthPayload;
use crate::Error;
use anyhow::Context;
use anyhow::Result;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash};
use db::system::models::sys_dept::DeptResp;
use db::system::models::sys_user::CreateUser;
use db::system::models::sys_user::LoginUser;
use db::system::models::sys_user::NewUser;
use db::system::models::sys_user::UserResp;
use db::system::models::sys_user::UserWithDept;
use uuid::Uuid;

use chrono::NaiveDateTime;
use db::db_conn; 
use db::DB;
use hyper::HeaderMap;
use serde::Deserialize;
use serde::Serialize;
use sqlx::{Pool, Postgres};

pub async fn create_user(db: &Pool<Postgres>, req: NewUser) -> Result<CreateUser> {
    let password_hash = hash_password(req.password).await?;
    let _user_id = sqlx::query_scalar!(
        // language=PostgreSQL
        r#"insert into "sys_user" (user_name, email, password_hash,is_admin,status,gender) values ($1, $2, $3,0,0,0) returning user_id"#,
        req.user_name,
        req.email,
        password_hash
    )
    .fetch_one(db)
    .await?;
    Ok(CreateUser {
        email: req.email,
        token: "token".to_string(),
        user_name: req.user_name,
    })
}
// pub type UserPageResponse = PageTurnResponse<User>;
// pub async fn page(req: PageParams) -> Result<UserPageResponse> {
//     let db = DB.get_or_init(db_conn).await;
//     let pagination = Pagination::build_from_request_query(req).count(1).build();
//     //Paging queries
//     let users = sqlx::query_as!(
//     User,
//     r#"select cast(user_id as varchar), email, username, bio, image from "sys_user" order by user_id limit $1 offset $2"#,
//     pagination.limit,
//     pagination.offset,
// ).fetch_all(db).await?;
//     //Query the total number of
//     let tatal_count = sqlx::query_scalar!(r#"select count(*) from "sys_user""#,)
//         .fetch_one(db)
//         .await?
//         .unwrap_or(0);
//     Ok(UserPageResponse::new(tatal_count, users))
// }
// pub async fn update_user(db: &Pool<Postgres>, req: UpdateUser) -> Result<String> {
//     let user_id = Uuid::parse_str(&req.id)?;
//     let _user_id = sqlx::query_scalar!(
//         // language=PostgreSQL
//         r#"update "sys_user" set username = $1, email = $2, bio = $3, image = $4 where user_id = $5 returning user_id"#,
//         req.username,
//         req.email,
//         req.bio,
//         req.image,
//         user_id
//     )
//     .fetch_one(db)
//     .await?;
//     Ok("ok".to_string())
// }

// pub async fn delete(db: &Pool<Postgres>, id: String) -> Result<String> {
//     let user_id = Uuid::parse_str(&id)?;
//     let _user_id = sqlx::query_scalar!(
//         // language=PostgreSQL
//         r#"delete from "sys_user" where user_id = $1 returning user_id"#,
//         user_id
//     )
//     .fetch_one(db)
//     .await?;
//     Ok("ok".to_string())
// }

pub async fn login(db: &Pool<Postgres>, req: LoginUser, header: HeaderMap) -> Result<AuthBody> {
    let msg = "登录成功".to_string();
    let status = "1".to_string();
    let user = sqlx::query!(
        r#"
            select user_id, email, user_name, bio, password_hash 
            from "sys_user" where email = $1
        "#,
        req.email,
    )
    .fetch_optional(db)
    .await?
    .ok_or(Error::unprocessable_entity([("email", "does not exist")]))?;

    verify_password(req.password, user.password_hash).await?;

    let claims = AuthPayload {
        user_id: user.user_id.to_string(),
        name: user.user_name.clone(),
    };
    let token_id = scru128::new_string();
    let token = utils::authorize(claims.clone(), token_id.clone())
        .await
        .unwrap();
    set_login_info(
        header,
        user.user_id.to_string(),
        user.user_name.clone(),
        msg.clone(),
        status.clone(),
        Some(token_id),
        Some(token.clone()),
    )
    .await;
    Ok(token)
}
// pub dept_id: Uuid,
// pub parent_id: String,
// pub dept_name: String,
// pub order_num: i32,
// pub leader: Option<String>,
// pub phone: Option<String>,
// pub email: Option<String>,
// pub created_at: time::OffsetDateTime,
// pub status: String,
pub async fn get_by_id(db: &Pool<Postgres>, u_id: &Uuid) -> Result<UserWithDept> {
    let user = sqlx::query_as!(
        UserResp,
        r#"select user_id , email, user_name, bio,user_nickname,gender,dept_id,remark,is_admin,phone_num,role_id,created_at from "sys_user"  where user_id = $1
        "#,
        u_id.clone(),
    )
    .fetch_one(db)
    .await?;
    let dept=sqlx::query_as!(
        DeptRessp,
        r#"select dept_id , parent_id, dept_name, order_num,leader,phone,email,created_at,status from "sys_dept"  where dept_id = $1
        "#,
        user.clone().dept_id,
    ).fetch_one(db).await?;
    let res=UserWithDept::new(user, dept);
    Ok(res)
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
        let result = hash.verify_password(&[&Argon2::default()], password);
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(anyhow::anyhow!("invalid password")),
        }
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
// cla

#[derive(serde::Deserialize)]
pub struct OrdersRequest {}
pub struct UserPageClient {}




// create table "sys_user" (
//     user_id uuid primary key default uuid_generate_v1mc(),
//     user_name text collate "case_insensitive" unique not null,
//     user_nickname text collate "case_insensitive",
//     email text collate "case_insensitive" unique not null,
//     bio text not null default '',
//     role_id uuid not null,
//     dept_id uuid not null,
//     remark text collate "case_insensitive" default null,
//     is_admin int not null,
//     phone_num varchar(20) default null,
//     last_login_ip inet default null,
//     last_login_time timestamptz default null,
//     gender bigint not null,
//     avatar text,
//     password_hash text not null,
//     created_at timestamptz not null default now(),
//     updated_at timestamptz,
//     deleted_at timestamptz
// );
