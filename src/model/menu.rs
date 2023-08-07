use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysMenu {
    pub  id: Option<i32>,
    pub  create_time: Option<DateTime>,
    pub  update_time: Option<DateTime>,
    pub  status_id: Option<i32>,
    pub  sort: Option<i32>,
    pub  parent_id: Option<i32>,
    pub  menu_name: Option<String>,
    pub  menu_url: Option<String>,
    pub  api_url: Option<String>,
    pub  menu_icon: Option<String>,
    pub  remark: Option<String>,
    pub  menu_type: Option<i32>,

}

rbatis::crud!(SysMenu {});
impl_select_page!(SysMenu{select_page() =>"
     if !sql.contains('count'):
       order by create_time asc"});

impl_select_page!(SysMenu{select_page_by_name(name:&str) =>"
     if name != null && name != '':
       where user_name != #{name}
     if name == '':
        where user_name != ''"});