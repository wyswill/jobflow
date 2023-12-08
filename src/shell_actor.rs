use actix::{Actor, Context, Handler, Message};

pub struct Despatch;

impl Default for Despatch {
    fn default() -> Self {
        Self {}
    }
}

impl Actor for Despatch {
    type Context = Context<Self>;
}
pub struct ShellExecute {
    pub shell_string: String,
}

impl Message for ShellExecute {
    type Result = String;
}

impl Handler<ShellExecute> for Despatch {
    type Result = String;

    fn handle(&mut self, msg: ShellExecute, ctx: &mut Self::Context) -> Self::Result {
        let _ = ctx;
        msg.shell_string
    }
}

// impl StreamHandler<String> for Despatch {
//     type Result = ResponseFuture<Result<web::Bytes, actix_web::Error>>;
//     fn handle(&mut self, item: String, _ctx: &mut Self::Context) -> Self::Result {
//         Box::pin(async {
//             Ok(web::Bytes::from_static(b"data\n"))
//         })
//     }
// }
