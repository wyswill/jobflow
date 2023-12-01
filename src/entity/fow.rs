use rbatis::{crud, impl_select, impl_select_page};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Flow {
    pub id: Option<i16>,
    pub name: String,
    pub create_time: String,
    pub update_time: String,
    pub shell_str: String,
}
crud!(Flow {}, "flow");

impl_select_page!(Flow{select_page_by_name(name: &str) => "` where name = #{name} order by create_time desc`" } );
impl_select!(Flow{select_by_name(name:&str) -> Option => "`where name = #{name} `"});
impl_select!(Flow{select_by_id(id:&str) -> Option => "`where id = #{id}`"});
