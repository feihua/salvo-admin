// Code generated by https://github.com/feihua/code_cli
// author：刘飞华
// date：2025/01/08 13:51:14

use crate::common::error::AppResult;
use crate::common::result::BaseResponse;
use crate::model::system::sys_post_model::Post;
use crate::model::system::sys_user_post_model::count_user_post_by_id;
use crate::utils::time_util::time_to_string;
use crate::vo::system::sys_post_vo::*;
use crate::RB;
use rbatis::plugin::page::PageRequest;
use rbs::to_value;
use salvo::prelude::*;
use salvo::{Request, Response};

/*
 *添加岗位信息表
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn add_sys_post(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let item = req.parse_json::<AddPostReq>().await?;
    log::info!("add sys_post params: {:?}", &item);

    let rb = &mut RB.clone();
    if Post::select_by_name(rb, &item.post_name).await?.is_some() {
        return BaseResponse::<String>::err_result_msg(res, "新增岗位失败,岗位名称已存在");
    }

    if Post::select_by_code(rb, &item.post_code).await?.is_some() {
        return BaseResponse::<String>::err_result_msg(res, "新增岗位失败,岗位编码已存在");
    }

    let sys_post = Post {
        id: None,                                //岗位id
        post_code: item.post_code,               //岗位编码
        post_name: item.post_name,               //岗位名称
        sort: item.sort,                         //显示顺序
        status: item.status,                     //部状态（0：停用，1:正常）
        remark: item.remark.unwrap_or_default(), //备注
        create_time: None,                       //创建时间
        update_time: None,                       //更新时间
    };

    Post::insert(rb, &sys_post).await?;
    BaseResponse::<String>::ok_result(res)
}

/*
 *删除岗位信息表
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn delete_sys_post(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let item = req.parse_json::<DeletePostReq>().await?;
    log::info!("delete sys_post params: {:?}", &item);

    let ids = item.ids.clone();
    let rb = &mut RB.clone();
    for id in ids {
        match Post::select_by_id(rb, &id).await? {
            None => return BaseResponse::<String>::err_result_msg(res, "岗位不存在,不能删除"),
            Some(x) => {
                if count_user_post_by_id(rb, id).await? > 0 {
                    let msg = format!("{}已分配,不能删除", x.post_name);
                    return BaseResponse::<String>::err_result_msg(res, msg.as_str());
                }
            }
        };
    }

    Post::delete_in_column(rb, "id", &item.ids).await?;
    BaseResponse::<String>::ok_result(res)
}

/*
 *更新岗位信息表
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn update_sys_post(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let item = req.parse_json::<UpdatePostReq>().await?;
    log::info!("update sys_post params: {:?}", &item);

    let rb = &mut RB.clone();

    if Post::select_by_id(rb, &item.id).await?.is_none() {
        return BaseResponse::<String>::err_result_msg(res, "更新岗位失败,岗位不存在");
    }

    if let Some(x) = Post::select_by_name(rb, &item.post_name).await? {
        if x.id.unwrap_or_default() != item.id {
            return BaseResponse::<String>::err_result_msg(res, "更新岗位失败,岗位名称已存在");
        }
    }

    if let Some(x) = Post::select_by_code(rb, &item.post_code).await? {
        if x.id.unwrap_or_default() != item.id {
            return BaseResponse::<String>::err_result_msg(res, "更新岗位失败,岗位编码已存在");
        }
    }

    let sys_post = Post {
        id: Some(item.id),                       //岗位id
        post_code: item.post_code,               //岗位编码
        post_name: item.post_name,               //岗位名称
        sort: item.sort,                         //显示顺序
        status: item.status,                     //部状态（0：停用，1:正常）
        remark: item.remark.unwrap_or_default(), //备注
        create_time: None,                       //创建时间
        update_time: None,                       //更新时间
    };

    Post::update_by_column(rb, &sys_post, "id").await?;
    BaseResponse::<String>::ok_result(res)
}

/*
 *更新岗位信息表状态
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn update_sys_post_status(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let item = req.parse_json::<UpdatePostStatusReq>().await?;
    log::info!("update sys_post_status params: {:?}", &item);

    let update_sql = format!(
        "update sys_post set status = ? where id in ({})",
        item.ids
            .iter()
            .map(|_| "?")
            .collect::<Vec<&str>>()
            .join(", ")
    );

    let mut param = vec![to_value!(item.status)];
    param.extend(item.ids.iter().map(|&id| to_value!(id)));

    let _ = &mut RB.clone().exec(&update_sql, param).await?;
    BaseResponse::<String>::ok_result(res)
}

/*
 *查询岗位信息表详情
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn query_sys_post_detail(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let item = req.parse_json::<QueryPostDetailReq>().await?;
    log::info!("query sys_post_detail params: {:?}", &item);

    let rb = &mut RB.clone();
    match Post::select_by_id(rb, &item.id).await? {
        Some(x) => {
            let sys_post = QueryPostDetailResp {
                id: x.id.unwrap_or_default(),               //岗位id
                post_code: x.post_code,                     //岗位编码
                post_name: x.post_name,                     //岗位名称
                sort: x.sort,                               //显示顺序
                status: x.status,                           //部状态（0：停用，1:正常）
                remark: x.remark,                           //备注
                create_time: time_to_string(x.create_time), //创建时间
                update_time: time_to_string(x.update_time), //更新时间
            };

            BaseResponse::<QueryPostDetailResp>::ok_result_data(res, sys_post)
        }
        None => BaseResponse::<QueryPostDetailResp>::err_result_data(
            res,
            QueryPostDetailResp::new(),
            "岗位不存在",
        ),
    }
}

/*
 *查询岗位信息表列表
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn query_sys_post_list(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let item = req.parse_json::<QueryPostListReq>().await?;
    log::info!("query sys_post_list params: {:?}", &item);

    let post_code = item.post_code.as_deref().unwrap_or_default(); //岗位编码
    let post_name = item.post_name.as_deref().unwrap_or_default(); //岗位名称
    let status = item.status.unwrap_or(2); //部状态（0：停用，1:正常）

    let page = &PageRequest::new(item.page_no, item.page_size);
    let rb = &mut RB.clone();
    let d = Post::select_post_list(rb, page, post_code, post_name, status).await?;

    let mut list: Vec<PostListDataResp> = Vec::new();

    let total = d.total;

    for x in d.records {
        list.push(PostListDataResp {
            id: x.id.unwrap_or_default(),               //岗位id
            post_code: x.post_code,                     //岗位编码
            post_name: x.post_name,                     //岗位名称
            sort: x.sort,                               //显示顺序
            status: x.status,                           //部状态（0：停用，1:正常）
            remark: x.remark,                           //备注
            create_time: time_to_string(x.create_time), //创建时间
            update_time: time_to_string(x.update_time), //更新时间
        })
    }

    BaseResponse::<Vec<PostListDataResp>>::ok_result_page(res, list, total)
}
