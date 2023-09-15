use rbatis::{crud, impl_select};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: Option<i16>,
    pub flow_id: Option<i16>,
    pub name: String,
    pub create_time: String,
    pub update_time: String,
}

impl_select!(Project{select_by_id(id:&str) -> Option => "`where id = #{id} limit 1`"});
impl_select!(Project{select_by_name(name:&str) -> Option => "`where name = #{name} limit 1`"});

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Flow {
    pub id: Option<i16>,
    pub name: String,
    pub create_time: String,
    pub update_time: String,
    pub shell_str: String,
}

crud!(Project {}, "project");
crud!(Flow {}, "flow");
