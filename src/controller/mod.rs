use crate::{
    model::Project,
    request::CreateProjectBody,
    response::{ListRsp, ResponseBody},
    util::date_fmt,
    DataStore,
};
use actix_web::{get, post, web, Responder};
use rbatis::RBatis;

#[get("/get_project_list")]
pub async fn get_project_list(data: web::Data<DataStore>) -> impl Responder {
    let db: &RBatis = &data.db;
    let mut rsp_body = ResponseBody {
        rsp_code: 0,
        rsp_msg: "".into(),
        data: ListRsp {
            list: Vec::new(),
            total: 10,
        },
    };
    if let Ok(project_list) = Project::select_all(db).await {
        rsp_body.data.total = project_list.len();
        rsp_body.data.list = project_list;
    }

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
