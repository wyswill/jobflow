use crate::{
    entity::{fow::Flow, project::Project},
    request::{CreateFlowReq, FlowPageQuery, WsData},
    response::{MyWs, ResponseBody},
    util::{get_current_time_fmt, DataStore},
};
use actix_web::{post, web, Error, HttpRequest, HttpResponse, Responder, get};
use actix_web_actors::ws;
use rbatis::{sql::PageRequest, RBatis};

#[post("/get_flow_list")]
pub async fn get_flow_list(
    _req: web::Json<FlowPageQuery>,
    _data: web::Data<DataStore>,
) -> impl Responder {
    let flows = Flow::select_page_by_name(
        &_data.db,
        &PageRequest::new(_req.offset as u64, _req.size as u64),
        &_req.project_name,
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

#[post("/create_flow")]
pub async fn create_flow(
    _req: web::Json<CreateFlowReq>,
    _data: web::Data<DataStore>,
) -> impl Responder {
    let mut res = ResponseBody {
        rsp_code: 0,
        rsp_msg: "".to_string(),
        data: "".to_string(),
    };

    let name: String = _req.flow_name.clone();
    let shell_str: String = _req.shell_str.clone();

    if let Ok(has_flow) = Flow::select_by_name(&_data.db, &name).await {
        match has_flow {
            Some(_) => {
                res.rsp_code = -1;
                res.rsp_msg = "流程已存在".into();
                return res;
            }
            _ => {}
        }
    }

    let flow_data = Flow {
        id: None,
        name,
        create_time: get_current_time_fmt(),
        update_time: get_current_time_fmt(),
        shell_str,
    };

    let _ = Flow::insert(&_data.db, &flow_data).await;
    res.rsp_msg = "流程创建成功".into();
    res
}

#[get("/ws")]
pub async fn handle_ws(
    req: HttpRequest,
    stream: web::Payload,
    _app_data: web::Data<DataStore>,
) -> Result<HttpResponse, Error> {
    let my_actor = MyWs::new(_app_data.db.clone());
    let res = ws::start(my_actor, &req, stream);
    res
}

pub async fn execute_shell_handler(ws_data: WsData, db: RBatis) -> String {
    let res: Option<Project> = Project::select_by_name(&db, &ws_data.project_name)
        .await
        .expect("查询项目失败");

    match res {
        Some(project_data) => {
            return format!("{:#?}", project_data);
        }
        _ => return "".to_string(),
    }
}
