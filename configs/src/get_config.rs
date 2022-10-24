use confique::Config;
use once_cell::sync::Lazy;

use super::configs::Configs;

const CFG_FILE: &str = "config/config.toml";

pub static CFG: Lazy<Configs> = Lazy::new(self::Configs::init);
impl Configs {
    pub fn init() -> Self {
        let config = Configs::builder()
        .env()
        .file(CFG_FILE)
        .load().expect("解析配置文件错误");
        config
    }
}