extern crate rbatis;
extern crate serde;
extern crate serde_yaml;

mod controller;
mod entity;
mod request;
mod response;
mod util;

use actix_web::{web, App, HttpServer};
use fast_log::{
    consts::LogSize,
    plugin::{file_split::RollingType, packer::LogPacker},
    Config,
};
use rbatis::RBatis;
use util::{DataStore, MainFlow};

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

async fn start_server() {
    let conf = MainFlow::prase_config();
    //创建 http 服务器
    let db: RBatis = MainFlow::init_db(&conf.db_url).await;

    let app_data: web::Data<DataStore> = web::Data::new(DataStore { db });

    let _ = HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(controller::project::crate_project)
            .service(controller::project::get_project_list)
            .service(controller::project::delete_project)
            .service(controller::flow::get_flow_list)
    })
    .workers(conf.server_worker_size)
    .bind(MainFlow::gen_server_url())
    .expect("服务启动失败")
    .run()
    .await;
}
