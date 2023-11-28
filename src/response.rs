use crate::{controller::flow::prase_cmd, util::prase_req};
use actix::{Actor, ActorFutureExt, AsyncContext, StreamHandler, WrapFuture};
use actix_web::{body::BoxBody, http::header::ContentType, HttpResponse, Responder};
use actix_web_actors::ws;
use rbatis::RBatis;
use serde::Serialize;
use std::process::Command;

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
    // TODO 添加心跳链接
}
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let db = self.db.clone();
                let ws_data = prase_req(text.to_string());
                let fut = async move {
                    let vec_shell = prase_cmd(ws_data, db).await;
                    vec_shell
                };
                ctx.wait(fut.into_actor(self).map(|res, _self, ctx| {
                    for sh in res {
                        if sh.is_empty() {
                            continue;
                        }
                        let output = Command::new("sh")
                            .arg("-c")
                            .arg(sh)
                            .output()
                            .expect("Failed to execute command");
                        if output.status.success() {
                            let stdout = String::from_utf8_lossy(&output.stdout);
                            println!("Command executed successfully:\n{}", stdout);
                            ctx.text(stdout.to_string());
                        } else {
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            println!("Command failed:\n{}", stderr);
                            ctx.text(stderr.to_string());
                        }
                    }
                }));
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}
