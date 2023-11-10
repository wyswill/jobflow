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
