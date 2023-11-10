use crate::{entity::fow::Flow, request::FlowPageQuery, response::ResponseBody, DataStore};
use actix_web::{post, web, Responder};
use rbatis::sql::PageRequest;

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

// #[post("/create_flow")]
// pub async fn create_flow(_req: web::Json<CreateFlowReq>) {

// }
