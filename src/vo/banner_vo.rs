use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BannerSaveReq {
    pub title: String,
    pub image_url: String,
    pub webview_url: String,
    pub banner_sort: i32,
    pub banner_status: i32,
    pub remark: Option<String>,


}

#[derive(Debug, Serialize, Deserialize)]
pub struct BannerDeleteReq {
    pub ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BannerUpdateReq {
    pub id: i32,
    pub title: String,
    pub image_url: String,
    pub webview_url: String,
    pub banner_sort: i32,
    pub banner_status: i32,
    pub remark: Option<String>,


}

#[derive(Debug, Serialize, Deserialize)]
pub struct BannerListReq {
    #[serde(rename = "current")]
    pub page_no: u64,
    #[serde(rename = "pageSize")]
    pub page_size: u64,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct BannerListResp {
    pub msg: String,
    pub code: i32,
    pub success: bool,
    pub total: u64,
    pub data: Option<Vec<BannerListData>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BannerListData {
    pub id: i32,
    pub title: String,
    pub image_url: String,
    pub webview_url: String,
    pub banner_sort: i32,
    pub banner_status: i32,
    pub remark: String,
    pub create_time: String,
    pub update_time: String,

}
