use configs::CFG;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::sync::OnceCell;
//  异步初始化数据库
pub static DB: OnceCell<PgPool> = OnceCell::const_new();

pub async fn db_conn() -> PgPool {
    let db = PgPoolOptions::new()
        // The default connection limit for a Postgres server is 100 connections, minus 3 for superusers.
        // Since we're using the default superuser we don't have to worry about this too much,
        // although we should leave some connections available for manual access.
        //
        // If you're deploying your application with multiple replicas, then the total
        // across all replicas should not exceed the Postgres connection limit.
        .max_connections(50)
        .connect(&CFG.database.link)
        .await.expect("数据库连接失败");
    // This embeds database migrations in the application binary so we can ensure the database
    // is migrated correctly on startup
    sqlx::migrate!().run(&db).await.expect("数据库迁移失败");
    db
}
