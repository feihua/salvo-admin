use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleListReq {
    #[serde(rename = "current")]
    pub page_no: u64,
    #[serde(rename = "pageSize")]
    pub page_size: u64,
    pub role_name: Option<String>,
    pub status_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleListData {
    pub id: i32,
    pub sort: i32,
    pub status_id: i32,
    pub role_name: String,
    pub remark: String,
    pub create_time: String,
    pub update_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleSaveReq {
    pub role_name: String,
    pub sort: i32,
    pub status_id: i32,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleUpdateReq {
    pub id: i32,
    pub sort: i32,
    pub status_id: i32,
    pub role_name: String,
    pub remark: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct RoleDeleteReq {
    pub ids: Vec<i32>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRoleMenuReq {
    pub role_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRoleMenuData {
    pub role_menus: Vec<i32>,
    pub menu_list: Vec<MenuDataList>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuDataList {
    pub id: i32,
    pub parent_id: i32,
    pub title: String,
    pub key: String,
    pub label: String,
    #[serde(rename = "isPenultimate")]
    pub is_penultimate: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoleMenuReq {
    pub menu_ids: Vec<i32>,
    pub role_id: i32,
}


