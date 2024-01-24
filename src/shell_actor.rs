use crate::util::{LineStream, ShellUtil};
use actix::{Actor, ActorFutureExt, Context, Handler, Message, ResponseActFuture, WrapFuture};

#[derive(Message)]
#[rtype(result = "LineStream")]
pub struct ShellExecute {
    pub shell_string: String,
}

pub struct Despatch;

impl Actor for Despatch {
    type Context = Context<Self>;
}

impl Handler<ShellExecute> for Despatch {
    type Result = ResponseActFuture<Self, LineStream>;

    fn handle(&mut self, msg: ShellExecute, _ctx: &mut Self::Context) -> Self::Result {
        ShellUtil::gen_line_stream(msg.shell_string)
            .into_actor(self)
            .map(|res, _act, _ctx| res)
            .boxed_local()
    }
}
