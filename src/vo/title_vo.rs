use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TitleSaveReq {
    pub id: i32,
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
    pub id: Option<i32>,
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
    pub id: Option<i32>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub interview_type: Option<String>,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct TitleListData {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
    pub interview_type: String,
    pub create_time: String,
    pub update_time: String,

}
