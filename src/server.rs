use crate::{
    controller::{flow, project},
    util::{DataStore, MainFlow},
};
use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use actix_web_static_files::ResourceFiles;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
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
    cfg.service(flow::delete_flow);
    cfg.service(flow::execute);
    cfg.service(flow::get_detail);
    cfg.service(flow::update_flow);
}

include!(concat!(env!("OUT_DIR"), "/generated.rs"));
/**
 * 启动服务
 */
pub async fn start_http_server(config: &mut MainFlow) {
    let db = config.init_db(&config.config.db_url).await;
    //创建 http 服务器
    let app_data: web::Data<DataStore> = web::Data::new(DataStore {
        db,
        work_space: config.config.work_space.clone(),
        executing_child: Arc::new(Mutex::new(HashMap::new())),
    });

    let _ = HttpServer::new(move || {
        let _generated = generate();
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "DELETE"])
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::ACCEPT,
                        http::header::CONTENT_TYPE,
                    ])
                    .max_age(3600),
            )
            .app_data(app_data.clone())
            .service(web::scope("/api/project").configure(project_config))
            .service(web::scope("/api/flow").configure(flow_config))
            .service(ResourceFiles::new("/", _generated))
    })
    .workers(config.config.server_worker_size)
    .bind(config.gen_server_url())
    .expect("服务启动失败")
    .run()
    .await;
}
