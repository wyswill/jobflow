extern crate serde_yaml;

use std::env;
use chrono::{DateTime, Utc, Local};
use log::info;
use rbatis::RBatis;
use rbdc_mysql::driver::MysqlDriver;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct ProgramConfig {
    pub db_url: String,
    pub server_worker_size: usize,
}
pub struct DataStore {
    pub db: RBatis,
}

pub fn date_fmt() -> String {
  let dt =  Utc::now();
  let local_dt: DateTime<Local> = dt.with_timezone(&Local);
  local_dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub struct MainFlow {}
impl MainFlow {
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

    pub fn prase_config() -> ProgramConfig {
        let yaml_str = include_str!("../config.yml");
        let conf: ProgramConfig = serde_yaml::from_str(yaml_str).unwrap();
        println!("config: {:#?}", conf);
        conf
    }

    pub async fn init_db(db_url: &str) -> RBatis {
        let rb = RBatis::new();
        rb.link(MysqlDriver {}, db_url).await.unwrap();
        return rb;
    }
}
