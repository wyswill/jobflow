use crate::{controller::flow::execute_shell_handler, util::prase_req};
use actix::{dev::ContextFutureSpawner, Actor, StreamHandler, WrapFuture};
use actix_web::{body::BoxBody, http::header::ContentType, HttpResponse, Responder};
use actix_web_actors::ws;
use rbatis::RBatis;
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

pub struct MyWs {
    db: RBatis,
}
impl MyWs {
    pub fn new(db: RBatis) -> Self {
        Self { db }
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

impl StreamHandler<String> for MyWs {
    fn handle(&mut self, item: String, ctx: &mut Self::Context) {
        print!("from str hand {}", item);
        ctx.text("from str hand");
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let db = self.db.clone();
                let ws_data = prase_req(text.to_string());
                let fut = async move {
                    let res = execute_shell_handler(ws_data, db).await;
                    ctx.text(res);
                };
                let _ = fut.into_actor(self).spawn(ctx);
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}
