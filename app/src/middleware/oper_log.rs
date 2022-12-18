use core::time::Duration;
use std::time::Instant;

use axum::{body::Body, http::Request, middleware::Next, response::IntoResponse};
use chrono::Local;
use crate::apps::system::check_user_online;
use db::{db_conn, DB, common::ctx::{UserInfo, ReqCtx}};
// use db::{
//     common::{
//         ctx::{ReqCtx, UserInfo},
//         res::ResJsonString,
//     },
//     db_conn,
//     system::entities::{prelude::SysOperLog, sys_oper_log},
//     DB,
// };
use hyper::StatusCode;

use anyhow::Result;

use crate::{
    custom_response::ResJsonString,
    utils::{api_utils::ALL_APIS}, error::ResErroString,
};

pub async fn oper_log_fn_mid(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // 查询ctx
    dbg!(1);
    let req_ctx = match req.extensions().get::<ReqCtx>() {
        Some(x) => x.clone(),
        None => return Ok(next.run(req).await),
    };
    let ctx_user = match req.extensions().get::<UserInfo>() {
        Some(x) => x.clone(),
        None => return Ok(next.run(req).await),
    };
    dbg!(1);
    let now = Instant::now();
    let res_end = next.run(req).await;
    let duration = now.elapsed();
    let res_string = match res_end.extensions().get::<ResJsonString>() {
        Some(x) => x.0.clone(),
        None => "".to_string(),
    };
    let error_msg=match res_end.extensions().get::<ResErroString>() {
        Some(x) => x.0.clone(),
        None => "".to_string(),
    };
    oper_log_add(
        req_ctx,
        ctx_user,
        res_string,
        "1".to_string(),
        error_msg,
        duration,
    )
    .await;
    Ok(res_end)
}

pub async fn oper_log_add(
    ctx: ReqCtx,
    ctx_user: UserInfo,
    res: String,
    status: String,
    err_msg: String,
    duration: Duration,
) {
    tokio::spawn(async move {
        match oper_log_add_fn(ctx, ctx_user, res, status, err_msg, duration).await {
            Ok(_) => {}
            Err(e) => {
                tracing::info!("日志添加失败：{}", e.to_string());
            }
        };
    });
}

/// add 添加
pub async fn oper_log_add_fn(
    ctx: ReqCtx,
    ctx_user: UserInfo,
    res: String,
    status: String,
    err_msg: String,
    duration: Duration,
) -> Result<()> {
    // if !CFG.log.enable_oper_log {
    //     return Ok(());
    // }
    let apis = ALL_APIS.lock().await;
    let (api_name, _is_log) = match apis.get(&ctx.path) {
        Some(x) => (x.name.clone(), x.log_method.clone()),
        None => ("".to_string(), "0".to_string()),
    };
    drop(apis);
    let now = Local::now().naive_local();
    // 打印日志
    let req_data = ctx.clone();
    let res_data = res.clone();
    let err_msg_data = err_msg.clone();
    let duration_data = duration;
    tokio::spawn(async move {
        file_log(req_data, now, duration_data, res_data, err_msg_data);
        match db_log(
            duration_data,
            ctx,
            ctx_user,
            now,
            api_name,
            res,
            status,
            err_msg,
        )
        .await
        {
            Ok(_) => {
                dbg!(2);
            }
            Err(e) => {
                tracing::info!("日志添加失败：{}", e.to_string());
            }
        };
    });
    // match is_log.as_str() {
    //     "1" => {
    //         tokio::spawn(async move {
    //             file_log(req_data, now, duration_data, res_data, err_msg_data);
    //         });
    //     }
    //     "2" => {
    //         tokio::spawn(async move {
    //             match db_log(
    //                 duration_data,
    //                 ctx,
    //                 ctx_user,
    //                 now,
    //                 api_name,
    //                 res,
    //                 status,
    //                 err_msg,
    //             )
    //             .await
    //             {
    //                 Ok(_) => {}
    //                 Err(e) => {
    //                     tracing::info!("日志添加失败：{}", e.to_string());
    //                 }
    //             };
    //         });
    //     }
    //     "3" => {
    //         tokio::spawn(async move {
    //             file_log(req_data, now, duration_data, res_data, err_msg_data);
    //             match db_log(
    //                 duration_data,
    //                 ctx,
    //                 ctx_user,
    //                 now,
    //                 api_name,
    //                 res,
    //                 status,
    //                 err_msg,
    //             )
    //             .await
    //             {
    //                 Ok(_) => {}
    //                 Err(e) => {
    //                     tracing::info!("日志添加失败：{}", e.to_string());
    //                 }
    //             };
    //         });
    //     }
    //     _ => return Ok(()),
    // }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn db_log(
    duration: Duration,
    ctx: ReqCtx,
    ctx_user: UserInfo,
    _now: chrono::NaiveDateTime,
    api_name: String,
    res: String,
    status: String,
    err_msg: String,
) -> Result<()> {
    let d = duration.as_micros() as i64;
    let db = DB.get_or_init(db_conn).await;
    let (_, m) = check_user_online(db, ctx_user.token_id.clone()).await;
    let user = match m {
        Some(x) => x,
        None => return Ok(()),
    };
    let operator_type = match ctx.method.as_str() {
        "GET" => "1",    // 查询
        "POST" => "2",   // 新增
        "PUT" => "3",    // 修改
        "DELETE" => "4", // 删除
        _ => "0",        // 其他
    };
    let _log_id = sqlx::query_scalar!(
        // language=PostgreSQL
        r#"INSERT INTO public.sys_oper_log(
            user_id, title, business_type, method, request_method, operator_type, oper_name, dept_name, oper_url, oper_ip, oper_location, oper_param, json_result, path_param, status, error_msg, duration)
            VALUES ($1,$2,$3, $4,$5,$6, $7,$8,$9, $10,$11,$12, $13,$14,$15, $16,$17)
             returning log_id"#,
        user.user_id,
        api_name,
        "0",
        ctx.method,
        ctx.method,
        operator_type,
        user.user_name,
        "default",
        ctx.path,
        user.net,
        "",
        "",
        res,
        "",
        status,
        err_msg,
        d
    )
    .fetch_one(db)
    .await?;

    Ok(())
}

fn file_log(
    req_data: ReqCtx,
    now: chrono::NaiveDateTime,
    duration_data: Duration,
    res_data: String,
    err_msg_data: String,
) {
    tracing::info!(
        "\n请求路径:{:?}\n完成时间:{:?}\n消耗时间:{:?}微秒 | {:?}毫秒\n请求数据:{:?}\n响应数据:{}\n错误信息:{:?}\n",
        req_data.path.clone(),
        now,
        duration_data.as_micros(),
        duration_data.as_millis(),
        req_data,
        res_data,
        err_msg_data,
    );
}
