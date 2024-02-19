extern crate rbatis;
extern crate serde;
extern crate serde_yaml;

mod controller;
mod entity;
mod request;
mod response;
mod server;
mod util;

use fast_log::{
    consts::LogSize,
    plugin::{file_split::RollingType, packer::LogPacker},
    Config,
};
use server::start_http_server;
use util::MainFlow;

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
    let mut main_flow = MainFlow::init().await;

    start_http_server(&mut main_flow).await;
    // log::logger().flush();
}
