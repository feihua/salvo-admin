use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::{PageRequest};
use salvo::{Request, Response};
use salvo::prelude::*;
use crate::{RB};

use crate::model::menu::{SysMenu};
use crate::vo::{BaseResponse, handle_result};
use crate::vo::menu_vo::{*};


#[handler]
pub async fn menu_list(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<MenuListReq>().await;
    log::info!("menu_list params: {:?}", &item);

    let result = SysMenu::select_page(&mut RB.clone(), &PageRequest::new(1, 1000)).await;

    let resp = match result {
        Ok(d) => {
            let total = d.total;

            let mut menu_list_resp: Vec<MenuListData> = Vec::new();

            for x in d.records {
                menu_list_resp.push(MenuListData {
                    id: x.id.unwrap(),
                    sort: x.sort,
                    status_id: x.status_id,
                    parent_id: x.parent_id,
                    menu_name: x.menu_name.to_string(),
                    label: x.menu_name,
                    menu_url: x.menu_url.unwrap_or_default(),
                    icon: x.menu_icon.unwrap_or_default(),
                    api_url: x.api_url.unwrap_or_default(),
                    remark: x.remark.unwrap_or_default(),
                    menu_type: x.menu_type,
                    create_time: x.create_time.unwrap().0.to_string(),
                    update_time: x.update_time.unwrap().0.to_string(),
                })
            }
            MenuListResp {
                msg: "successful".to_string(),
                code: 0,
                total,
                data: Some(menu_list_resp),
            }
        }
        Err(err) => {
            MenuListResp {
                msg: err.to_string(),
                code: 1,
                total: 0,
                data: None,
            }
        }
    };

    res.render(Json(resp))
}

#[handler]
pub async fn menu_save(req: &mut Request, res: &mut Response) {
    let menu = req.parse_json::<MenuSaveReq>().await.unwrap();
    log::info!("menu_save params: {:?}", &menu);

    let role = SysMenu {
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

    let result = SysMenu::insert(&mut RB.clone(), &role).await;

    res.render(Json(handle_result(result)))
}

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


#[handler]
pub async fn menu_delete(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<MenuDeleteReq>().await.unwrap();
    log::info!("menu_delete params: {:?}", &item);

    let id = item.id;

    //有下级的时候 不能直接删除
    let menus = SysMenu::select_by_column(&mut RB.clone(), "parent_id", id.clone()).await.unwrap_or_default();

    if menus.len() > 0 {
        return res.render(Json(BaseResponse {
            msg: "有下级菜单,不能直接删除".to_string(),
            code: 1,
            data: Some("None".to_string()),
        }));
    }

    let result = SysMenu::delete_by_column(&mut RB.clone(), "id", id).await;
    res.render(Json(handle_result(result)))
}