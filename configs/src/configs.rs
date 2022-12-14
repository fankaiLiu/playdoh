#![allow(dead_code)]

use confique::Config;
use std::{net::IpAddr, path::PathBuf};

#[derive(Debug, Config)]
pub struct Configs {
    #[config(nested)]
    pub http: Http,

    #[config(nested)]
    pub log: LogConfig,

    #[config(nested)]
    pub database: Database,

    #[config(nested)]
    pub jwt: Jwt,
    #[config(nested)]
    pub system: System,
}

#[derive(Debug, Config)]
pub struct Jwt {
    pub jwt_secret: String,
    pub jwt_exp: i64,
}

/// Configuring the HTTP server of our app.
#[derive(Debug, Config)]
pub struct Http {
    /// The port the server will listen on.
    #[config(env = "PORT")]
    pub port: u16,

    /// The bind address of the server. Can be set to `0.0.0.0` for example, to
    /// allow other users of the network to access the server.
    #[config(default = "127.0.0.1")]
    pub bind: IpAddr,
}

#[derive(Debug, Config)]
pub struct LogConfig {
    /// If set to `true`, the app will log to stdout.
    #[config(default = true)]
    pub stdout: bool,
    /// log level of the app.
    pub log_level: String,
    /// `dir` is the directory where the log files will be stored.
    pub dir: String,
    /// `file` is the name of the log file.
    pub file: PathBuf,
}

#[derive(Debug, Config)]
pub struct Database {
    pub link: String,
    pub hmac_key: String,
}

#[derive(Debug, Config)]

pub struct System {
    pub user_agent_parser: String,
    pub super_user: Vec<String>,
}
