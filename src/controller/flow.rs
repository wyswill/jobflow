use std::default;

use crate::{
    entity::{fow::Flow, project::Project, project_flow::ProjectFlow},
    request::{CreateFlowReq, FlowPageQuery, IdReq, UpdateFLowReq},
    response::ResponseBody,
    shell_actor::{Despatch, ShellExecute},
    util::{get_current_time_fmt, DataStore, LineStream, ShellUtil},
};
use actix::Actor;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use rbatis::{rbdc::db::ExecResult, sql::Page, RBatis};
use rbs::{to_value, Value};
use tokio::{io::AsyncBufReadExt, sync::mpsc};
enum HasFlowInDb<T> {
    Has(T),
    None(T),
}

#[post("/get_flow_list")]
pub async fn get_flow_list(
    _req: web::Json<FlowPageQuery>,
    _data: web::Data<DataStore>,
) -> impl Responder {
    let relations: Vec<ProjectFlow> = _data
        .db
        .query_decode(
            "select * from project_flow where project_id = ?",
            vec![to_value!(_req.project_id)],
        )
        .await
        .expect("获取项目和流程关系失败");

    let default_data: Page<Flow> = Page {
        records: vec![],
        total: 0,
        page_no: 0,
        page_size: 10,
        do_count: true,
    };
    let mut res = ResponseBody {
        rsp_code: 0,
        rsp_msg: "".to_string(),
        data: default_data,
    };
    if relations.len().eq(&0) {
        return res;
    }

    let relation_iter = relations.into_iter();
    let mut ids: Vec<String> = Vec::new();

    relation_iter.for_each(|ele| {
        ids.push(ele.flow_id.to_string());
    });

    let sql = format!("select * from flow where id in ({})", ids.join(","));

    let flows: Vec<Flow> = _data
        .db
        .query_decode(&sql, vec![])
        .await
        .expect("查询flow失败");

    let total = flows.len() as u64;
    let page_res: Page<Flow> = Page {
        records: flows,
        total,
        page_no: 0,
        page_size: 10,
        do_count: true,
    };
    res.data = page_res;
    res
}

#[post("/create_flow")]
pub async fn create_flow(
    _req: web::Json<CreateFlowReq>,
    _data: web::Data<DataStore>,
) -> impl Responder {
    let mut res = ResponseBody {
        rsp_code: 0,
        rsp_msg: "".to_string(),
        data: "".to_string(),
    };

    let name: String = _req.flow_name.clone();
    // TODO: 添加危险shell 过滤
    let shell_str: String = _req.shell_str.clone();

    // 检测流程是否存在
    if let Ok(has_flow) = Flow::select_by_name(&_data.db, &name).await {
        match has_flow {
            Some(_) => {
                res.rsp_code = -1;
                res.rsp_msg = "流程已存在".into();
                return res;
            }
            _ => {}
        }
    }

    let flow_data = Flow {
        id: None,
        name,
        create_time: get_current_time_fmt(),
        update_time: get_current_time_fmt(),
        shell_str,
    };

    let insert_flow_res: ExecResult = Flow::insert(&_data.db, &flow_data)
        .await
        .expect("创建流程失败");
    // 检测关系是否存在

    if let Value::U64(id) = insert_flow_res.last_insert_id {
        let project_flow_res = ProjectFlow::select_by_flow_id(&_data.db, &id.to_string())
            .await
            .expect("获取项目和流程关系失败");
        match project_flow_res {
            // 存在关系
            Some(pf) => {
                if pf.project_id.eq(&_req.project_id) {
                    res.rsp_code = -1;
                    res.rsp_msg = "该项目下流程已存在".into();
                    return res;
                }
            }
            _ => {
                let pf = ProjectFlow {
                    id: None,
                    project_id: _req.project_id,
                    flow_id: id as i16,
                };
                ProjectFlow::insert(&_data.db, &pf)
                    .await
                    .expect("创建项目-流程关系失败");
            }
        }
    } else {
        res.rsp_msg = "流程创建失败".into();
        return res;
    }

    res.rsp_msg = "流程创建成功".into();
    res
}

#[get("/get_detail")]
async fn get_detail(_req: web::Query<IdReq>, _data: web::Data<DataStore>) -> impl Responder {
    let data = Flow::select_by_id(&_data.db, &_req.id.to_string())
        .await
        .expect("flow不存在");
    return ResponseBody {
        rsp_code: 0,
        rsp_msg: "".to_string(),
        data,
    };
}

async fn check_flow_in_db(db: &RBatis, id: &str) -> HasFlowInDb<ResponseBody<Option<Flow>>> {
    let db_flow: Option<Flow> = Flow::select_by_id(db, id).await.expect("flow不存在");

    match db_flow {
        Some(_) => HasFlowInDb::Has(ResponseBody {
            rsp_code: -1,
            rsp_msg: "查询flow失败".to_string(),
            data: db_flow,
        }),
        _ => HasFlowInDb::None(ResponseBody {
            rsp_code: -1,
            rsp_msg: "查询flow失败".to_string(),
            data: None,
        }),
    }
}

