use crate::{
    controller::{flow, project},
    util::{DataStore, MainFlow, ProgramConfig},
};
use actix_web::{web, App, HttpServer};
use rbatis::RBatis;
/**
 * 项目接口router
 */
fn project_config(cfg: &mut web::ServiceConfig) {
    cfg.service(project::crate_project);
    cfg.service(project::delete_project);
    cfg.service(project::get_project_list);
}
/**
 * 流程接口router
 */
fn flow_config(cfg: &mut web::ServiceConfig) {
    cfg.service(flow::create_flow);
    cfg.service(flow::get_flow_list);
}
/**
 * 启动服务
 */
pub async fn start_server() {
    let conf: ProgramConfig = MainFlow::prase_config();
    //创建 http 服务器
    let db: RBatis = MainFlow::init_db(&conf.db_url).await;

    let app_data: web::Data<DataStore> = web::Data::new(DataStore { db });

    let _ = HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(web::scope("/api/project").configure(project_config))
            .service(web::scope("/api/flow").configure(flow_config))
    })
    .workers(conf.server_worker_size)
    .bind(MainFlow::gen_server_url())
    .expect("服务启动失败")
    .run()
    .await;
}
