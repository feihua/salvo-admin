use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberSaveReq {
    pub phone: String,
    pub name: String,
    pub password: String,
    pub level: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberDeleteReq {
    pub ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberUpdateReq {
    pub id: Option<i32>,
    pub phone: String,
    pub name: String,
    pub password: String,
    pub level: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberListReq {
    #[serde(rename = "current")]
    pub page_no: u64,
    #[serde(rename = "pageSize")]
    pub page_size: u64,
    pub phone: Option<String>,
    pub name: Option<String>,
    pub level: Option<String>,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberListData {
    pub id: Option<i32>,
    pub phone: String,
    pub name: String,
    pub password: String,
    pub level: String,
    pub create_time: String,
    pub update_time: String,

}
