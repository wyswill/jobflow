use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct CreateProjectBody {
    pub name: String,
}
