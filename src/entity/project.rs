use rbatis::{crud, impl_select, impl_select_page};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: Option<i16>,
    pub name: String,
    pub create_time: String,
    pub update_time: String,
}

crud!(Project {}, "project");

impl_select!(Project{select_by_id(id:&str) -> Option => "`where id = #{id}`"});
impl_select!(Project{select_by_name(name:&str) -> Option => "`where name = #{name} `"});
impl_select_page!(Project{select_page() => "`order by create_time desc`" } );