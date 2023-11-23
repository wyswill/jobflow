use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct CreateProjectBody {
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FlowPageQuery {
    pub offset: usize,
    pub size: usize,
    pub project_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PageQuery {
    pub offset: usize,
    pub size: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateFlowReq {
    pub project_id: i16,
    pub flow_name: String,
    pub shell_str: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct IdReq {
    pub id: usize,
}
#[derive(Debug, Deserialize, Clone)]
pub struct WsData {
    pub project_name: String,
    pub flow_name: String,
}
