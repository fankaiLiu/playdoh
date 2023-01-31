use crate::{
    apps::system::{SysLogins,SysLoginsPage},
    middleware::{ctx::ctx_fn_mid, oper_log::oper_log_fn_mid},
    utils::jwt::Claims,
};
use axum::{middleware, routing::{post, get}, Router};
use configs::CFG;
use tower_cookies::{CookieManagerLayer};
pub mod system;

pub fn api() -> Router {
    Router::new()
        // 系统管理模块
        .nest("/system", auth_api())
        .nest("/", no_auth_api()).layer(CookieManagerLayer::new())
}

//无需授权api
pub fn no_auth_api() -> Router {
    Router::new().route("/login", post(SysLogins)) // 登录
    .route("/login", get(SysLoginsPage)) 

}

// 需要授权的api
fn auth_api() -> Router {
    let router = system::system_api();
    let router = match &CFG.log.stdout {
        true => router.layer(middleware::from_fn(oper_log_fn_mid)),
        false => router,
    };
    // let router = match CFG.server.cache_time {
    //     0 => router,
    //     _ => router.layer(middleware::from_fn(cache_fn_mid)),
    // };

    let router=router
        //     .layer(middleware::from_fn(auth_fn_mid))
        .layer(middleware::from_fn(ctx_fn_mid))
        .layer(middleware::from_extractor::<Claims>());
    router
}
