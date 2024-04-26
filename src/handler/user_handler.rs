use std::collections::{HashMap, HashSet};

use log::error;
use rbatis::rbdc::datetime::DateTime;
use rbatis::plugin::page::PageRequest;
use rbs::to_value;
use salvo::{Request, Response};
use salvo::prelude::*;

use crate::model::menu::SysMenu;
use crate::model::role::SysRole;
use crate::model::user::SysUser;
use crate::model::user_role::SysUserRole;
use crate::RB;
use crate::utils::error::WhoUnfollowedError;
use crate::utils::jwt_util::JWTToken;
use crate::vo::{err_result_msg, err_result_page, handle_result, ok_result_data, ok_result_msg, ok_result_page};
use crate::vo::user_vo::*;

// 后台用户登录
#[handler]
pub async fn login(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<UserLoginReq>().await.unwrap();
    log::info!("user login params: {:?}", &item);

    let user_result = SysUser::select_by_mobile(&mut RB.clone(), &item.mobile).await;
    log::info!("select_by_mobile: {:?}",user_result);

    match user_result {
        Ok(u) => {
            match u {
                None => {
                    return res.render(Json(err_result_msg("用户不存在".to_string())));
                }
                Some(user) => {
                    let id = user.id.unwrap();
                    let username = user.user_name;
                    let password = user.password;

                    if password.ne(&item.password) {
                        return res.render(Json(err_result_msg("密码不正确".to_string())));
                    }

                    let btn_menu = query_btn_menu(&id).await;

                    if btn_menu.len() == 0 {
                        return res.render(Json(err_result_msg("用户没有分配角色或者菜单,不能登录".to_string())));
                    }

                    match JWTToken::new(id, &username, btn_menu).create_token("123") {
                        Ok(token) => {
                            res.render(Json(ok_result_data(token)))
                        }
                        Err(err) => {
                            let er = match err {
                                WhoUnfollowedError::JwtTokenError(s) => { s }
                                _ => "no math error".to_string()
                            };

                            res.render(Json(err_result_msg(er)))
                        }
                    }
                }
            }
        }

        Err(err) => {
            log::error!("select_by_column: {:?}",err);
            return res.render(Json(err_result_msg("查询用户异常".to_string())));
        }
    }
}

async fn query_btn_menu(id: &i32) -> Vec<String> {
    let user_role = SysUserRole::is_admin(&mut RB.clone(), id).await;
    let mut btn_menu: Vec<String> = Vec::new();
    if user_role.unwrap().len() == 1 {
        let data = SysMenu::select_all(&mut RB.clone()).await;

        for x in data.unwrap() {
            btn_menu.push(x.api_url.unwrap_or_default());
        }
        log::info!("admin login: {:?}",id);
        btn_menu
    } else {
        let sql_str = "select distinct u.api_url from sys_user_role t left join sys_role usr on t.role_id = usr.id left join sys_role_menu srm on usr.id = srm.role_id left join sys_menu u on srm.menu_id = u.id where t.user_id = ?";
        let btn_menu_map = RB.query_decode::<Vec<HashMap<String, String>>>(sql_str, vec![to_value!(id)]).await.unwrap();
        for x in btn_menu_map {
            btn_menu.push(x.get("api_url").unwrap().to_string());
        }
        log::info!("ordinary login: {:?}",id);
        btn_menu
    }
}

#[handler]
pub async fn query_user_role(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<QueryUserRoleReq>().await.unwrap();
    log::info!("query_user_role params: {:?}", item);

    let user_role = SysUserRole::select_by_column(&mut RB.clone(), "user_id", item.user_id).await;
    let mut user_role_ids: Vec<i32> = Vec::new();

    for x in user_role.unwrap() {
        user_role_ids.push(x.role_id);
    }

    let sys_role = SysRole::select_all(&mut RB.clone()).await;

    let mut sys_role_list: Vec<UserRoleList> = Vec::new();

    for x in sys_role.unwrap() {
        sys_role_list.push(UserRoleList {
            id: x.id.unwrap(),
            status_id: x.status_id,
            sort: x.sort,
            role_name: x.role_name,
            remark: x.remark.unwrap_or_default(),
            create_time: x.create_time.unwrap().0.to_string(),
            update_time: x.update_time.unwrap().0.to_string(),
        });
    }

    res.render(Json(ok_result_data(QueryUserRoleData {
        sys_role_list,
        user_role_ids,
    })))
}

