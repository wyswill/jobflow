use rbatis::{crud, impl_select, impl_select_page};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectFlow {
    pub id: Option<i16>,
    pub project_id: i16,
    pub flow_id: i16,
}
crud!(ProjectFlow {}, "project_flow");
impl_select!(ProjectFlow{select_by_id(id:&str) -> Option => "`where id = #{id}`"});
impl_select_page!(ProjectFlow{select_by_project_id(id:usize) => "`where project_id = #{id}`"});
impl_select!(ProjectFlow{select_by_flow_id(id: &str) -> Option => "`where flow_id = #{id}`"});