#[delete("/delete_flow")]
pub async fn delete_flow(_req: web::Json<IdReq>, _data: web::Data<DataStore>) -> impl Responder {
    let mut res: ResponseBody<Option<Flow>> = ResponseBody {
        rsp_code: 0,
        rsp_msg: "".to_string(),
        data: None,
    };

    match check_flow_in_db(&_data.db, &_req.id.to_string()).await {
        HasFlowInDb::None(check_res) => {
            return check_res;
        }
        HasFlowInDb::Has(db_flow) => {
            let flow_data = db_flow.data.unwrap();
            let project: Project = get_project_by_flow_id(&_data.db, flow_data.id.unwrap()).await;
            let work_space =
                ShellUtil::check_work_space(_data.work_space.clone(), project.name, flow_data.name);
            if let Err(e) = ShellUtil::del_work_space(work_space) {
                panic!("{}", e.to_string());
            }
        }
    }

    let _ = Flow::delete_by_column(&_data.db, "id", &_req.id)
        .await
        .expect("flow删除失败");
    res.rsp_msg = "flow删除成功".to_string();

    res
}

async fn get_project_by_flow_id(db: &RBatis, id: i16) -> Project {
    let project: Project = db.query_decode("select project.* from project left join job_flow.project_flow pf on project.id = pf.project_id where pf.flow_id = ?", vec![to_value!(id)]).await.expect("查询项目失败");
    project
}

#[post("/update_flow")]
pub async fn update_flow(
    _req: web::Json<UpdateFLowReq>,
    _data: web::Data<DataStore>,
) -> impl Responder {
    let mut res: ResponseBody<Option<Flow>> = ResponseBody {
        rsp_code: 0,
        rsp_msg: "".to_string(),
        data: None,
    };

    match check_flow_in_db(&_data.db, &_req.id.to_string()).await {
        HasFlowInDb::None(check_res) => {
            return check_res;
        }
        HasFlowInDb::Has(db_flow) => {
            let table = Flow {
                id: Some(_req.id as i16),
                name: _req.flow_name.clone(),
                shell_str: _req.shell_str.clone(),
                create_time: db_flow.data.unwrap().create_time,
                update_time: get_current_time_fmt(),
            };
            let _ = Flow::update_by_column(&_data.db, &table, "id")
                .await
                .expect("更新流程失败");
            res.rsp_msg = "更新成功".to_string();
        }
    };
    res
}

#[get("/execute")]
async fn execute(_req: web::Query<IdReq>, app_data: web::Data<DataStore>) -> impl Responder {
    // let mut map = app_data.executing_child.lock().await;

    let flow_data: Flow = Flow::select_by_id(&app_data.db, &_req.id.to_string())
        .await
        .expect("流程查询失败")
        .unwrap();

    let project: Project = get_project_by_flow_id(&app_data.db, flow_data.id.unwrap()).await;

    let work_space =
        ShellUtil::check_work_space(app_data.work_space.clone(), project.name, flow_data.name);
    let mut cd_shell = format!("cd {} \n", work_space);
    cd_shell.push_str(&flow_data.shell_str);

    // 从cache中检测是否已经有执行的任务

    // 创建输出流
    let (sender, receiver) = mpsc::channel(10);
    let despatch = Despatch::default().start();


    
    let res = despatch
        .send(ShellExecute {
            shell_string: cd_shell,
        })
        .await;

    let res = match res {
        Ok(cmd_res) => cmd_res,
        _ => "".to_string(),
    };

    sender.send(Ok(res)).await.unwrap();

    // let sender_stdout = sender.clone();
    // let sender_stderr = sender.clone();
    // let out = sender.clone();
    // match map.get_mut(&flow_data.id.unwrap()) {
    //     Some(tk) => {
    //         if let Some(child) = tk {
    //             let _ = (*child).kill().await;
    //         }
    //     }
    //     _ => {
    //         let mut child = ShellUtil::spawn_new_command(cd_shell);
    //         // 拿去标准输出和标准错误输出
    //         let (stdout_reader, stderr_reader) = ShellUtil::get_std_reader(&mut child);
    //         // 创建流读取器
    //         let mut lines = stdout_reader.lines();
    //         while let Some(mut line) = lines.next_line().await.unwrap() {
    //             line.push_str("\n");
    //             sender_stdout.send(Ok(line)).await.unwrap();
    //         }

    //         let mut lines = stderr_reader.lines();
    //         while let Some(mut line) = lines.next_line().await.unwrap() {
    //             line.push_str("\n");
    //             sender_stderr.send(Ok(line)).await.unwrap();
    //         }

    //         match child.wait().await {
    //             Ok(status) => out
    //                 .send(Ok(format!("{}", status.to_string())))
    //                 .await
    //                 .unwrap(),
    //             Err(e) => println!("Failed to wait for child process: {}", e),
    //         }
    //         // TODO: 插值
    //         // map.insert(flow_data.id.unwrap(), Some(child));
    //     }
    // };

    HttpResponse::Ok()
        .content_type("text/event-stream")
        .streaming(LineStream { receiver })
}
