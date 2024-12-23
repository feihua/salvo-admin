// author：刘飞华
// date：2024/12/16 10:07:18

use log::error;
use rbatis::plugin::page::PageRequest;
use rbatis::rbdc::datetime::DateTime;
use rbs::to_value;
use salvo::prelude::*;
use salvo::{Request, Response};

use crate::common::result::BaseResponse;
use crate::model::system::sys_menu_model::Menu;
use crate::model::system::sys_role_menu_model::{query_menu_by_role, RoleMenu};
use crate::model::system::sys_role_model::Role;
use crate::model::system::sys_user_role_model::UserRole;
use crate::vo::system::sys_role_vo::*;
use crate::RB;

/*
 *添加角色信息
 *author：刘飞华
 *date：2024/12/16 10:07:18
 */
#[handler]
pub async fn add_sys_role(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<AddRoleReq>().await.unwrap();
    log::info!("add sys_role params: {:?}", &item);

    let sys_role = Role {
        id: None,                  //主键
        role_name: item.role_name, //名称
        status_id: item.status_id, //状态(1:正常，0:禁用)
        sort: item.sort,           //排序
        remark: item.remark,       //备注
        create_time: None,         //创建时间
        update_time: None,         //修改时间
    };

    let result = Role::insert(&mut RB.clone(), &sys_role).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(res),
        Err(err) => BaseResponse::<String>::err_result_msg(res, err.to_string()),
    }
}

/*
 *删除角色信息
 *author：刘飞华
 *date：2024/12/16 10:07:18
 */
#[handler]
pub async fn delete_sys_role(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<DeleteRoleReq>().await.unwrap();
    log::info!("delete sys_role params: {:?}", &item);

    let ids = item.ids;
    let user_role_list = UserRole::select_in_column(&mut RB.clone(), "role_id", &ids)
        .await
        .unwrap_or_default();

    if user_role_list.len() > 0 {
        BaseResponse::<String>::err_result_msg(res, "角色已被使用,不能直接删除".to_string());
        return;
    }

    let result = Role::delete_in_column(&mut RB.clone(), "id", &ids).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(res),
        Err(err) => BaseResponse::<String>::err_result_msg(res, err.to_string()),
    }
}

/*
 *更新角色信息
 *author：刘飞华
 *date：2024/12/16 10:07:18
 */
#[handler]
pub async fn update_sys_role(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<UpdateRoleReq>().await.unwrap();
    log::info!("update sys_role params: {:?}", &item);

    let sys_role = Role {
        id: Some(item.id),         //主键
        role_name: item.role_name, //名称
        status_id: item.status_id, //状态(1:正常，0:禁用)
        sort: item.sort,           //排序
        remark: item.remark,       //备注
        create_time: None,         //创建时间
        update_time: None,         //修改时间
    };

    let result = Role::update_by_column(&mut RB.clone(), &sys_role, "id").await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(res),
        Err(err) => BaseResponse::<String>::err_result_msg(res, err.to_string()),
    }
}

/*
 *更新角色信息状态
 *author：刘飞华
 *date：2024/12/16 10:07:18
 */
