use crate::{
    apps::system::{SysLogins,SysLoginCheck,SysLoginsPage},
    middleware::{ctx::ctx_fn_mid, oper_log::oper_log_fn_mid},
    utils::jwt::Claims,
};
use axum::{middleware, routing::{post, get}, Router};
use configs::CFG;
use once_cell::sync::Lazy;
use tower_cookies::{CookieManagerLayer};

use self::runtime::{service::{runtime_function::RuntimeFuctionService, runtime_function_log::RuntimeFuctionLogService}, api::runtime_function_api};
pub mod system;
pub mod runtime;

pub fn api() -> Router {
    Router::new()
        // 系统管理模块
        .nest("/system", auth_api())
        .nest("/", no_auth_api())
        .nest("/runtime", runtime_function_api())
        .layer(CookieManagerLayer::new())
}

//无需授权api
pub fn no_auth_api() -> Router {
    Router::new().route("/login", post(SysLogins)) // 登录
    .route("/login", get(SysLoginCheck)) 
    .route("/login_page", get(SysLoginsPage)) 
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

pub static CONTEXT: Lazy<ServiceContext> = Lazy::new(|| ServiceContext::default());
pub struct ServiceContext {
    pub runtime_funciton:RuntimeFuctionService,
    pub runtime_function_log:RuntimeFuctionLogService,
}
impl Default for ServiceContext {
    fn default() -> Self {
        ServiceContext {
            runtime_funciton:RuntimeFuctionService::new(),
            runtime_function_log:RuntimeFuctionLogService::new(),
         }
    }
}
