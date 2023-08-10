use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TypesSaveReq {
    pub id: i32,
    pub interview_code: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypesDeleteReq {
    pub ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypesUpdateReq {
    pub id: Option<i32>,
    pub interview_code: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypesListReq {
    #[serde(rename = "current")]
    pub page_no: u64,
    #[serde(rename = "pageSize")]
    pub page_size: u64,
    pub id: Option<i32>,
    pub interview_code: Option<String>,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypesListData {
    pub id: Option<i32>,
    pub interview_code: String,
    pub create_time: String,
    pub update_time: String,

}