#[handler]
pub async fn update_sys_role_status(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<UpdateRoleStatusReq>().await.unwrap();
    log::info!("update sys_role_status params: {:?}", &item);

    let rb = &mut RB.clone();

    let update_sql = format!(
        "update sys_role set status_id = ? where id in ({})",
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

/*
 *查询角色信息详情
 *author：刘飞华
 *date：2024/12/16 10:07:18
 */
#[handler]
pub async fn query_sys_role_detail(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<QueryRoleDetailReq>().await.unwrap();
    log::info!("query sys_role_detail params: {:?}", &item);

    let result = Role::select_by_id(&mut RB.clone(), &item.id).await;

    match result {
        Ok(d) => {
            let x = d.unwrap();

            let sys_role = QueryRoleDetailResp {
                id: x.id.unwrap(),                      //主键
                role_name: x.role_name,                 //名称
                status_id: x.status_id,                 //状态(1:正常，0:禁用)
                sort: x.sort,                           //排序
                remark: x.remark,                       //备注
                create_time: x.create_time.unwrap().0.to_string(), //创建时间
                update_time: x.update_time.unwrap().0.to_string(), //修改时间
            };

            BaseResponse::<QueryRoleDetailResp>::ok_result_data(res, sys_role)
        }
        Err(err) => BaseResponse::<QueryRoleDetailResp>::err_result_data(
            res,
            QueryRoleDetailResp::new(),
            err.to_string(),
        ),
    }
}

/*
 *查询角色信息列表
 *author：刘飞华
 *date：2024/12/16 10:07:18
 */
#[handler]
pub async fn query_sys_role_list(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<QueryRoleListReq>().await.unwrap();
    log::info!("query sys_role_list params: {:?}", &item);

    let role_name = item.role_name.unwrap_or_default();
    let status_id = item.status_id.unwrap_or_default();

    let page = &PageRequest::new(item.page_no, item.page_size);
    let result = Role::select_page_by_name(&mut RB.clone(), page, &role_name, status_id).await;

    match result {
        Ok(d) => {
            let total = d.total;

            let mut sys_role_list_data: Vec<RoleListDataResp> = Vec::new();

            for x in d.records {
                let sys_role = RoleListDataResp {
                    id: x.id.unwrap(),                                 //主键
                    role_name: x.role_name,                            //名称
                    status_id: x.status_id,                            //状态(1:正常，0:禁用)
                    sort: x.sort,                                      //排序
                    remark: x.remark,                                  //备注
                    create_time: x.create_time.unwrap().0.to_string(), //创建时间
                    update_time: x.update_time.unwrap().0.to_string(), //修改时间
                };
                sys_role_list_data.push(sys_role);
            }

            BaseResponse::<Vec<RoleListDataResp>>::ok_result_page(res, sys_role_list_data, total)
        }
        Err(err) => BaseResponse::<String>::err_result_page(res, err.to_string()),
    }
}

/*
 *查询角色关联的菜单
 *author：刘飞华
 *date：2024/12/16 10:07:18
 */
#[handler]
pub async fn query_role_menu(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<QueryRoleMenuReq>().await.unwrap();
    log::info!("query_role_menu params: {:?}", &item);

    // 查询所有菜单
    let menu_list = Menu::select_all(&mut RB.clone()).await.unwrap_or_default();

    let mut menu_data_list: Vec<MenuDataList> = Vec::new();
    let mut menu_ids: Vec<i64> = Vec::new();

    for y in menu_list {
        let x = y.clone();
        menu_data_list.push(MenuDataList {
            id: x.id.unwrap(),
            parent_id: x.parent_id,
            title: x.menu_name,
            key: y.id.unwrap().to_string(),
            label: y.menu_name,
            is_penultimate: y.parent_id == 2,
        });
        menu_ids.push(x.id.unwrap())
    }

    //不是超级管理员的时候,就要查询角色和菜单的关联
    if item.role_id != 1 {
        menu_ids.clear();
        let role_menu_list = query_menu_by_role(&mut RB.clone(), item.role_id)
            .await
            .unwrap_or_default();

        for x in role_menu_list {
            let m_id = x.get("menu_id").unwrap().clone();
            menu_ids.push(m_id)
        }
    }

    BaseResponse::<QueryRoleMenuData>::ok_result_data(
        res,
        QueryRoleMenuData {
            menu_ids,
            menu_list: menu_data_list,
        },
    )
}

/*
 *更新角色关联的菜单
 *author：刘飞华
 *date：2024/12/16 10:07:18
 */
#[handler]
pub async fn update_role_menu(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<UpdateRoleMenuReq>().await.unwrap();
    log::info!("update_role_menu params: {:?}", &item);
    let role_id = item.role_id;

    let role_menu_result = RoleMenu::delete_by_column(&mut RB.clone(), "role_id", &role_id).await;

    match role_menu_result {
        Ok(_) => {
            let mut menu_role: Vec<RoleMenu> = Vec::new();

            for id in &item.menu_ids {
                let menu_id = id.clone();
                menu_role.push(RoleMenu {
                    id: None,
                    create_time: Some(DateTime::now()),
                    menu_id,
                    role_id: role_id.clone(),
                })
            }

            let result =
                RoleMenu::insert_batch(&mut RB.clone(), &menu_role, item.menu_ids.len() as u64)
                    .await;

            match result {
                Ok(_u) => BaseResponse::<String>::ok_result(res),
                Err(err) => BaseResponse::<String>::err_result_msg(res, err.to_string()),
            };
        }
        Err(err) => {
            error!("{}", err.to_string());
            BaseResponse::<String>::err_result_msg(res, err.to_string())
        }
    }
}
