use crate::{
    controller::{flow, project},
    util::{DataStore, MainFlow},
};
use actix_web::{web, App, HttpServer};
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
pub async fn start_http_server(config: &MainFlow) {
    let db = config.init_db(&config.config.db_url).await;
    //创建 http 服务器
    let app_data: web::Data<DataStore> = web::Data::new(DataStore { db });

    let _ = HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(web::scope("/api/project").configure(project_config))
            .service(web::scope("/api/flow").configure(flow_config))
            .service(flow::handle_ws)
    })
    .workers(config.config.server_worker_size)
    .bind(MainFlow::gen_server_url())
    .expect("服务启动失败")
    .run()
    .await;
}
