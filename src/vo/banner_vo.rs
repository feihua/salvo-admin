use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BannerSaveReq {
    pub id: i32,
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
    pub id: Option<i32>,
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
    pub id: Option<i32>,
    pub title: Option<String>,
    pub image_url: Option<String>,
    pub webview_url: Option<String>,
    pub banner_sort: Option<i32>,
    pub banner_status: Option<i32>,
    pub remark: Option<String>,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct BannerListData {
    pub id: Option<i32>,
    pub title: String,
    pub image_url: String,
    pub webview_url: String,
    pub banner_sort: i32,
    pub banner_status: i32,
    pub remark: String,
    pub create_time: String,
    pub update_time: String,

}
