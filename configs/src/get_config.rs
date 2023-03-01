use std::path::PathBuf;

use confique::Config;
use once_cell::sync::Lazy;

use super::configs::Configs;

pub static CFG: Lazy<Configs> = Lazy::new(self::Configs::init);
impl Configs {
    pub fn init() -> Self {
        let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        //
        config_path.pop();
        let config_path = config_path.join("config").join("config.toml");
        //回到上一级目录
        dbg!(&config_path);
        Configs::builder()
            .env()
            .file(config_path)
            .load()
            .expect("解析配置文件错误")
    }
}
