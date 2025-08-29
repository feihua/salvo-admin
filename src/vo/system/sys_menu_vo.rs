// author：刘飞华
// createTime：2024/12/12 14:41:44

use crate::common::result::serialize_datetime;
use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};

/*
添加菜单信息请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMenuReq {
    pub menu_name: String,         //菜单名称
    pub menu_type: i8,             //菜单类型(1：目录   2：菜单   3：按钮)
    pub visible: i8,               //菜单状态（0:隐藏, 显示:1）
    pub status: i8,                //状态(1:正常，0:禁用)
    pub sort: i32,                 //排序
    pub parent_id: Option<i64>,    //父ID
    pub menu_url: Option<String>,  //路由路径
    pub api_url: Option<String>,   //接口URL
    pub menu_icon: Option<String>, //菜单图标
    pub remark: Option<String>,    //备注
}

/*
删除菜单信息请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteMenuReq {
    pub id: i64,
}

/*
更新菜单信息请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMenuReq {
    pub id: i64,                   //主键
    pub menu_name: String,         //菜单名称
    pub menu_type: i8,             //菜单类型(1：目录   2：菜单   3：按钮)
    pub visible: i8,               //菜单状态（0:隐藏, 显示:1）
    pub status: i8,                //状态(1:正常，0:禁用)
    pub sort: i32,                 //排序
    pub parent_id: i64,            //父ID
    pub menu_url: Option<String>,  //路由路径
    pub api_url: Option<String>,   //接口URL
    pub menu_icon: Option<String>, //菜单图标
    pub remark: Option<String>,    //备注
}

/*
更新菜单信息状态请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMenuStatusReq {
    pub ids: Vec<i64>,
    pub status: i8,
}

/*
查询菜单信息详情请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryMenuDetailReq {
    pub id: i64,
}

/*
查询菜单信息详情响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryMenuDetailResp {
    pub id: i64,             //主键
    pub menu_name: String,   //菜单名称
    pub menu_type: i8,       //菜单类型(1：目录   2：菜单   3：按钮)
    pub visible: i8,         //菜单状态（0:隐藏, 显示:1）
    pub status: i8,          //状态(1:正常，0:禁用)
    pub sort: i32,           //排序
    pub parent_id: i64,      //父ID
    pub menu_url: String,    //路由路径
    pub api_url: String,     //接口URL
    pub menu_icon: String,   //菜单图标
    pub remark: String,      //备注
    #[serde(serialize_with = "serialize_datetime")]
    pub create_time: Option<DateTime>, //创建时间
    #[serde(serialize_with = "serialize_datetime")]
    pub update_time: Option<DateTime>, //修改时间
}

/*
查询菜单信息列表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryMenuListReq {
    pub menu_name: Option<String>, //菜单名称
}

/*
查询菜单信息列表响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuListDataResp {
    pub id: i64,             //主键
    pub menu_name: String,   //菜单名称
    pub menu_type: i8,       //菜单类型(1：目录   2：菜单   3：按钮)
    pub visible: i8,         //菜单状态（0:隐藏, 显示:1）
    pub status: i8,          //状态(1:正常，0:禁用)
    pub sort: i32,           //排序
    pub parent_id: i64,      //父ID
    pub menu_url: String,    //路由路径
    pub api_url: String,     //接口URL
    pub menu_icon: String,   //菜单图标
    pub remark: String,      //备注
    #[serde(serialize_with = "serialize_datetime")]
    pub create_time: Option<DateTime>, //创建时间
    #[serde(serialize_with = "serialize_datetime")]
    pub update_time: Option<DateTime>, //修改时间
}

/*
查询菜单信息列表响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuListSimpleDataResp {
    pub id: i64,           //主键
    pub menu_name: String, //菜单名称
    pub parent_id: i64,    //父ID
}
