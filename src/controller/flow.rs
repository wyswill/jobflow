use crate::{
    entity::{fow::Flow, project::Project},
    request::{CreateFlowReq, FlowPageQuery, WsData},
    response::{MyWs, ResponseBody},
    util::{get_current_time_fmt, DataStore},
};
use actix_web::{post, web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use rbatis::sql::PageRequest;

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

pub async fn handle_ws(
    req: HttpRequest,
    stream: web::Payload,
    // app_data: web::Data<DataStore>,
) -> Result<HttpResponse, Error> {
    // let resp = ws::start(MyWs {}, &req, stream);
    let ws_rsp: HttpResponse = handle_ws(req, stream).await?;
    // let ws_data: WsData = serde_json::from_str(&resp.to_string()).expect("ws data 解析失败");
    // actix_web::web::block(async move {
    //     execute_shell_handler(ws_data, &self.app_data).await
    // })
    // .await
    // .unwrap()
    Ok(ws_rsp)
}

pub async fn execute_shell_handler(ws_data: WsData, _data: &web::Data<DataStore>) -> String {
    let res = Project::select_by_name(&_data.db, &ws_data.project_name)
        .await
        .expect("查询项目失败");
    match res {
        Some(project_data) => {
            println!("{:#?}", project_data);
            return format!("{:#?}", project_data);
        }
        _ => return "".to_string(),
    }
}
