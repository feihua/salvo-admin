use salvo::prelude::Json;
use salvo::Response;
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
    T: Serialize + Debug+ Send,
{
    pub fn ok_result(res: &mut Response) {
        res.render(Json(BaseResponse {
            msg: "操作成功".to_string(),
            code: 0,
            data: Some("None".to_string()),
        }))
    }

    pub fn ok_result_msg(res: &mut Response, msg: String) {
        res.render(Json(BaseResponse {
            msg: msg.to_string(),
            code: 0,
            data: Some("None".to_string()),
        }))
    }

    pub fn ok_result_code(res: &mut Response, code: i32, msg: String) {
        res.render(Json(BaseResponse {
            msg: msg.to_string(),
            code,
            data: Some("None".to_string()),
        }))
    }

    pub fn ok_result_data(res: &mut Response, data: T) {
        res.render(Json(BaseResponse {
            msg: "操作成功".to_string(),
            code: 0,
            data: Some(data),
        }))
    }

    pub fn err_result_msg(res: &mut Response, msg: String) {
        res.render(Json(BaseResponse {
            msg: msg.to_string(),
            code: 1,
            data: Some("None".to_string()),
        }))
    }

    pub fn err_result_code(res: &mut Response, code: i32, msg: String) {
        res.render(Json(BaseResponse {
            msg: msg.to_string(),
            code,
            data: Some("None".to_string()),
        }))
    }
}
