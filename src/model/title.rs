use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Title {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
    pub interview_type: String,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,

}

rbatis::crud!(Title {},"interview_title");
impl_select_page!(Title{select_page() =>"
     if !sql.contains('count'):
       order by create_time desc"
},"interview_title");

impl_select_page!(Title{select_page_by_name(name:&str) =>"
     if name != null && name != '':
       where real_name != #{name}
     if name == '':
       where real_name != ''"
},"interview_title");