// Code generated by https://github.com/feihua/code_cli
// author：刘飞华
// date：2025/01/08 13:51:14

use rbatis::rbatis_codegen::ops::AsProxy;
use rbs::to_value;
use salvo::prelude::*;
use salvo::{Request, Response};

use crate::common::result::BaseResponse;
use crate::model::system::sys_dept_model::{
    check_dept_exist_user, select_children_dept_by_id, select_dept_count,
    select_normal_children_dept_by_id, Dept,
};
use crate::utils::time_util::time_to_string;
use crate::vo::system::sys_dept_vo::*;
use crate::RB;
/*
 *添加部门表
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn add_sys_dept(req: &mut Request, res: &mut Response) {
    match req.parse_json::<AddDeptReq>().await {
        Ok(item) => {
            log::info!("add sys_dept params: {:?}", &item);

            let rb = &mut RB.clone();
            let res_dept = Dept::select_by_dept_name(rb, &item.dept_name, item.parent_id).await;
            match res_dept {
                Ok(r) => {
                    if r.is_some() {
                        return BaseResponse::<String>::err_result_msg(
                            res,
                            "部门名称已存在".to_string(),
                        );
                    }
                }
                Err(err) => return BaseResponse::<String>::err_result_msg(res, err.to_string()),
            }

            let res_dept = Dept::select_by_id(rb, &item.parent_id).await;
            let ancestors = match res_dept {
                Ok(r) => match r {
                    None => {
                        return BaseResponse::<String>::err_result_msg(
                            res,
                            "添加失败,上级部门不存在".to_string(),
                        )
                    }
                    Some(dept) => {
                        if dept.status == 0 {
                            return BaseResponse::<String>::err_result_msg(
                                res,
                                "部门停用，不允许添加".to_string(),
                            );
                        }
                        format!("{},{}", dept.ancestors, &item.parent_id)
                    }
                },
                Err(err) => return BaseResponse::<String>::err_result_msg(res, err.to_string()),
            };

            let sys_dept = Dept {
                id: None,                  //部门id
                parent_id: item.parent_id, //父部门id
                ancestors,                 //祖级列表
                dept_name: item.dept_name, //部门名称
                sort: item.sort,           //显示顺序
                leader: item.leader,       //负责人
                phone: item.phone,         //联系电话
                email: item.email,         //邮箱
                status: item.status,       //部状态（0：停用，1:正常）
                del_flag: None,            //删除标志（0代表删除 1代表存在）
                create_time: None,         //创建时间
                update_time: None,         //修改时间
            };

            let result = Dept::insert(rb, &sys_dept).await;

            match result {
                Ok(_u) => BaseResponse::<String>::ok_result(res),
                Err(err) => BaseResponse::<String>::err_result_msg(res, err.to_string()),
            }
        }
        Err(err) => {
            BaseResponse::<String>::err_result_msg(res, format!("解析请求参数失败: {}", err))
        }
    }
}

/*
 *删除部门表
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn delete_sys_dept(req: &mut Request, res: &mut Response) {
    match req.parse_json::<DeleteDeptReq>().await {
        Ok(item) => {
            log::info!("delete sys_dept params: {:?}", &item);

            let rb = &mut RB.clone();
            let res_dept = select_dept_count(rb, &item.id).await.unwrap_or_default();
            if res_dept > 0 {
                return BaseResponse::<String>::err_result_msg(
                    res,
                    "存在下级部门,不允许删除".to_string(),
                );
            }

            let res_dept = check_dept_exist_user(rb, &item.id)
                .await
                .unwrap_or_default();
            if res_dept > 0 {
                return BaseResponse::<String>::err_result_msg(
                    res,
                    "部门存在用户,不允许删除".to_string(),
                );
            }

            let result = Dept::delete_by_column(rb, "id", &item.id).await;

            match result {
                Ok(_u) => BaseResponse::<String>::ok_result(res),
                Err(err) => BaseResponse::<String>::err_result_msg(res, err.to_string()),
            }
        }
        Err(err) => {
            BaseResponse::<String>::err_result_msg(res, format!("解析请求参数失败: {}", err))
        }
    }
}

/*
 *更新部门表
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn update_sys_dept(req: &mut Request, res: &mut Response) {
    match req.parse_json::<UpdateDeptReq>().await {
        Ok(item) => {
            log::info!("update sys_dept params: {:?}", &item);

            if item.parent_id == item.id {
                return BaseResponse::<String>::err_result_msg(
                    res,
                    "上级部门不能是自己".to_string(),
                );
            }

            let rb = &mut RB.clone();
            let res_dept = Dept::select_by_id(rb, &item.id).await;
            let old_ancestors = match res_dept {
                Ok(r) => match r {
                    None => {
                        return BaseResponse::<String>::err_result_msg(
                            res,
                            "更新失败,部门不存在".to_string(),
                        )
                    }
                    Some(dept) => dept.ancestors,
                },
                Err(err) => return BaseResponse::<String>::err_result_msg(res, err.to_string()),
            };

            let res_dept = Dept::select_by_id(rb, &item.parent_id).await;
            let ancestors = match res_dept {
                Ok(r) => match r {
                    None => {
                        return BaseResponse::<String>::err_result_msg(
                            res,
                            "更新失败,上级部门不存在".to_string(),
                        )
                    }
                    Some(dept) => {
                        format!("{},{}", dept.ancestors, &item.parent_id)
                    }
                },
                Err(err) => return BaseResponse::<String>::err_result_msg(res, err.to_string()),
            };

            let res_dept = Dept::select_by_dept_name(rb, &item.dept_name, item.parent_id).await;
            match res_dept {
                Ok(r) => {
                    if r.is_some() && r.unwrap().id.unwrap_or_default() != item.id {
                        return BaseResponse::<String>::err_result_msg(
                            res,
                            "部门名称已存在".to_string(),
                        );
                    }
                }
                Err(err) => return BaseResponse::<String>::err_result_msg(res, err.to_string()),
            }

            let count = select_normal_children_dept_by_id(rb, &item.id)
                .await
                .unwrap_or_default();
            if count > 0 && item.status == 0 {
                return BaseResponse::<String>::err_result_msg(
                    res,
                    "该部门包含未停用的子部门".to_string(),
                );
            }

            let res_dept = select_children_dept_by_id(rb, &item.id).await;
            match res_dept {
                Ok(list) => {
                    let mut depts = vec![];
                    for mut x in list {
                        x.ancestors = x
                            .ancestors
                            .replace(old_ancestors.as_str(), ancestors.as_str());
                        depts.push(x)
                    }
                    let result =
                        Dept::update_by_column_batch(rb, &depts, "id", depts.len() as u64).await;
                    if result.is_err() {
                        return BaseResponse::<String>::err_result_msg(
                            res,
                            "修改下级部门祖级列失败".to_string(),
                        );
                    }
                }
                Err(err) => return BaseResponse::<String>::err_result_msg(res, err.to_string()),
            }

            let sys_dept = Dept {
                id: Some(item.id),            //部门id
                parent_id: item.parent_id,    //父部门id
                ancestors: ancestors.clone(), //祖级列表
                dept_name: item.dept_name,    //部门名称
                sort: item.sort,              //显示顺序
                leader: item.leader,          //负责人
                phone: item.phone,            //联系电话
                email: item.email,            //邮箱
                status: item.status,          //部状态（0：停用，1:正常）
                del_flag: None,               //删除标志（0代表删除 1代表存在）
                create_time: None,            //创建时间
                update_time: None,            //修改时间
            };

            let result = Dept::update_by_column(rb, &sys_dept, "id").await;

            if result.is_err() {
                return BaseResponse::<String>::err_result_msg(res, "更新部门失败".to_string());
            }

            // 如果该部门是启用状态，则启用该部门的所有上级部门
            if item.status == 1 && sys_dept.ancestors != "0" {
                let ids = ancestors.split(",").map(|s| s.i64()).collect::<Vec<i64>>();

                let update_sql = format!(
                    "update sys_dept set status = ? where id in ({})",
                    ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", ")
                );

                let mut param = vec![to_value!(item.status)];
                param.extend(ids.iter().map(|&id| to_value!(id)));
                let res_dept = &mut RB.clone().exec(&update_sql, param).await;

                match res_dept {
                    Ok(_u) => BaseResponse::<String>::ok_result(res),
                    Err(err) => BaseResponse::<String>::err_result_msg(res, err.to_string()),
                }
            } else {
                BaseResponse::<String>::ok_result(res)
            }
        }
        Err(err) => {
            BaseResponse::<String>::err_result_msg(res, format!("解析请求参数失败: {}", err))
        }
    }
}

/*
 *更新部门表状态
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn update_sys_dept_status(req: &mut Request, res: &mut Response) {
    match req.parse_json::<UpdateDeptStatusReq>().await {
        Ok(item) => {
            log::info!("update sys_dept_status params: {:?}", &item);

            let rb = &mut RB.clone();
            if item.status == 1 {
                for id in item.ids.clone() {
                    let result = Dept::select_by_id(rb, &id).await.unwrap_or_default();
                    if result.is_some() {
                        let ancestors = result.unwrap().ancestors;
                        let ids = ancestors.split(",").map(|s| s.i64()).collect::<Vec<i64>>();

                        let update_sql = format!(
                            "update sys_dept set status = ? where id in ({})",
                            ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", ")
                        );

                        let mut param = vec![to_value!(item.status)];
                        param.extend(ids.iter().map(|&id| to_value!(id)));
                        let res_dept = rb.exec(&update_sql, param).await;

                        match res_dept {
                            Err(err) => {
                                BaseResponse::<String>::err_result_msg(res, err.to_string())
                            }
                            _ => {}
                        }
                    }
                }
            }

            let update_sql = format!(
                "update sys_dept set status = ? where id in ({})",
                item.ids
                    .iter()
                    .map(|_| "?")
                    .collect::<Vec<&str>>()
                    .join(", ")
            );

            let mut param = vec![to_value!(item.status)];
            param.extend(item.ids.iter().map(|&id| to_value!(id)));
            let result = rb.exec(&update_sql, param).await;
            match result {
                Ok(_u) => BaseResponse::<String>::ok_result(res),
                Err(err) => BaseResponse::<String>::err_result_msg(res, err.to_string()),
            }
        }
        Err(err) => {
            BaseResponse::<String>::err_result_msg(res, format!("解析请求参数失败: {}", err))
        }
    }
}

/*
 *查询部门表详情
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn query_sys_dept_detail(req: &mut Request, res: &mut Response) {
    match req.parse_json::<QueryDeptDetailReq>().await {
        Ok(item) => {
            log::info!("query sys_dept_detail params: {:?}", &item);

            let rb = &mut RB.clone();
            let result = Dept::select_by_id(rb, &item.id).await;

            match result {
                Ok(d) => {
                    if d.is_none() {
                        return BaseResponse::<QueryDeptDetailResp>::err_result_data(
                            res,
                            QueryDeptDetailResp::new(),
                            "部门不存在".to_string(),
                        );
                    }
                    let x = d.unwrap();

                    let sys_dept = QueryDeptDetailResp {
                        id: x.id.unwrap_or_default(),               //部门id
                        parent_id: x.parent_id,                     //父部门id
                        ancestors: x.ancestors,                     //祖级列表
                        dept_name: x.dept_name,                     //部门名称
                        sort: x.sort,                               //显示顺序
                        leader: x.leader,                           //负责人
                        phone: x.phone,                             //联系电话
                        email: x.email,                             //邮箱
                        status: x.status,                           //部状态（0：停用，1:正常）
                        del_flag: x.del_flag.unwrap_or_default(), //删除标志（0代表删除 1代表存在）
                        create_time: time_to_string(x.create_time), //创建时间
                        update_time: time_to_string(x.update_time), //修改时间
                    };

                    BaseResponse::<QueryDeptDetailResp>::ok_result_data(res, sys_dept)
                }
                Err(err) => BaseResponse::<QueryDeptDetailResp>::err_result_data(
                    res,
                    QueryDeptDetailResp::new(),
                    err.to_string(),
                ),
            }
        }
        Err(err) => {
            BaseResponse::<String>::err_result_msg(res, format!("解析请求参数失败: {}", err))
        }
    }
}

/*
 *查询部门表列表
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn query_sys_dept_list(req: &mut Request, res: &mut Response) {
    match req.parse_json::<QueryDeptListReq>().await {
        Ok(item) => {
            log::info!("query sys_dept_list params: {:?}", &item);
            let dept_name = item.dept_name.as_deref().unwrap_or_default(); //部门名称
            let status = item.status.unwrap_or(2); //部状态（0：停用，1:正常）

            let rb = &mut RB.clone();
            let result = Dept::select_page_dept_list(rb, dept_name, status).await;

            match result {
                Ok(page) => {
                    let list = page
                        .into_iter()
                        .map(|x| {
                            DeptListDataResp {
                                id: x.id.unwrap_or_default(),               //部门id
                                parent_id: x.parent_id,                     //父部门id
                                ancestors: x.ancestors,                     //祖级列表
                                dept_name: x.dept_name,                     //部门名称
                                sort: x.sort,                               //显示顺序
                                leader: x.leader,                           //负责人
                                phone: x.phone,                             //联系电话
                                email: x.email,                             //邮箱
                                status: x.status, //部状态（0：停用，1:正常）
                                del_flag: x.del_flag.unwrap_or_default(), //删除标志（0代表删除 1代表存在）
                                create_time: time_to_string(x.create_time), //创建时间
                                update_time: time_to_string(x.update_time), //修改时间
                            }
                        })
                        .collect::<Vec<DeptListDataResp>>();

                    BaseResponse::ok_result_data(res, list)
                }
                Err(err) => {
                    BaseResponse::err_result_data(res, DeptListDataResp::new(), err.to_string())
                }
            }
        }
        Err(err) => {
            BaseResponse::<String>::err_result_msg(res, format!("解析请求参数失败: {}", err))
        }
    }
}
