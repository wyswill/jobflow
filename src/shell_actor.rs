use actix::{Actor, ActorFutureExt, Context, Handler, Message, ResponseActFuture, WrapFuture};
use tokio::{io::AsyncBufReadExt, sync::mpsc};

use crate::util::{LineStream, ShellUtil};

#[derive(Message)]
#[rtype(result = "LineStream")]
pub struct ShellExecute {
    pub shell_string: String,
}

pub struct Despatch;

impl Actor for Despatch {
    type Context = Context<Self>;
}

pub async fn gen_line_stream(shell: String) -> LineStream {
    // 注意: 复杂shell会导致队列堵塞
    let (sender, receiver) = mpsc::channel(1000);

    let mut child = ShellUtil::spawn_new_command(shell);
    let (stdout_reader, stderr_reader) = ShellUtil::get_std_reader(&mut child);

    // 创建流读取器
    let mut lines = stdout_reader.lines();
    while let Some(mut line) = lines.next_line().await.unwrap() {
        line.push_str("\n");
        sender.send(Ok(line)).await.expect("send shell msg err")
    }

    let mut err_lines = stderr_reader.lines();
    while let Some(mut line) = err_lines.next_line().await.unwrap() {
        line.push_str("\n");
        sender.send(Ok(line)).await.expect("send err msg err")
    }

    match child.wait().await {
        Ok(status) => sender
            .send(Ok(format!("{}", status.to_string())))
            .await
            .unwrap(),
        Err(e) => println!("Failed to wait for child process: {}", e),
    }

    LineStream { receiver }
}

impl Handler<ShellExecute> for Despatch {
    type Result = ResponseActFuture<Self, LineStream>;

    fn handle(&mut self, msg: ShellExecute, _ctx: &mut Self::Context) -> Self::Result {
        gen_line_stream(msg.shell_string)
            .into_actor(self)
            .map(|res, _act, _ctx| res)
            .boxed_local()
    }
}
