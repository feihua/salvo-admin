// Code generated by https://github.com/feihua/code_cli
// author：刘飞华
// date：2025/01/08 13:51:14

use crate::common::error::{AppError, AppResult};
use crate::common::result::{ok_result, ok_result_data, ok_result_page};
use crate::model::system::sys_login_log_model::LoginLog;
use crate::utils::time_util::time_to_string;
use crate::vo::system::sys_login_log_vo::*;
use crate::RB;
use rbatis::plugin::page::PageRequest;
use rbs::value;
use salvo::prelude::*;
use salvo::{Request, Response};
/*
 *删除系统访问记录
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn delete_sys_login_log(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let item = req.parse_json::<DeleteLoginLogReq>().await?;
    log::info!("delete sys_login_log params: {:?}", &item);

    let rb = &mut RB.clone();

    LoginLog::delete_by_map(rb, value! {"id": &item.ids}).await?;
    ok_result(res)
}

/*
 *查询系统访问记录详情
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn query_sys_login_log_detail(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let item = req.parse_json::<QueryLoginLogDetailReq>().await?;
    log::info!("query sys_login_log_detail params: {:?}", &item);

    match LoginLog::select_by_id(&mut RB.clone(), &item.id).await? {
        None => Err(AppError::BusinessError("系统访问记录不存在")),
        Some(x) => {
            let sys_login_log = QueryLoginLogDetailResp {
                id: x.id.unwrap(),                        //访问ID
                login_name: x.login_name,                 //登录账号
                ipaddr: x.ipaddr,                         //登录IP地址
                login_location: x.login_location,         //登录地点
                platform: x.platform,                     //平台信息
                browser: x.browser,                       //浏览器类型
                version: x.version,                       //浏览器版本
                os: x.os,                                 //操作系统
                arch: x.arch,                             //体系结构信息
                engine: x.engine,                         //渲染引擎信息
                engine_details: x.engine_details,         //渲染引擎详细信息
                extra: x.extra,                           //其他信息（可选）
                status: x.status,                         //登录状态(0:失败,1:成功)
                msg: x.msg,                               //提示消息
                login_time: time_to_string(x.login_time), //访问时间
            };

            ok_result_data(res, sys_login_log)
        }
    }
}

/*
 *查询系统访问记录列表
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn query_sys_login_log_list(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let item = req.parse_json::<QueryLoginLogListReq>().await?;
    log::info!("query sys_login_log_list params: {:?}", &item);

    let page = &PageRequest::new(item.page_no, item.page_size);

    let page_info = LoginLog::select_login_log_list(&mut RB.clone(), page, &item).await?;

    let list = page_info
        .records
        .into_iter()
        .map(|x| {
            LoginLogListDataResp {
                id: x.id.unwrap_or_default(),             //访问ID
                login_name: x.login_name,                 //登录账号
                ipaddr: x.ipaddr,                         //登录IP地址
                login_location: x.login_location,         //登录地点
                platform: x.platform,                     //平台信息
                browser: x.browser,                       //浏览器类型
                version: x.version,                       //浏览器版本
                os: x.os,                                 //操作系统
                arch: x.arch,                             //体系结构信息
                engine: x.engine,                         //渲染引擎信息
                engine_details: x.engine_details,         //渲染引擎详细信息
                extra: x.extra,                           //其他信息（可选）
                status: x.status,                         //登录状态(0:失败,1:成功)
                msg: x.msg,                               //提示消息
                login_time: time_to_string(x.login_time), //访问时间
            }
        })
        .collect::<Vec<LoginLogListDataResp>>();

    ok_result_page(res, list, page_info.total)
}
