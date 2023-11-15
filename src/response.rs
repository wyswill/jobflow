use actix::{Actor, StreamHandler};
use actix_web::{body::BoxBody, http::header::ContentType, web::Data, HttpResponse, Responder};
use actix_web_actors::ws;
use serde::Serialize;
use tokio::runtime::Runtime;

use crate::{controller::flow::execute_shell_handler, request::WsData, util::DataStore};

#[derive(Serialize)]
pub struct ResponseBody<T> {
    pub rsp_code: i8,
    pub rsp_msg: String,
    pub data: T,
}

impl<T: Serialize> Responder for ResponseBody<T> {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

pub struct MyWs {
    app_data: Data<DataStore>,
}
impl MyWs {
    pub fn new(app_data: Data<DataStore>) -> MyWs {
        MyWs { app_data }
    }
}
impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("ws 链接已建立");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("ws 链接已断开");
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                println!("ws text {}", text);
                let ws_data: WsData =
                    serde_json::from_str(&text.to_string()).expect("ws data 解析失败");
                let res = execute_shell_handler(ws_data, &self.app_data);
                ctx.text(res)
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}
