use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Member {
    pub id: Option<i32>,
    pub phone: String,
    pub name: String,
    pub password: String,
    pub level: String,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,

}

rbatis::crud!(Member {},"interview_member");
impl_select_page!(Member{select_page() =>"
     if !sql.contains('count'):
       order by create_time desc"
},"interview_member");

impl_select_page!(Member{select_page_by_name(name:&str) =>"
     if name != null && name != '':
       where real_name != #{name}
     if name == '':
       where real_name != ''"
},"interview_member");