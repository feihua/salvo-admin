use crate::common::error::{AppResult, AppResultPage};
use rbatis::rbdc::DateTime;
use salvo::writing::Json;
use serde::Serialize;
use std::fmt::Debug;

// 统一返回vo
#[derive(Serialize, Debug, Clone)]
pub struct BaseResponse<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

#[derive(Serialize, Debug, Clone)]
pub struct PageResult<T> {
    pub list: Vec<T>,
    pub total: u64,
}

pub fn ok_result() -> AppResult<String> {
    ok_result_msg("操作成功".to_string())
}

pub fn ok_result_msg(msg: String) -> AppResult<String> {
    let response = BaseResponse {
        msg,
        code: 0,
        data: Some("None".to_string()),
    };
    Ok(Json(response))
}

pub fn ok_result_data<T: Serialize + Send>(data: T) -> AppResult<T> {
    let response = BaseResponse {
        msg: "操作成功".to_string(),
        code: 0,
        data: Some(data),
    };
    Ok(Json(response))
}

pub fn err_result_msg(msg: String) -> AppResult<String> {
    let response = BaseResponse {
        msg,
        code: 1,
        data: Some("None".to_string()),
    };
    Ok(Json(response))
}

pub fn ok_result_page<T: Serialize + Send>(list: Vec<T>, total: u64) -> AppResultPage<T> {
    Ok(Json(BaseResponse {
        msg: "操作成功".to_string(),
        code: 0,
        data: Some(PageResult { list, total }),
    }))
}

pub fn serialize_datetime<S>(dt: &Option<DateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match dt {
        Some(datetime) => {
            let formatted = datetime.format("YYYY-MM-DD hh:mm:ss");
            serializer.serialize_str(&formatted)
        }
        None => serializer.serialize_str(""),
    }
}
