use actix::{Actor, Addr, AsyncContext, Context, Handler, Message, Running};
use std::process::Stdio;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

struct Executor {
    command: Option<tokio::process::Child>,
}

impl Actor for Executor {
    type Context = Context<Self>;

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        if let Some(mut command) = self.command.take() {
            let _ = command.kill();
        }
        Running::Stop
    }
}

struct ExecuteScript(String);

impl Message for ExecuteScript {
    type Result = Result<Addr<Executor>, std::io::Error>;
}
impl Handler<ExecuteScript> for Executor {
    type Result = Result<Addr<Executor>, std::io::Error>;

    fn handle(&mut self, msg: ExecuteScript, ctx: &mut Self::Context) -> Self::Result {
        let script = &msg.0;
        let output = Command::new("sh")
            .arg("-c")
            .arg(&script)
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute process");

        // let addr = ctx.address();
        let mut reader = BufReader::new(output.stdout.unwrap()).lines();
        actix::spawn(async move {
            if let Some(mut line) = reader.next_line().await.unwrap() {
                line.push_str("\n");
            }
            futures::future::ready(())
        });

        // self.command = Some(output);

        Ok(ctx.address())
    }
}
