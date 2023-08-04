use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeSaveReq {
    pub interview_code: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeDeleteReq {
    pub ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeUpdateReq {
    pub id: i32,
    pub interview_code: String,


}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeListReq {
    #[serde(rename = "current")]
    pub page_no: u64,
    #[serde(rename = "pageSize")]
    pub page_size: u64,


}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeListResp {
    pub msg: String,
    pub code: i32,
    pub success: bool,
    pub total: u64,
    pub data: Option<Vec<TypeListData>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeListData {
    pub id: i32,
    pub interview_code: String,
    pub create_time: String,
    pub update_time: String,

}
