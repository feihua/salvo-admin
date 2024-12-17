// Code generated by https://github.com/feihua/code_cli
// author：刘飞华
// date：2024/12/17 10:59:27

use crate::common::error::WhoUnfollowedError;
use crate::common::result::BaseResponse;
use crate::model::system::prelude::{SysMenu, SysRole, SysUser, SysUserRole};
use crate::model::system::{sys_menu, sys_user, sys_user_role};
use crate::utils::jwt_util::JWTToken;
use crate::vo::system::sys_user_vo::*;
use crate::AppState;
use salvo::prelude::*;
use sea_orm::prelude::Expr;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseBackend, DatabaseConnection,
    EntityTrait, NotSet, PaginatorTrait, QueryFilter, QueryOrder, QueryTrait, Statement,
};
use std::collections::HashSet;

/*
 *添加用户信息
 *author：刘飞华
 *date：2024/12/17 10:59:27
 */
#[handler]
pub async fn add_sys_user(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let item = req.parse_json::<AddUserReq>().await.unwrap();
    log::info!("add sys_user params: {:?}", &item);

    let state = depot.obtain::<AppState>().unwrap();
    let conn = &state.conn;

    let sys_user = sys_user::ActiveModel {
        id: NotSet,                     //主键
        mobile: Set(item.mobile),       //手机
        user_name: Set(item.user_name), //姓名
        password: Set(item.password),   //密码
        status_id: Set(item.status_id), //状态(1:正常，0:禁用)
        sort: Set(item.sort),           //排序
        remark: Set(item.remark),       //备注
        create_time: NotSet,            //创建时间
        update_time: NotSet,            //修改时间
    };

    let result = SysUser::insert(sys_user).exec(conn).await;
    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(res),
        Err(err) => BaseResponse::<String>::err_result_msg(res, err.to_string()),
    }
}

/*
 *删除用户信息
 *author：刘飞华
 *date：2024/12/17 10:59:27
 */
#[handler]
pub async fn delete_sys_user(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let item = req.parse_json::<DeleteUserReq>().await.unwrap();
    log::info!("delete sys_user params: {:?}", &item);

    let state = depot.obtain::<AppState>().unwrap();
    let conn = &state.conn;

    let ids = item.ids.clone();
    for id in ids {
        if id != 1 {
            //id为1的用户为系统预留用户,不能删除
            let _ = SysUser::delete_by_id(id).exec(conn).await;
        }
    }

    BaseResponse::<String>::ok_result(res)
}

/*
 *更新用户信息
 *author：刘飞华
 *date：2024/12/17 10:59:27
 */
#[handler]
pub async fn update_sys_user(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let item = req.parse_json::<UpdateUserReq>().await.unwrap();
    log::info!("update sys_user params: {:?}", &item);

    let state = depot.obtain::<AppState>().unwrap();
    let conn = &state.conn;

    if SysUser::find_by_id(item.id.clone())
        .one(conn)
        .await
        .unwrap_or_default()
        .is_none()
    {
        return BaseResponse::<String>::err_result_msg(res, "用户信息不存在,不能更新!".to_string());
    }

    let sys_user = sys_user::ActiveModel {
        id: Set(item.id),               //主键
        mobile: Set(item.mobile),       //手机
        user_name: Set(item.user_name), //姓名
        password: NotSet,               //密码
        status_id: Set(item.status_id), //状态(1:正常，0:禁用)
        sort: Set(item.sort),           //排序
        remark: Set(item.remark),       //备注
        create_time: NotSet,            //创建时间
        update_time: NotSet,            //修改时间
    };

    let result = SysUser::update(sys_user).exec(conn).await;
    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(res),
        Err(err) => BaseResponse::<String>::err_result_msg(res, err.to_string()),
    }
}

/*
 *更新用户信息状态
 *author：刘飞华
 *date：2024/12/17 10:59:27
 */
#[handler]
pub async fn update_sys_user_status(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let item = req.parse_json::<UpdateUserStatusReq>().await.unwrap();
    log::info!("update sys_user_status params: {:?}", &item);

    let state = depot.obtain::<AppState>().unwrap();
    let conn = &state.conn;

    let result = SysUser::update_many()
        .col_expr(sys_user::Column::StatusId, Expr::value(item.status))
        .filter(sys_user::Column::Id.is_in(item.ids))
        .exec(conn)
        .await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(res),
        Err(err) => BaseResponse::<String>::err_result_msg(res, err.to_string()),
    }
}

