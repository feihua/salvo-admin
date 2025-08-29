use rbatis::rbdc::DateTime;

pub mod sys_dept_vo;
pub mod sys_dict_data_vo;
pub mod sys_dict_type_vo;
pub mod sys_login_log_vo;
pub mod sys_menu_vo;
pub mod sys_notice_vo;
pub mod sys_operate_log_vo;
pub mod sys_post_vo;
pub mod sys_role_vo;
pub mod sys_user_vo;


pub fn serialize_datetime<S>(dt: &Option<DateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match dt {
        Some(datetime) => {
            let formatted = datetime.format("YYYY-MM-DD hh:mm:ss");
            serializer.serialize_str(&formatted)
        }
        None => serializer.serialize_none(),
    }
}