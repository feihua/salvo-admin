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

impl_select_page!(Member{select_page_by_name(phone:&str, name:&str, level:&str) =>"
        where 1=1
     if phone != null && phone != '':
        ` and phone= #{phone}`
     if name != null && name != '':
        ` and name= #{name}`
     if level != null && level != '':
        ` and level= #{level}` "
},"interview_member");