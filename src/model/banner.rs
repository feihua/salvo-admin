use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Banner {
    pub id: Option<i32>,
    pub title: String,
    pub image_url: String,
    pub webview_url: String,
    pub banner_sort: i32,
    pub banner_status: i32,
    pub remark: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,

}

rbatis::crud!(Banner {},"interview_banner");
impl_select_page!(Banner{select_page() =>"
     if !sql.contains('count'):
       order by create_time desc"
},"interview_banner");

impl_select_page!(Banner{select_page_by_name(name:&str) =>"
     if name != null && name != '':
       where user_name != #{name}
     if name == '':
       where user_name != ''"
},"interview_banner");