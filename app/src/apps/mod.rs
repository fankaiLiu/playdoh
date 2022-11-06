use axum::Router;


pub mod system;

pub fn api() -> Router {
    Router::new()
        // 系统管理模块
        .nest("/system", auth_api())
}

// 需要授权的api
fn auth_api() -> Router {
    
    // let router = match &CFG.log.enable_oper_log {
    //     true => router.layer(middleware::from_fn(oper_log_fn_mid)),
    //     false => router,
    // };
    // let router = match CFG.server.cache_time {
    //     0 => router,
    //     _ => router.layer(middleware::from_fn(cache_fn_mid)),
    // };

    // router
    //     .layer(middleware::from_fn(auth_fn_mid))
    //     .layer(middleware::from_fn(ctx_fn_mid))
    //     .layer(middleware::from_extractor::<Claims>())
    system::system_api()
}