/*
 *更新用户密码
 *author：刘飞华
 *date：2024/12/17 10:59:27
 */
#[handler]
pub async fn update_user_password(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user_pwd = req.parse_json::<UpdateUserPwdReq>().await.unwrap();
    log::info!("update_user_pwd params: {:?}", &user_pwd);

    let state = depot.obtain::<AppState>().unwrap();
    let conn = &state.conn;

    let result = SysUser::find_by_id(user_pwd.id)
        .one(conn)
        .await
        .unwrap_or_default();
    if result.is_none() {
        return BaseResponse::<String>::err_result_msg(res, "用户不存在!".to_string());
    };

    let user = result.unwrap();
    if user.clone().password.unwrap_or_default() == user_pwd.pwd {
        let mut s_user: sys_user::ActiveModel = user.into();
        s_user.password = Set(Option::from(user_pwd.re_pwd));

        s_user.update(conn).await.unwrap();
        BaseResponse::<String>::ok_result_msg(res, "更新用户密码成功!".to_string())
    } else {
        BaseResponse::<String>::err_result_msg(res, "旧密码不正确!".to_string())
    }
}

/*
 *查询用户信息详情
 *author：刘飞华
 *date：2024/12/17 10:59:27
 */
#[handler]
pub async fn query_sys_user_detail(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let item = req.parse_json::<QueryUserDetailReq>().await.unwrap();
    log::info!("query sys_user_detail params: {:?}", &item);

    let state = depot.obtain::<AppState>().unwrap();
    let conn = &state.conn;

    let result = SysUser::find_by_id(item.id.clone()).one(conn).await;

    match result {
        Ok(d) => {
            let x = d.unwrap();

            let sys_user = QueryUserDetailResp {
                id: x.id,                               //主键
                mobile: x.mobile,                       //手机
                user_name: x.user_name,                 //姓名
                status_id: x.status_id,                 //状态(1:正常，0:禁用)
                sort: x.sort,                           //排序
                remark: x.remark.unwrap_or_default(),   //备注
                create_time: x.create_time.to_string(), //创建时间
                update_time: x.update_time.to_string(), //修改时间
            };

            BaseResponse::<QueryUserDetailResp>::ok_result_data(res, sys_user)
        }
        Err(err) => BaseResponse::<QueryUserDetailResp>::err_result_data(
            res,
            QueryUserDetailResp::new(),
            err.to_string(),
        ),
    }
}

/*
 *查询用户信息列表
 *author：刘飞华
 *date：2024/12/17 10:59:27
 */
#[handler]
pub async fn query_sys_user_list(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let item = req.parse_json::<QueryUserListReq>().await.unwrap();
    log::info!("query sys_user_list params: {:?}", &item);

    let state = depot.obtain::<AppState>().unwrap();
    let conn = &state.conn;

    let paginator = SysUser::find()
        .apply_if(item.mobile.clone(), |query, v| {
            query.filter(sys_user::Column::Mobile.eq(v))
        })
        .apply_if(item.user_name.clone(), |query, v| {
            query.filter(sys_user::Column::UserName.eq(v))
        })
        .apply_if(item.status_id.clone(), |query, v| {
            query.filter(sys_user::Column::StatusId.eq(v))
        })
        .paginate(conn, item.page_size.clone());

    let total = paginator.num_items().await.unwrap_or_default();

    let mut sys_user_list_data: Vec<UserListDataResp> = Vec::new();

    for x in paginator
        .fetch_page(item.page_no.clone() - 1)
        .await
        .unwrap_or_default()
    {
        sys_user_list_data.push(UserListDataResp {
            id: x.id,                               //主键
            mobile: x.mobile,                       //手机
            user_name: x.user_name,                 //姓名
            status_id: x.status_id,                 //状态(1:正常，0:禁用)
            sort: x.sort,                           //排序
            remark: x.remark.unwrap_or_default(),   //备注
            create_time: x.create_time.to_string(), //创建时间
            update_time: x.update_time.to_string(), //修改时间
        })
    }

    BaseResponse::<Vec<UserListDataResp>>::ok_result_page(res, sys_user_list_data, total)
}
// 后台用户登录
/*
 *后台用户登录
 *author：刘飞华
 *date：2024/12/17 10:59:27
 */
