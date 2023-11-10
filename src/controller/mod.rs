use crate::{
    model::{Flow, Project},
    request::{CreateProjectBody, FlowPageQuery, PageQuery},
    response::ResponseBody,
    util::date_fmt,
    DataStore,
};
use actix_web::{post, web, Responder};
use rbatis::{sql::PageRequest, RBatis};

#[post("/get_project_list")]
pub async fn get_project_list(
    data: web::Data<DataStore>,
    _req: web::Json<PageQuery>,
) -> impl Responder {
    let db: &RBatis = &data.db;
    let project_list = Project::select_page(
        db,
        &PageRequest::new(_req.0.offset as u64, _req.0.size as u64),
    )
    .await
    .unwrap();
    let rsp_body = ResponseBody {
        rsp_code: 0,
        rsp_msg: "".into(),
        data: project_list,
    };
    rsp_body
}

#[post("/crate_project")]
pub async fn crate_project(
    _req: web::Json<CreateProjectBody>,
    _data: web::Data<DataStore>,
) -> impl Responder {
    let mut rsp = ResponseBody {
        rsp_code: 0,
        rsp_msg: "".into(),
        data: "".to_string(),
    };
    let name = _req.0.name.clone();

    if let Ok(has_project) = Project::select_by_name(&_data.db, &name).await {
        match has_project {
            Some(_) => {
                rsp.rsp_code = -1;
                rsp.rsp_msg = "项目已存在".into();
                return rsp;
            }
            _ => {}
        }
    }

    let project: Project = Project {
        id: None,
        flow_id: None,
        name,
        create_time: date_fmt(),
        update_time: date_fmt(),
    };

    let _ = Project::insert(&_data.db, &project).await;
    rsp.rsp_msg = "项目创建成功".into();
    rsp
}

#[post("/get_flow_list")]
pub async fn get_flow_list(
    _req: web::Json<FlowPageQuery>,
    _data: web::Data<DataStore>,
) -> impl Responder {
    let flows = Flow::select_page_by_name(
        &_data.db,
        &PageRequest::new(_req.0.offset as u64, _req.0.size as u64),
        &_req.0.project_name,
    )
    .await
    .unwrap();
    let rsp = ResponseBody {
        rsp_code: 0,
        rsp_msg: "".to_string(),
        data: flows,
    };
    rsp
}
