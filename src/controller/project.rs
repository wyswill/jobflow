use crate::{
    entity::{fow::Flow, project::Project, project_flow::ProjectFlow},
    request::{CreateProjectBody, IdReq, PageQuery},
    response::ResponseBody,
    util::{get_current_time_fmt, DataStore, ShellUtil},
};
use actix_web::{delete, post, web, Responder};
use rbatis::{sql::PageRequest, RBatis};

#[post("/get_project_list")]
pub async fn get_project_list(
    data: web::Data<DataStore>,
    _req: web::Json<PageQuery>,
) -> impl Responder {
    let db: &RBatis = &data.db;
    let project_list =
        Project::select_page(db, &PageRequest::new(_req.offset as u64, _req.size as u64))
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
    let name = _req.name.clone();

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
        name,
        create_time: get_current_time_fmt(),
        update_time: get_current_time_fmt(),
    };

    let _ = Project::insert(&_data.db, &project).await;
    rsp.rsp_msg = "项目创建成功".into();
    rsp
}

#[delete("/delete_project")]
pub async fn delete_project(_req: web::Json<IdReq>, _data: web::Data<DataStore>) -> impl Responder {
    let mut res = ResponseBody {
        rsp_code: 0,
        rsp_msg: "".to_string(),
        data: "".to_string(),
    };

    let db_project = Project::select_by_id(&_data.db, &_req.id.to_string())
        .await
        .expect("项目查询失败");
    match db_project {
        Some(_) => {}
        _ => {
            res.rsp_code = -1;
            res.rsp_msg = "项目未找到".to_string();
            return res;
        }
    }

    let flow_relation = ProjectFlow::select_by_project_id(&_data.db, _req.id)
        .await
        .expect("查询关系失败");

    for fr in flow_relation {
        let _ = Flow::delete_by_column(&_data.db, "id", fr.flow_id)
            .await
            .expect("删除flow失败");
    }

    let work_space = ShellUtil::check_work_space(
        _data.work_space.clone(),
        db_project.unwrap().name,
        "".to_string(),
    );
    
    if let Err(e) = ShellUtil::del_work_space(work_space) {
        panic!("{}", e.to_string());
    }

    let _ = ProjectFlow::delete_by_project_id(&_data.db, _req.id)
        .await
        .expect("删除关联的flow");

    let _ = Project::delete_by_column(&_data.db, "id", &_req.id)
        .await
        .expect("项目删除失败");
    res.rsp_msg = "项目删除成功".to_string();

    res
}