#[handler]
pub async fn login(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let item = req.parse_json::<UserLoginReq>().await.unwrap();
    log::info!("user login params: {:?}", &item);

    let state = depot.obtain::<AppState>().unwrap();
    let conn = &state.conn;

    let user_result = SysUser::find()
        .filter(sys_user::Column::Mobile.eq(&item.mobile))
        .one(conn)
        .await
        .unwrap_or_default();
    log::info!("select_by_mobile: {:?}", user_result);

    if user_result.is_none() {
        return BaseResponse::<String>::err_result_msg(res, "用户不存在!".to_string());
    }

    let user = user_result.unwrap();

    let id = user.id;
    let username = user.user_name;
    let password = user.password;

    if password.unwrap_or_default().ne(&item.password) {
        return BaseResponse::<String>::err_result_msg(res, "密码不正确!".to_string());
    }

    let btn_menu = query_btn_menu(conn, id.clone()).await;

    if btn_menu.len() == 0 {
        return BaseResponse::<String>::err_result_msg(
            res,
            "用户没有分配角色或者菜单,不能登录!".to_string(),
        );
    }

    match JWTToken::new(id, &username, btn_menu).create_token("123") {
        Ok(token) => BaseResponse::<String>::ok_result_data(res, token),
        Err(err) => {
            let er = match err {
                WhoUnfollowedError::JwtTokenError(s) => s,
                _ => "no math error".to_string(),
            };

            BaseResponse::<String>::err_result_msg(res, er.to_string())
        }
    }
}

/*
 *查询权限
 *author：刘飞华
 *date：2024/12/17 10:59:27
 */
async fn query_btn_menu(conn: &DatabaseConnection, id: i64) -> Vec<String> {
    let mut btn_menu: Vec<String> = Vec::new();
    //角色Id为1的是系统预留超级管理员角色
    if SysUserRole::find()
        .filter(sys_user_role::Column::UserId.eq(id.clone()))
        .filter(sys_user_role::Column::RoleId.eq(1))
        .count(conn)
        .await
        .unwrap_or_default()
        != 0
    {
        for x in SysMenu::find().all(conn).await.unwrap_or_default() {
            btn_menu.push(x.api_url.unwrap_or_default());
        }
        log::info!("admin login: {:?}", id);
    } else {
        let sql_str = r#"select distinct u.api_url from sys_user_role t left join sys_role usr on t.role_id = usr.id left join sys_role_menu srm on usr.id = srm.role_id left join sys_menu u on srm.menu_id = u.id where t.user_id = $1"#;
        for x in conn
            .query_all(Statement::from_sql_and_values(
                DatabaseBackend::MySql,
                sql_str,
                [id.into()],
            ))
            .await
            .unwrap_or_default()
        {
            btn_menu.push(x.try_get("", "api_url").unwrap_or_default());
        }
        log::info!("ordinary login: {:?}", id);
    }

    btn_menu
}

/*
 *查询用户角色
 *author：刘飞华
 *date：2024/12/17 10:59:27
 */
#[handler]
pub async fn query_user_role(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let item = req.parse_json::<QueryUserRoleReq>().await.unwrap();
    log::info!("query_user_role params: {:?}", item);

    let state = depot.obtain::<AppState>().unwrap();
    let conn = &state.conn;

    let mut user_role_ids: Vec<i64> = Vec::new();

    for x in SysUserRole::find()
        .filter(sys_user_role::Column::UserId.eq(item.user_id.clone()))
        .all(conn)
        .await
        .unwrap_or_default()
    {
        user_role_ids.push(x.role_id);
    }

    let mut sys_role_list: Vec<RoleList> = Vec::new();

    for x in SysRole::find().all(conn).await.unwrap_or_default() {
        sys_role_list.push(RoleList {
            id: x.id,
            status_id: x.status_id,
            sort: x.sort,
            role_name: x.role_name,
            remark: x.remark,
            create_time: x.create_time.to_string(),
            update_time: x.update_time.to_string(),
        });
    }

    BaseResponse::<QueryUserRoleResp>::ok_result_data(
        res,
        QueryUserRoleResp {
            role_list: sys_role_list,
            role_ids: user_role_ids,
        },
    )
}

/*
 *更新用户与角色的关联
 *author：刘飞华
 *date：2024/12/17 10:59:27
 */