// 更新用户与角色的关联
#[handler]
pub async fn update_user_role(req: &mut Request, res: &mut Response) {
    let user_role = req.parse_json::<UpdateUserRoleReq>().await.unwrap();
    log::info!("update_user_role params: {:?}", user_role);

    let user_id = user_role.user_id.clone();
    let role_ids = &user_role.role_ids;
    let len = user_role.role_ids.len();

    if user_id.clone() == 1 {
        return res.render(Json(err_result_msg("不能修改超级管理员的角色".to_string())));
    }

    let sys_result = SysUserRole::delete_by_column(&mut RB.clone(), "user_id", user_id.clone()).await;

    if sys_result.is_err() {
        return res.render(Json(err_result_msg("更新用户角色异常".to_string())));
    }

    let mut sys_role_user_list: Vec<SysUserRole> = Vec::new();
    for role_id in role_ids {
        let r_id = role_id.clone();
        sys_role_user_list.push(SysUserRole {
            id: None,
            create_time: Some(DateTime::now()),
            update_time: Some(DateTime::now()),
            status_id: 1,
            sort: 1,
            role_id: r_id,
            user_id: user_id.clone(),
        })
    }

    let result = SysUserRole::insert_batch(&mut RB.clone(), &sys_role_user_list, len as u64).await;

    res.render(Json(handle_result(result)))
}


#[handler]
pub async fn query_user_menu(depot: &mut Depot, res: &mut Response) {
    log::info!("query user menu params {:?}",depot);
    let user_id = depot.get::<i32>("userId").copied().unwrap();
    let username = depot.get::<String>("username").unwrap().to_string();
    log::info!("query user menu params {:?}",user_id);
    log::info!("query user menu params {:?}",username);

    //根据id查询用户
    let result = SysUser::select_by_id(&mut RB.clone(), user_id).await;

    match result {
        Ok(sys_user) => {
            match sys_user {
                // 用户不存在的情况
                None => {
                    res.render(Json(err_result_msg("用户不存在".to_string())))
                }
                Some(user) => {
                    //role_id为1是超级管理员--判断是不是超级管理员
                    let sql_str = "select count(id) from sys_user_role where role_id = 1 and user_id = ?";
                    let count = RB.query_decode::<i32>(sql_str, vec![to_value!(user.id)]).await.unwrap();

                    let sys_menu_list: Vec<SysMenu>;

                    if count > 0 {
                        sys_menu_list = SysMenu::select_all(&mut RB.clone()).await.unwrap_or_default();
                    } else {
                        let sql_str = "select u.* from sys_user_role t left join sys_role usr on t.role_id = usr.id left join sys_role_menu srm on usr.id = srm.role_id left join sys_menu u on srm.menu_id = u.id where t.user_id = ?";
                        sys_menu_list = RB.query_decode(sql_str, vec![to_value!(user.id)]).await.unwrap();
                    }

                    let mut sys_menu: Vec<MenuUserList> = Vec::new();
                    let mut btn_menu: Vec<String> = Vec::new();
                    let mut sys_menu_ids: HashSet<i32> = HashSet::new();

                    for x in sys_menu_list {
                        if x.menu_type != 3 {
                            sys_menu_ids.insert(x.id.unwrap_or_default().clone());
                            sys_menu_ids.insert(x.parent_id.clone());
                        }
                        if x.api_url.clone().unwrap_or_default().len() > 0 {
                            btn_menu.push(x.api_url.unwrap_or_default());
                        }
                    }

                    let mut menu_ids = Vec::new();
                    for id in sys_menu_ids {
                        menu_ids.push(id)
                    }
                    let menu_result = SysMenu::select_by_ids(&mut RB.clone(), &menu_ids).await.unwrap();
                    for menu in menu_result {
                        sys_menu.push(MenuUserList {
                            id: menu.id.unwrap(),
                            parent_id: menu.parent_id,
                            name: menu.menu_name,
                            icon: menu.menu_icon.unwrap_or_default(),
                            api_url: menu.api_url.as_ref().unwrap().to_string(),
                            menu_type: menu.menu_type,
                            path: menu.menu_url.unwrap_or_default(),
                        });

                        if menu.api_url.clone().unwrap_or_default().len() > 0 {
                            btn_menu.push(menu.api_url.unwrap_or_default());
                        }
                    }

                    res.render(Json(ok_result_data(QueryUserMenuData {
                        sys_menu,
                        btn_menu,
                        avatar: "https://gw.alipayobjects.com/zos/antfincdn/XAosXuNZyF/BiazfanxmamNRoxxVxka.png".to_string(),
                        name: user.user_name,
                    })))
                }
            }
        }
        // 查询用户数据库异常
        Err(err) => {
            error!("{}", err.to_string());
            res.render(Json(err_result_msg(err.to_string())))
        }
    }
}

