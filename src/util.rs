extern crate serde_yaml;

use chrono::{DateTime, Local, Utc};
use log::info;
use rbatis::RBatis;
use rbdc_mysql::driver::MysqlDriver;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgramConfig {
    pub db_url: String,
    pub server_worker_size: usize,
}
pub struct DataStore {
    pub db: RBatis,
}

/**
 * 获取当前时间的时间戳
 * 格式: YYYY-MM-DD HH:mm:ss
 */
pub fn get_current_time_fmt() -> String {
    let dt = Utc::now();
    let local_dt: DateTime<Local> = dt.with_timezone(&Local);
    return local_dt.format("%Y-%m-%d %H:%M:%S").to_string();
}

pub struct MainFlow {}
impl MainFlow {
    /**
     * 生成服务启动日志
     */
    pub fn gen_server_url() -> String {
        let args: Vec<String> = env::args().collect();
        println!("cmd arg {:?}", args);
        let host = "0.0.0.0";
        let mut port = "8080";
        if let Some(val) = args.get(1) {
            port = val;
        }
        let url = format!("{}:{}", host, port);
        info!("server is on, addr http://{}", url);
        return url;
    }

    /**
     * 解析服务配置
     */
    pub fn prase_config() -> ProgramConfig {
        let yaml_str = include_str!("../config.yml");
        let conf: ProgramConfig = serde_yaml::from_str(yaml_str).unwrap();
        println!("config: {:#?}", conf);
        return conf;
    }

    /**
     * 初始化db链接
     */
    pub async fn init_db(db_url: &str) -> RBatis {
        let rb = RBatis::new();
        rb.link(MysqlDriver {}, db_url).await.unwrap();
        return rb;
    }
}