#[handler]
pub async fn update_user_role(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user_role = req.parse_json::<UpdateUserRoleReq>().await.unwrap();
    log::info!("update_user_role params: {:?}", user_role);

    let state = depot.obtain::<AppState>().unwrap();
    let conn = &state.conn;

    let user_id = user_role.user_id;
    let role_ids = &user_role.role_ids;

    if user_id == 1 {
        return BaseResponse::<String>::err_result_msg(
            res,
            "不能修改超级管理员的角色!".to_string(),
        );
    }

    SysUserRole::delete_many()
        .filter(sys_user_role::Column::UserId.eq(user_id))
        .exec(conn)
        .await
        .unwrap();

    let mut sys_role_user_list: Vec<sys_user_role::ActiveModel> = Vec::new();
    for role_id in role_ids {
        let r_id = role_id.clone();
        if r_id == 1 {
            continue;
        }
        sys_role_user_list.push(sys_user_role::ActiveModel {
            id: NotSet,
            role_id: Set(r_id),
            user_id: Set(user_id.clone()),
            create_time: NotSet,
        })
    }

    SysUserRole::insert_many(sys_role_user_list)
        .exec(conn)
        .await
        .unwrap();
    BaseResponse::<String>::ok_result(res)
}

/*
 *查询用户菜单
 *author：刘飞华
 *date：2024/12/17 10:59:27
 */
#[handler]
pub async fn query_user_menu(depot: &mut Depot, res: &mut Response) {
    log::info!("query user menu params {:?}", depot);

    let state = depot.obtain::<AppState>().unwrap();
    let conn = &state.conn;

    let user_id = depot.get::<i64>("userId").copied().unwrap();
    let username = depot.get::<String>("username").unwrap().to_string();
    log::info!("query user menu params {:?}", user_id);
    log::info!("query user menu params {:?}", username);

    if SysUser::find_by_id(user_id)
        .one(conn)
        .await
        .unwrap_or_default()
        .is_none()
    {
        return BaseResponse::<String>::err_result_msg(res, "用户不存在!".to_string());
    }

    let sys_menu_list: Vec<sys_menu::Model>;

    if SysUserRole::find()
        .filter(sys_user_role::Column::UserId.eq(user_id.clone()))
        .filter(sys_user_role::Column::RoleId.eq(1))
        .one(conn)
        .await
        .unwrap_or_default()
        .is_some()
    {
        sys_menu_list = SysMenu::find().all(conn).await.unwrap_or_default();
    } else {
        let sql_str = r#"select u.* from sys_user_role t left join sys_role usr on t.role_id = usr.id left join sys_role_menu srm on usr.id = srm.role_id left join sys_menu u on srm.menu_id = u.id where t.user_id = $1 order by u.id asc"#;
        sys_menu_list = SysMenu::find()
            .from_raw_sql(Statement::from_sql_and_values(
                DatabaseBackend::MySql,
                sql_str,
                [user_id.into()],
            ))
            .all(conn)
            .await
            .unwrap_or_default();
    }

    let mut btn_menu: HashSet<String> = HashSet::new();
    let mut sys_menu_ids: HashSet<i64> = HashSet::new();

    for x in sys_menu_list {
        if x.menu_type != 3 {
            sys_menu_ids.insert(x.id);
            sys_menu_ids.insert(x.parent_id);
        }
        if x.api_url.clone().unwrap_or_default().len() > 0 {
            btn_menu.insert(x.api_url.unwrap_or_default());
        }
    }

    let mut menu_ids = Vec::new();
    for id in sys_menu_ids {
        menu_ids.push(id)
    }
    let mut sys_menu: HashSet<MenuList> = HashSet::new();
    for y in SysMenu::find()
        .filter(sys_menu::Column::Id.is_in(menu_ids))
        .filter(sys_menu::Column::StatusId.eq(1))
        .order_by_asc(sys_menu::Column::Sort)
        .all(conn)
        .await
        .unwrap_or_default()
    {
        sys_menu.insert(MenuList {
            id: y.id,
            parent_id: y.parent_id,
            name: y.menu_name,
            icon: y.menu_icon.unwrap_or_default(),
            api_url: y.api_url.clone().unwrap_or_default(),
            menu_type: y.menu_type,
            path: y.menu_url.unwrap().to_string(),
        });
        if y.api_url.clone().unwrap_or_default().len() > 0 {
            btn_menu.insert(y.api_url.unwrap_or_default());
        }
    }

    let avatar = "https://gw.alipayobjects.com/zos/antfincdn/XAosXuNZyF/BiazfanxmamNRoxxVxka.png"
        .to_string();

    BaseResponse::<QueryUserMenuResp>::ok_result_data(
        res,
        QueryUserMenuResp {
            sys_menu,
            btn_menu,
            avatar,
            name: username,
        },
    )
}
