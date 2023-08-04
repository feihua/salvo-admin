use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TitleSaveReq {
    pub title: String,
    pub content: String,
    pub interview_type: String,


}

#[derive(Debug, Serialize, Deserialize)]
pub struct TitleDeleteReq {
    pub ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TitleUpdateReq {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub interview_type: String,


}

#[derive(Debug, Serialize, Deserialize)]
pub struct TitleListReq {
    #[serde(rename = "current")]
    pub page_no: u64,
    #[serde(rename = "pageSize")]
    pub page_size: u64,


}

#[derive(Debug, Serialize, Deserialize)]
pub struct TitleListResp {
    pub msg: String,
    pub code: i32,
    pub success: bool,
    pub total: u64,
    pub data: Option<Vec<TitleListData>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TitleListData {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub interview_type: String,
    pub create_time: String,
    pub update_time: String,

}
