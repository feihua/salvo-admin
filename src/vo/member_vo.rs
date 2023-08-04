use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberSaveReq {
    pub phone: String,
    pub name: String,
    pub password: String,
    pub level: String

}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberDeleteReq {
    pub ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberUpdateReq {
    pub id: i32,
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

}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberListResp {
    pub msg: String,
    pub code: i32,
    pub success: bool,
    pub total: u64,
    pub data: Option<Vec<MemberListData>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberListData {
    pub id: i32,
    pub phone: String,
    pub name: String,
    pub password: String,
    pub level: String,
    pub create_time: String,
    pub update_time: String,

}
