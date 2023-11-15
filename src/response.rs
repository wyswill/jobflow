use actix::{Actor, StreamHandler};
use actix_web::{body::BoxBody, http::header::ContentType, HttpResponse, Responder};
use actix_web_actors::ws;
use serde::Serialize;

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

pub struct MyWs;
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
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}
