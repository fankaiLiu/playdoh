use std::net::SocketAddr;

use app::apps;
use app::starting::{self};
use axum::Router;
use configs::CFG;
use db::{db_conn, DB};
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};
#[tokio::main]

async fn main() {
    let configs = &CFG.http;

    let log_env = starting::get_log_level();

    //  日志设置
    let format = starting::get_log_format();

    // 文件输出
    let file_appender = tracing_appender::rolling::hourly(&CFG.log.dir, &CFG.log.file);
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // 控制台输出
    let (std_non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    let logger = Registry::default()
        .with(EnvFilter::from_default_env().add_directive(log_env.into()))
        .with(fmt::Layer::default().with_writer(std_non_blocking).event_format(format.clone()).pretty())
        .with(fmt::Layer::default().with_writer(non_blocking).event_format(format))
        // .with(console_layer)
        ;
    tracing::subscriber::set_global_default(logger).unwrap();
    dbg!(configs);

    // let app = Router::new()
    //     // `GET /` goes to `root`
    //     .route("/", get(root));
    let addr = SocketAddr::from((configs.bind, configs.port));
    tracing::debug!("listening on {}", addr);

    let app = Router::new().nest("/", apps::api());

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    let _db = DB.get_or_init(db_conn).await;
    "Hello, World!"
}
