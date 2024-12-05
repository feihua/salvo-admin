use rbatis::rbdc::db::ExecResult;
use rbatis::rbdc::Error;
use serde::Serialize;
use std::fmt::Debug;

// 统一返回vo
#[derive(Serialize, Debug, Clone)]
pub struct BaseResponse<T>
where
    T: Serialize + Debug,
{
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> BaseResponse<T>
where
    T: Serialize + Debug,
{
    // 处理统一返回
    pub fn handle_result(result: Result<ExecResult, Error>) -> BaseResponse<String> {
        match result {
            Ok(_u) => Self::ok_result(),
            Err(err) => Self::err_result_msg(err.to_string()),
        }
    }

    pub fn ok_result() -> BaseResponse<String> {
        BaseResponse {
            msg: "操作成功".to_string(),
            code: 0,
            data: Some("None".to_string()),
        }
    }

    pub fn ok_result_msg(msg: String) -> BaseResponse<String> {
        BaseResponse {
            msg: msg.to_string(),
            code: 0,
            data: Some("None".to_string()),
        }
    }

    pub fn ok_result_code(code: i32, msg: String) -> BaseResponse<String> {
        BaseResponse {
            msg: msg.to_string(),
            code,
            data: Some("None".to_string()),
        }
    }

    pub fn ok_result_data(data: T) -> BaseResponse<T> {
        BaseResponse {
            msg: "操作成功".to_string(),
            code: 0,
            data: Some(data),
        }
    }

    pub fn err_result_msg(msg: String) -> BaseResponse<String> {
        BaseResponse {
            msg: msg.to_string(),
            code: 1,
            data: Some("None".to_string()),
        }
    }

    pub fn err_result_code(code: i32, msg: String) -> BaseResponse<String> {
        BaseResponse {
            msg: msg.to_string(),
            code,
            data: Some("None".to_string()),
        }
    }
}
