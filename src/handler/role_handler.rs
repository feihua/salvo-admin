use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::{PageRequest};
use salvo::{Request, Response};
use salvo::prelude::*;

use crate::model::role::{SysRole};
use crate::model::menu::{ SysMenu};
use crate::model::role_menu::{query_menu_by_role, SysRoleMenu};
use crate::RB;
use crate::vo::handle_result;
use crate::vo::role_vo::*;


#[handler]
pub async fn role_list(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<RoleListReq>().await.unwrap();
    log::info!("role_list params: {:?}", &item);


    let role_name = item.role_name.as_deref().unwrap_or_default();
    let status_id = item.status_id.as_deref().unwrap_or_default();

    let page = &PageRequest::new(item.page_no, item.page_size);
    let result = SysRole::select_page_by_name(&mut RB.clone(), page, role_name, status_id).await;

    let resp = match result {
        Ok(d) => {
            let total = d.total;

            let mut role_list_resp: Vec<RoleListData> = Vec::new();

            for x in d.records {
                role_list_resp.push(RoleListData {
                    id: x.id.unwrap(),
                    sort: x.sort,
                    status_id: x.status_id,
                    role_name: x.role_name,
                    remark: x.remark.unwrap_or_default(),
                    create_time: x.create_time.unwrap().0.to_string(),
                    update_time: x.update_time.unwrap().0.to_string(),
                })
            }

            RoleListResp {
                msg: "successful".to_string(),
                code: 0,
                success: true,
                total,
                data: Some(role_list_resp),
            }
        }
        Err(err) => {
            RoleListResp {
                msg: err.to_string(),
                code: 1,
                success: true,
                total: 0,
                data: None,
            }
        }
    };

    res.render(Json(resp))
}


#[handler]
pub async fn role_save(req: &mut Request, res: &mut Response) {
    let role = req.parse_json::<RoleSaveReq>().await.unwrap();
    println!("model: {:?}", &role);

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


#[handler]
pub async fn role_update(req: &mut Request, res: &mut Response) {
    let role = req.parse_json::<RoleUpdateReq>().await.unwrap();
    println!("item: {:?}", &role);

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


#[handler]
pub async fn role_delete(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<RoleDeleteReq>().await.unwrap();
    println!("item: {:?}", &item);


    let result = SysRole::delete_in_column(&mut RB.clone(), "id", &item.ids).await;

    res.render(Json(handle_result(result)))
}


#[handler]
pub async fn query_role_menu(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<QueryRoleMenuReq>().await.unwrap();
    log::info!("query_role_menu params: {:?}", &item);


    let role_menu_list = query_menu_by_role(&mut RB.clone(), item.role_id).await;

    let menu_list = SysMenu::select_all(&mut RB.clone()).await;

    let mut menu_data_list: Vec<MenuDataList> = Vec::new();
    let mut role_menus: Vec<i32> = Vec::new();


    for y in menu_list.unwrap_or_default() {
        let x = y.clone();
        menu_data_list.push(MenuDataList {
            id: x.id.unwrap(),
            parent_id: x.parent_id,
            title: x.menu_name,
            key: y.id.unwrap().to_string(),
            label: y.menu_name,
            is_penultimate: y.parent_id == 2,
        });
    }

    for x in role_menu_list.unwrap_or_default() {
        let m_id = x.get("menu_id").unwrap().clone();
        role_menus.push(m_id)
    }

    let resp = QueryRoleMenuResp {
        msg: "successful".to_string(),
        code: 0,
        data: QueryRoleMenuData {
            role_menus,
            menu_list: menu_data_list,
        },
    };

    res.render(Json(resp))
}


#[handler]
pub async fn update_role_menu(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<UpdateRoleMenuReq>().await.unwrap();
    log::info!("update_role_menu params: {:?}", &item);
    let role_id = item.role_id;



    SysRoleMenu::delete_by_column(&mut RB.clone(), "role_id", &role_id).await.expect("删除角色菜单异常");

    let mut menu_role: Vec<SysRoleMenu> = Vec::new();

    for x in &item.menu_ids {
        let menu_id= x.clone();
        menu_role.push(SysRoleMenu {
            id: None,
            create_time: Some(DateTime::now()),
            update_time: Some(DateTime::now()),
            status_id: Some(1),
            sort: Some(1),
            menu_id: Some(menu_id),
            role_id: Some(role_id.clone()),
        })
    }

    let result = SysRoleMenu::insert_batch(&mut RB.clone(), &menu_role, item.menu_ids.len() as u64).await;

    res.render(Json(handle_result(result)))
}
