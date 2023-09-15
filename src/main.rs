extern crate rbatis;

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

struct MainFlow {}
impl MainFlow {
    pub fn gen_url() -> String {
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

    pub async fn init_db() -> RBatis {
        let rb = RBatis::new();
        rb.link(
            MysqlDriver {},
            "",
        )
        .await
        .unwrap();
        return rb;
    }
}

pub struct DataStore {
    pub db: RBatis,
}

async fn start_server() {
    //创建 http 服务器
    let db: RBatis = MainFlow::init_db().await;

    let app_data: web::Data<DataStore> = web::Data::new(DataStore { db });

    let _ = HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(controller::crate_project)
            .service(controller::get_project_list)
    })
    .workers(1)
    .bind(MainFlow::gen_url())
    .expect("服务启动失败")
    .run()
    .await;
}
