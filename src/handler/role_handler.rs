use log::error;
use rbatis::rbdc::datetime::DateTime;
use rbatis::plugin::page::PageRequest;
use salvo::{Request, Response};
use salvo::prelude::*;

use crate::model::menu::SysMenu;
use crate::model::role::SysRole;
use crate::model::role_menu::{query_menu_by_role, SysRoleMenu};
use crate::model::user_role::SysUserRole;
use crate::RB;
use crate::vo::{err_result_msg, err_result_page, handle_result, ok_result_data, ok_result_page};
use crate::vo::role_vo::*;

// 查询角色列表
#[handler]
pub async fn role_list(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<RoleListReq>().await.unwrap();
    log::info!("role_list params: {:?}", &item);

    let role_name = item.role_name.unwrap_or_default();
    let status_id = item.status_id.unwrap_or_default();

    let page_req = &PageRequest::new(item.page_no, item.page_size);
    let result = SysRole::select_page_by_name(&mut RB.clone(), page_req, &role_name, &status_id).await;

    match result {
        Ok(page) => {
            let total = page.total;

            let mut list_data: Vec<RoleListData> = Vec::new();

            for role in page.records {
                list_data.push(RoleListData {
                    id: role.id.unwrap(),
                    sort: role.sort,
                    status_id: role.status_id,
                    role_name: role.role_name,
                    remark: role.remark.unwrap_or_default(),
                    create_time: role.create_time.unwrap().0.to_string(),
                    update_time: role.update_time.unwrap().0.to_string(),
                })
            }

            res.render(Json(ok_result_page(list_data, total)))
        }
        Err(err) => {
            error!("{}", err.to_string());
            res.render(Json(err_result_page(err.to_string())))
        }
    }
}

// 添加角色信息
#[handler]
pub async fn role_save(req: &mut Request, res: &mut Response) {
    let role = req.parse_json::<RoleSaveReq>().await.unwrap();
    log::info!("role_save params: {:?}", &role);

    let sys_role = SysRole {
        id: None,
        create_time: Some(DateTime::now()),
        update_time: Some(DateTime::now()),
        status_id: role.status_id,
        sort: role.sort,
        role_name: role.role_name,
        remark: role.remark,
    };

    let result = SysRole::insert(&mut RB.clone(), &sys_role).await;

    res.render(Json(handle_result(result)))
}

// 更新角色信息
#[handler]
pub async fn role_update(req: &mut Request, res: &mut Response) {
    let role = req.parse_json::<RoleUpdateReq>().await.unwrap();
    log::info!("role_update params: {:?}", &role);

    let sys_role = SysRole {
        id: Some(role.id),
        create_time: None,
        update_time: Some(DateTime::now()),
        status_id: role.status_id,
        sort: role.sort,
        role_name: role.role_name,
        remark: role.remark,
    };

    let result = SysRole::update_by_column(&mut RB.clone(), &sys_role, "id").await;

    res.render(Json(handle_result(result)))
}

// 删除角色信息
#[handler]
pub async fn role_delete(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<RoleDeleteReq>().await.unwrap();
    log::info!("role_delete params: {:?}", &item);

    let ids = item.ids;
    let user_role_list = SysUserRole::select_in_column(&mut RB.clone(), "role_id", &ids).await.unwrap_or_default();

    if user_role_list.len() > 0 {
        return res.render(Json(err_result_msg("角色已被使用,不能直接删除".to_string())));
    }


    let result = SysRole::delete_in_column(&mut RB.clone(), "id", &ids).await;

    res.render(Json(handle_result(result)))
}

// 查询角色关联的菜单
#[handler]
pub async fn query_role_menu(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<QueryRoleMenuReq>().await.unwrap();
    log::info!("query_role_menu params: {:?}", &item);

    // 查询所有菜单
    let menu_list = SysMenu::select_all(&mut RB.clone()).await.unwrap_or_default();

    let mut menu_data_list: Vec<MenuDataList> = Vec::new();
    let mut role_menu_ids: Vec<i32> = Vec::new();

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
        role_menu_ids.push(x.id.unwrap())
    }

    //不是超级管理员的时候,就要查询角色和菜单的关联
    if item.role_id != 1 {
        role_menu_ids.clear();
        let role_menu_list = query_menu_by_role(&mut RB.clone(), item.role_id).await.unwrap_or_default();

        for x in role_menu_list {
            let m_id = x.get("menu_id").unwrap().clone();
            role_menu_ids.push(m_id)
        }
    }

    res.render(Json(ok_result_data(QueryRoleMenuData {
        role_menus: role_menu_ids,
        menu_list: menu_data_list,
    })))
}

// 更新角色关联的菜单
#[handler]
pub async fn update_role_menu(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<UpdateRoleMenuReq>().await.unwrap();
    log::info!("update_role_menu params: {:?}", &item);
    let role_id = item.role_id;

    let role_menu_result = SysRoleMenu::delete_by_column(&mut RB.clone(), "role_id", &role_id).await;

    match role_menu_result {
        Ok(_) => {
            let mut menu_role: Vec<SysRoleMenu> = Vec::new();

            for id in &item.menu_ids {
                let menu_id = id.clone();
                menu_role.push(SysRoleMenu {
                    id: None,
                    create_time: Some(DateTime::now()),
                    update_time: Some(DateTime::now()),
                    status_id: 1,
                    sort: 1,
                    menu_id,
                    role_id: role_id.clone(),
                })
            }

            let result = SysRoleMenu::insert_batch(&mut RB.clone(), &menu_role, item.menu_ids.len() as u64).await;

            res.render(Json(handle_result(result)))
        }
        Err(err) => {
            error!("{}", err.to_string());
            res.render(Json(err_result_msg(err.to_string())))
        }
    }
}
