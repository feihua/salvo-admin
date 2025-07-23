use crate::common::error::AppResult;
use salvo::prelude::Json;
use salvo::Response;
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
pub struct ResponsePage<T> {
    pub code: i32,
    pub msg: &'static str,
    pub total: u64,
    pub success: bool,
    pub data: Option<T>,
}

pub fn ok_result(res: &mut Response) -> AppResult<()> {
    ok_result_msg(res, "操作成功".to_string())
}

pub fn ok_result_msg(res: &mut Response, msg: String) -> AppResult<()> {
    let response = BaseResponse {
        msg,
        code: 0,
        data: Some("None".to_string()),
    };
    res.render(Json(response));
    Ok(())
}

pub fn ok_result_data<T: Serialize + Send>(res: &mut Response, data: T) -> AppResult<()> {
    let response = BaseResponse {
        msg: "操作成功".to_string(),
        code: 0,
        data: Some(data),
    };
    res.render(Json(response));
    Ok(())
}

pub fn err_result_msg(res: &mut Response, msg: String) -> AppResult<()> {
    let resp = BaseResponse {
        msg,
        code: 1,
        data: Some("None".to_string()),
    };
    res.render(Json(resp));
    Ok(())
}

pub fn ok_result_page<T: Serialize + Send>(res: &mut Response, data: T, total: u64) -> AppResult<()> {
    let page = ResponsePage {
        msg: "操作成功",
        code: 0,
        success: true,
        data: Some(data),
        total,
    };
    res.render(Json(page));
    Ok(())
}