// 查询用户列表
#[handler]
pub async fn user_list(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<UserListReq>().await.unwrap();
    log::info!("query user_list params: {:?}", &item);

    let mobile = item.mobile.as_deref().unwrap_or_default();
    //状态(1:正常，0:禁用),不传状态参数的时候,默认为2,在rbatis语句中判断
    let status_id = item.status_id.unwrap_or(2);

    let page_req = &PageRequest::new(item.page_no, item.page_size);
    let result = SysUser::select_page_by_name(&mut RB.clone(), page_req, mobile, status_id).await;

    match result {
        Ok(page) => {
            let total = page.total;

            let mut list_data: Vec<UserListData> = Vec::new();

            for user in page.records {
                list_data.push(UserListData {
                    id: user.id.unwrap(),
                    sort: user.sort,
                    status_id: user.status_id,
                    mobile: user.mobile,
                    user_name: user.user_name,
                    remark: user.remark.unwrap_or_default(),
                    create_time: user.create_time.unwrap().0.to_string(),
                    update_time: user.update_time.unwrap().0.to_string(),
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

// 添加用户信息
#[handler]
pub async fn user_save(req: &mut Request, res: &mut Response) {
    let user = req.parse_json::<UserSaveReq>().await.unwrap();
    log::info!("user_save params: {:?}", &user);

    let sys_user = SysUser {
        id: None,
        create_time: Some(DateTime::now()),
        update_time: Some(DateTime::now()),
        status_id: user.status_id,
        sort: user.sort,
        mobile: user.mobile,
        user_name: user.user_name,
        remark: user.remark,
        password: "123456".to_string(),//默认密码为123456,暂时不加密
    };

    let result = SysUser::insert(&mut RB.clone(), &sys_user).await;

    res.render(Json(handle_result(result)))
}

// 更新用户信息
#[handler]
pub async fn user_update(req: &mut Request, res: &mut Response) {
    let user = req.parse_json::<UserUpdateReq>().await.unwrap();
    log::info!("user_update params: {:?}", &user);

    let result = SysUser::select_by_id(&mut RB.clone(), user.id.clone()).await.unwrap();

    match result {
        None => {
            res.render(Json(err_result_msg("用户不存在".to_string())))
        }
        Some(sys_user) => {
            let sys_user = SysUser {
                id: Some(user.id),
                create_time: sys_user.create_time,
                update_time: Some(DateTime::now()),
                status_id: user.status_id,
                sort: user.sort,
                mobile: user.mobile,
                user_name: user.user_name,
                remark: user.remark,
                password: sys_user.password,
            };

            let result = SysUser::update_by_column(&mut RB.clone(), &sys_user, "id").await;

            res.render(Json(handle_result(result)))
        }
    }
}

// 删除用户信息
#[handler]
pub async fn user_delete(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<UserDeleteReq>().await.unwrap();
    log::info!("user_delete params: {:?}", &item);

    let ids = item.ids;
    for id in ids {
        if id != 1 {//id为1的用户为系统预留用户,不能删除
            let _ = SysUser::delete_by_column(&mut RB.clone(), "id", &id).await;
        }
    }

    res.render(Json(ok_result_msg("删除用户信息成功".to_string())))
}

// 更新用户密码
#[handler]
pub async fn update_user_password(req: &mut Request, res: &mut Response) {
    let user_pwd = req.parse_json::<UpdateUserPwdReq>().await.unwrap();
    log::info!("update_user_pwd params: {:?}", &user_pwd);

    let sys_user_result = SysUser::select_by_id(&mut RB.clone(), user_pwd.id).await;

    match sys_user_result {
        Ok(user_result) => {
            match user_result {
                None => {
                    res.render(Json(err_result_msg("用户不存在".to_string())))
                }
                Some(mut user) => {
                    if user.password == user_pwd.pwd {
                        user.password = user_pwd.re_pwd;
                        let result = SysUser::update_by_column(&mut RB.clone(), &user, "id").await;

                        res.render(Json(handle_result(result)))
                    } else {
                        res.render(Json(err_result_msg("旧密码不正确".to_string())))
                    }
                }
            }
        }
        Err(err) => {
            error!("{}", err.to_string());
            res.render(Json(err_result_msg(err.to_string())))
        }
    }
}