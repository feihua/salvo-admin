use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Types {
    pub id: Option<i32>,
    pub interview_code: String,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,

}

rbatis::crud!(Types {},"interview_types");
impl_select_page!(Types{select_page() =>"
     if !sql.contains('count'):
       order by create_time desc"
},"interview_types");

impl_select_page!(Types{select_page_by_name(name:&str) =>"
     if name != null && name != '':
       where real_name != #{name}
     if name == '':
       where real_name != ''"
},"interview_types");