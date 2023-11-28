use crate::{
    entity::{fow::Flow, project_flow::ProjectFlow},
    request::{CreateFlowReq, FlowPageQuery, WsData},
    response::{MyWs, ResponseBody},
    util::{get_current_time_fmt, DataStore},
};
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use rbatis::{rbdc::db::ExecResult, sql::PageRequest, RBatis};
use rbs::Value;

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
    // TODO: 添加危险shell 过滤
    let shell_str: String = _req.shell_str.clone();

    // 检测流程是否存在
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

    let insert_flow_res: ExecResult = Flow::insert(&_data.db, &flow_data)
        .await
        .expect("创建流程失败");
    // 检测关系是否存在

    if let Value::U64(id) = insert_flow_res.last_insert_id {
        println!("flow id : {}", id);
        let project_flow_res = ProjectFlow::select_by_flow_id(&_data.db, id)
            .await
            .expect("获取项目和流程关系失败");
        match project_flow_res {
            // 存在关系
            Some(pf) => {
                if pf.project_id.eq(&_req.project_id) {
                    res.rsp_code = -1;
                    res.rsp_msg = "该项目下流程已存在".into();
                    return res;
                }
            }
            _ => {
                let pf = ProjectFlow {
                    id: None,
                    project_id: _req.project_id,
                    flow_id: id as i16,
                };
                ProjectFlow::insert(&_data.db, &pf)
                    .await
                    .expect("创建项目-流程关系失败");
            }
        }
    } else {
        res.rsp_msg = "流程创建失败".into();
        return res;
    }

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

pub async fn prase_cmd(ws_data: WsData, db: RBatis) -> Vec<String> {
    let flow_data: Flow = Flow::select_bu_id(&db, &ws_data.flow_id)
        .await
        .expect("流程查询失败")
        .unwrap();
    // TODO: 添加危险shell 过滤
    let vec_shell = Vec::from_iter(flow_data.shell_str.split("\n").map(|sh| sh.to_string()));
    vec_shell
}
