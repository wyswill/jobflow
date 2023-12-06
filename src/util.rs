extern crate serde_yaml;
use actix_web::{HttpResponse, Responder};
use chrono::{DateTime, Local, Utc};
use log::info;
use rbatis::RBatis;
use rbdc_mysql::driver::MysqlDriver;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::{env, fs, path};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::mpsc;
use tokio_stream::Stream;
use tokio_util::bytes::Bytes;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgramConfig {
    pub db_url: String,
    pub server_worker_size: usize,
    pub server_port: String,
    pub work_space: String,
}
pub struct DataStore {
    pub db: RBatis,
    pub work_space: String,
}

/**
 * 获取当前时间的时间戳
 * 格式: YYYY-MM-DD HH:mm:ss
 */
pub fn get_current_time_fmt() -> String {
    let dt = Utc::now();
    let local_dt: DateTime<Local> = dt.with_timezone(&Local);
    return local_dt.format("%Y-%m-%d %H:%M:%S").to_string();
}

pub struct MainFlow {
    pub config: ProgramConfig,
}
impl MainFlow {
    pub async fn init() -> MainFlow {
        let config: ProgramConfig = MainFlow::prase_config();
        MainFlow { config }
    }
    /**
     * 生成服务启动日志
     */
    pub fn gen_server_url(&mut self) -> String {
        let host = "0.0.0.0";
        let url = format!("{}:{}", host, self.config.server_port);
        info!("server is on, addr http://{}", url);
        return url;
    }

    /**
     * 解析服务配置
     */
    fn prase_config() -> ProgramConfig {
        let args: Vec<String> = env::args().collect();
        println!("{:?}", args);
        if let Some(config_path) = args.get(1) {
            let yaml_str = fs::read_to_string(config_path).expect("配置读取失败");
            let conf: ProgramConfig = serde_yaml::from_str(&yaml_str).expect("配置转换失败");
            println!("config: {:#?}", conf);
            return conf;
        } else {
            panic!("配置读取失败");
        }
    }

    /**
     * 初始化db链接
     */
    pub async fn init_db(&self, db_url: &str) -> RBatis {
        let rb = RBatis::new();
        rb.link(MysqlDriver {}, db_url).await.unwrap();
        return rb;
    }
}

pub struct LineStream {
    pub receiver: mpsc::Receiver<std::io::Result<String>>,
}

impl Stream for LineStream {
    type Item = std::io::Result<Bytes>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.receiver).poll_recv(cx) {
            Poll::Ready(Some(Ok(line))) => Poll::Ready(Some(Ok(Bytes::from(line)))),
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

pub struct ShellUtil;

impl ShellUtil {
    pub fn check_work_space(work_space: String, project_name: String, flow_name: String) -> String {
        let root_dir = path::Path::new(&work_space);
        let root_dir = root_dir.join(project_name).join(flow_name);
        if !root_dir.exists() {
            fs::create_dir_all(root_dir.clone()).expect("创建work space 失败");
        }
        root_dir.to_str().unwrap().to_string()
    }

    pub fn del_work_space(work_space: String) -> Result<(), std::io::Error> {
        let res: Result<(), std::io::Error> = fs::remove_dir_all(path::Path::new(&work_space));
        res
    }

    pub fn exec_shell(shell_str: String) -> impl Responder {
        let output = Command::new("sh")
            .arg("-c")
            .arg(shell_str)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn();
        // TODO: 加入取消job 功能 加入执行完成回调
        let mut child = match output {
            Ok(child) => child,
            Err(e) => {
                println!("{:#?}", e);
                return HttpResponse::InternalServerError()
                    .body(format!("Command spawn error: {}", e));
            }
        };

        // 创建输出流
        let (sender, receiver) = mpsc::channel(10);

        // 拿去标准输出和标准错误输出
        let stdout = match child.stdout.take() {
            Some(stdout) => stdout,
            None => return HttpResponse::InternalServerError().body("Failed to capture stdout"),
        };
        let stderr = match child.stderr.take() {
            Some(stderr) => stderr,
            None => return HttpResponse::InternalServerError().body("Failed to capture stderr"),
        };

        // 创建流读取器
        let stdout_reader = BufReader::new(stdout);
        let stderr_reader = BufReader::new(stderr);

        let sender_stdout = sender.clone();
        tokio::spawn(async move {
            let mut lines = stdout_reader.lines();
            while let Some(mut line) = lines.next_line().await.unwrap() {
                line.push_str("\n");
                sender_stdout.send(Ok(line)).await.unwrap();
            }
        });

        let sender_stderr = sender.clone();
        tokio::spawn(async move {
            let mut lines = stderr_reader.lines();
            while let Some(mut line) = lines.next_line().await.unwrap() {
                line.push_str("\n");
                sender_stderr.send(Ok(line)).await.unwrap();
            }
        });

        HttpResponse::Ok()
            .content_type("text/event-stream")
            .streaming(LineStream { receiver })
    }
}
