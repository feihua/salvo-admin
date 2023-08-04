use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::{PageRequest};
use salvo::{Request, Response};
use salvo::prelude::*;
use crate::{RB};

use crate::model::menu::{SysMenu};
use crate::vo::handle_result;
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
                    sort: x.sort.unwrap(),
                    status_id: x.status_id.unwrap(),
                    parent_id: x.parent_id.unwrap(),
                    menu_name: x.menu_name.as_ref().unwrap().to_string(),
                    label: x.menu_name.unwrap_or_default(),
                    menu_url: x.menu_url.unwrap_or_default(),
                    icon: x.menu_icon.unwrap_or_default(),
                    api_url: x.api_url.unwrap_or_default(),
                    remark: x.remark.unwrap_or_default(),
                    menu_type: x.menu_type.unwrap(),
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
        status_id: Some(menu.status_id),
        sort: Some(menu.sort),
        parent_id: Some(menu.parent_id.unwrap_or(0)),
        menu_name: Some(menu.menu_name),
        menu_url: Some(menu.menu_url),
        api_url: Some(menu.api_url),
        menu_icon: Some(menu.icon),
        remark: Some(menu.remark),
        menu_type: Some(menu.menu_type),
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
        status_id: Some(menu.status_id),
        sort: Some(menu.sort),
        parent_id: Some(menu.parent_id),
        menu_name: Some(menu.menu_name),
        menu_url: Some(menu.menu_url),
        api_url: Some(menu.api_url),
        menu_icon: Some(menu.icon),
        remark: Some(menu.remark),
        menu_type: Some(menu.menu_type),
    };

    let result = SysMenu::update_by_column(&mut RB.clone(), &sys_menu, "id").await;

    res.render(Json(handle_result(result)))
}


#[handler]
pub async fn menu_delete(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<MenuDeleteReq>().await.unwrap();
    log::info!("menu_delete params: {:?}", &item);


    let result = SysMenu::delete_in_column(&mut RB.clone(), "id", &item.ids).await;

    res.render(Json(handle_result(result)))
}