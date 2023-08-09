use rbatis::rbdc::datetime::DateTime;
use salvo::{Request, Response};
use salvo::prelude::*;
use crate::{RB};

use crate::model::menu::{SysMenu};
use crate::vo::{err_result_msg, err_result_page, handle_result, ok_result_page};
use crate::vo::menu_vo::{*};


// 查询菜单
#[handler]
pub async fn menu_list(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<MenuListReq>().await;
    log::info!("menu_list params: {:?}", &item);

    // 菜单是树形结构不需要分页
    let result = SysMenu::select_all(&mut RB.clone()).await;

    match result {
        Ok(sys_menu_list) => {
            let mut list_data: Vec<MenuListData> = Vec::new();

            for menu in sys_menu_list {
                list_data.push(MenuListData {
                    id: menu.id.unwrap(),
                    sort: menu.sort,
                    status_id: menu.status_id,
                    parent_id: menu.parent_id,
                    menu_name: menu.menu_name.to_string(),
                    label: menu.menu_name,
                    menu_url: menu.menu_url.unwrap_or_default(),
                    icon: menu.menu_icon.unwrap_or_default(),
                    api_url: menu.api_url.unwrap_or_default(),
                    remark: menu.remark.unwrap_or_default(),
                    menu_type: menu.menu_type,
                    create_time: menu.create_time.unwrap().0.to_string(),
                    update_time: menu.update_time.unwrap().0.to_string(),
                })
            }
            res.render(Json(ok_result_page(list_data, 0)))
        }
        Err(err) => {
            res.render(Json(err_result_page(err.to_string())))
        }
    };
}

// 添加菜单
#[handler]
pub async fn menu_save(req: &mut Request, res: &mut Response) {
    let menu = req.parse_json::<MenuSaveReq>().await.unwrap();
    log::info!("menu_save params: {:?}", &menu);

    let sys_menu = SysMenu {
        id: None,
        create_time: Some(DateTime::now()),
        update_time: Some(DateTime::now()),
        status_id: menu.status_id,
        sort: menu.sort,
        parent_id: menu.parent_id,
        menu_name: menu.menu_name,
        menu_url: menu.menu_url,
        api_url: menu.api_url,
        menu_icon: menu.icon,
        remark: menu.remark,
        menu_type: menu.menu_type,
    };

    let result = SysMenu::insert(&mut RB.clone(), &sys_menu).await;

    res.render(Json(handle_result(result)))
}

// 更新菜单
#[handler]
pub async fn menu_update(req: &mut Request, res: &mut Response) {
    let menu = req.parse_json::<MenuUpdateReq>().await.unwrap();
    log::info!("menu_update params: {:?}", &menu);

    let sys_menu = SysMenu {
        id: Some(menu.id),
        create_time: None,
        update_time: Some(DateTime::now()),
        status_id: menu.status_id,
        sort: menu.sort,
        parent_id: menu.parent_id,
        menu_name: menu.menu_name,
        menu_url: menu.menu_url,
        api_url: menu.api_url,
        menu_icon: menu.icon,
        remark: menu.remark,
        menu_type: menu.menu_type,
    };

    let result = SysMenu::update_by_column(&mut RB.clone(), &sys_menu, "id").await;

    res.render(Json(handle_result(result)))
}

// 删除菜单信息
#[handler]
pub async fn menu_delete(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<MenuDeleteReq>().await.unwrap();
    log::info!("menu_delete params: {:?}", &item);

    let id = item.id;

    //有下级的时候 不能直接删除
    let menus = SysMenu::select_by_column(&mut RB.clone(), "parent_id", id.clone()).await.unwrap_or_default();

    if menus.len() > 0 {
        return res.render(Json(err_result_msg("有下级菜单,不能直接删除".to_string())));
    }

    let result = SysMenu::delete_by_column(&mut RB.clone(), "id", id).await;
    res.render(Json(handle_result(result)))
}