extern crate rbatis;
extern crate serde;
extern crate serde_yaml;

mod controller;
mod model;
mod request;
mod response;
mod util;

use actix_web::{web, App, HttpServer};
use fast_log::{
    consts::LogSize,
    plugin::{file_split::RollingType, packer::LogPacker},
    Config,
};
use log::info;
use rbatis::RBatis;
use rbdc_mysql::driver::MysqlDriver;
use serde::{Deserialize, Serialize};
use std::env;

//将 async main 函数标记为 actix 系统的入口点。
#[actix_web::main]
async fn main() {
    fast_log::init(Config::new().console().chan_len(Some(100000)).file_split(
        "logs/",
        LogSize::MB(10),
        RollingType::All,
        LogPacker {},
    ))
    .unwrap();
    start_server().await;
    log::logger().flush();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgramConfig {
    pub db_url: String,
}
struct MainFlow {}
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

    fn prase_config() -> ProgramConfig {
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

pub struct DataStore {
    pub db: RBatis,
}

async fn start_server() {
    let conf = MainFlow::prase_config();
    //创建 http 服务器
    let db: RBatis = MainFlow::init_db(&conf.db_url).await;

    let app_data: web::Data<DataStore> = web::Data::new(DataStore { db });

    let _ = HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(controller::crate_project)
            .service(controller::get_project_list)
    })
    .workers(1)
    .bind(MainFlow::gen_server_url())
    .expect("服务启动失败")
    .run()
    .await;
}
