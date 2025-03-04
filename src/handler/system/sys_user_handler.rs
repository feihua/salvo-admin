// Code generated by https://github.com/feihua/code_cli
// author：刘飞华
// date：2025/01/08 13:51:14

use crate::common::error::AppResult;
use crate::common::result::BaseResponse;
use crate::model::system::sys_dept_model::Dept;
use crate::model::system::sys_login_log_model::LoginLog;
use crate::model::system::sys_menu_model::Menu;
use crate::model::system::sys_role_model::Role;
use crate::model::system::sys_user_model::User;
use crate::model::system::sys_user_post_model::UserPost;
use crate::model::system::sys_user_role_model::{is_admin, UserRole};
use crate::utils::jwt_util::JwtToken;
use crate::utils::time_util::time_to_string;
use crate::utils::user_agent_util::UserAgentUtil;
use crate::vo::system::sys_dept_vo::QueryDeptDetailResp;
use crate::vo::system::sys_user_vo::*;
use crate::RB;
use rbatis::plugin::page::PageRequest;
use rbatis::rbatis_codegen::ops::AsProxy;
use rbatis::rbdc::datetime::DateTime;
use rbs::to_value;
use salvo::prelude::*;
use salvo::{Request, Response};
use std::collections::{HashMap, HashSet};

/*
 *添加用户信息
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn add_sys_user(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let item = req.parse_json::<AddUserReq>().await?;
    log::info!("add sys_user params: {:?}", &item);

    let rb = &mut RB.clone();
    let name = item.user_name;
    if User::select_by_user_name(rb, &name).await?.is_some() {
        return BaseResponse::<String>::err_result_msg(res, "登录账号已存在");
    }

    if User::select_by_mobile(rb, &item.mobile).await?.is_some() {
        return BaseResponse::<String>::err_result_msg(res, "手机号码已存在");
    }

    if User::select_by_email(rb, &item.email).await?.is_some() {
        return BaseResponse::<String>::err_result_msg(res, "邮箱账号已存在");
    }

    let avatar = item.avatar.unwrap_or(
        "https://gw.alipayobjects.com/zos/antfincdn/XAosXuNZyF/BiazfanxmamNRoxxVxka.png"
            .to_string(),
    );
    let sys_user = User {
        id: None,                          //主键
        mobile: item.mobile,               //手机
        user_name: name,                   //用户账号
        nick_name: item.nick_name,         //用户昵称
        user_type: Some("01".to_string()), //用户类型（00系统用户）
        email: item.email,                 //用户邮箱
        avatar,                            //头像路径
        password: item.password,           //密码
        status: item.status,               //状态(1:正常，0:禁用)
        dept_id: item.dept_id,             //部门ID
        login_ip: "".to_string(),          //最后登录IP
        login_date: None,                  //最后登录时间
        login_browser: "".to_string(),     //浏览器类型
        login_os: "".to_string(),          //操作系统
        pwd_update_date: None,             //密码最后更新时间
        remark: item.remark,               //备注
        del_flag: 1,                       //删除标志（0代表删除 1代表存在）
        create_time: None,                 //创建时间
        update_time: None,                 //修改时间
    };

    let id = User::insert(rb, &sys_user).await?.last_insert_id;

    let mut user_post_list: Vec<UserPost> = Vec::new();
    for post_id in item.post_ids {
        user_post_list.push(UserPost {
            user_id: id.i64(),
            post_id,
        })
    }

    let size = user_post_list.len() as u64;
    UserPost::insert_batch(rb, &user_post_list, size).await?;
    BaseResponse::<String>::ok_result(res)
}

/*
 *删除用户信息
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn delete_sys_user(
    depot: &mut Depot,
    req: &mut Request,
    res: &mut Response,
) -> AppResult<()> {
    let item = req.parse_json::<DeleteUserReq>().await?;
    log::info!("delete sys_user params: {:?}", &item);

    let user_id = depot.get::<i64>("userId").copied().unwrap();

    let ids = item.ids.clone();
    if ids.contains(&user_id) {
        return BaseResponse::<String>::err_result_msg(res, "当前用户不能删除");
    }
    if ids.contains(&1) {
        return BaseResponse::<String>::err_result_msg(res, "不允许操作超级管理员用户");
    }

    let rb = &mut RB.clone();
    UserRole::delete_in_column(rb, "user_id", &ids).await?;

    UserPost::delete_in_column(rb, "user_id", &ids).await?;

    User::delete_in_column(rb, "id", &item.ids).await?;
    BaseResponse::<String>::ok_result(res)
}

/*
 *更新用户信息
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn update_sys_user(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let item = req.parse_json::<UpdateUserReq>().await?;
    log::info!("update sys_user params: {:?}", &item);

    let id = item.id.clone();
    if id == 1 {
        return BaseResponse::<String>::err_result_msg(res, "不允许操作超级管理员用户");
    }

    let rb = &mut RB.clone();
    let user = match User::select_by_id(rb, item.id).await? {
        None => return BaseResponse::<String>::err_result_msg(res, "用户不存在"),
        Some(x) => x,
    };

    if let Some(x) = User::select_by_user_name(rb, &item.user_name).await? {
        if x.id.unwrap_or_default() != item.id {
            return BaseResponse::<String>::err_result_msg(res, "登录账号已存在");
        }
    }

    if let Some(x) = User::select_by_mobile(rb, &item.mobile).await? {
        if x.id.unwrap_or_default() != item.id {
            return BaseResponse::<String>::err_result_msg(res, "手机号码已存在");
        }
    }

    if let Some(x) = User::select_by_email(rb, &item.email).await? {
        if x.id.unwrap_or_default() != item.id {
            return BaseResponse::<String>::err_result_msg(res, "邮箱账号已存在");
        }
    }

    let avatar = item.avatar.unwrap_or(
        "https://gw.alipayobjects.com/zos/antfincdn/XAosXuNZyF/BiazfanxmamNRoxxVxka.png"
            .to_string(),
    );
    let sys_user = User {
        id: Some(item.id),                     //主键
        mobile: item.mobile,                   //手机
        user_name: item.user_name,             //用户账号
        nick_name: item.nick_name,             //用户昵称
        user_type: None,                       //用户类型（00系统用户）
        email: item.email,                     //用户邮箱
        avatar,                                //头像路径
        password: user.password,               //密码
        status: item.status,                   //状态(1:正常，0:禁用)
        dept_id: item.dept_id,                 //部门ID
        login_ip: user.login_ip,               //最后登录IP
        login_date: user.login_date,           //最后登录时间
        login_browser: user.login_browser,     //浏览器类型
        login_os: user.login_os,               //操作系统
        pwd_update_date: user.pwd_update_date, //密码最后更新时间
        remark: item.remark,                   //备注
        del_flag: user.del_flag,               //删除标志（0代表删除 1代表存在）
        create_time: None,                     //创建时间
        update_time: None,                     //修改时间
    };

    User::update_by_column(rb, &sys_user, "id").await?;
    let _ = UserPost::delete_by_column(rb, "user_id", &item.id).await;
    let mut user_post_list: Vec<UserPost> = Vec::new();
    for post_id in item.post_ids {
        user_post_list.push(UserPost {
            user_id: sys_user.id.unwrap_or_default(),
            post_id,
        })
    }

    UserPost::insert_batch(rb, &user_post_list, user_post_list.len() as u64).await?;
    BaseResponse::<String>::ok_result(res)
}

/*
 *更新用户信息状态
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn update_sys_user_status(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let item = req.parse_json::<UpdateUserStatusReq>().await?;
    log::info!("update sys_user_status params: {:?}", &item);

    let ids = item.ids.clone();
    if ids.contains(&1) {
        return BaseResponse::<String>::err_result_msg(res, "不允许操作超级管理员用户");
    }

    let update_sql = format!(
        "update sys_user set status = ? where id in ({})",
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
 *重置用户密码
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn reset_sys_user_password(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let item = req.parse_json::<ResetUserPwdReq>().await?;
    log::info!("update sys_user_password params: {:?}", &item);

    let id = item.id.clone();
    if id == 1 {
        return BaseResponse::<String>::err_result_msg(res, "不允许操作超级管理员用户");
    }

    let rb = &mut RB.clone();

    match User::select_by_id(rb, item.id).await? {
        None => BaseResponse::<String>::err_result_msg(res, "用户不存在"),
        Some(x) => {
            let mut user = x;
            user.password = item.password;
            User::update_by_column(rb, &user, "id").await?;
            BaseResponse::<String>::ok_result(res)
        }
    }
}

/*
 *用户修改自己的密码
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn update_sys_user_password(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> AppResult<()> {
    let item = req.parse_json::<UpdateUserPwdReq>().await?;
    log::info!("update sys_user_password params: {:?}", &item);
    let user_id = depot.get::<i64>("userId").copied().unwrap();

    let rb = &mut RB.clone();

    match User::select_by_id(rb, user_id).await? {
        None => BaseResponse::<String>::err_result_msg(res, "用户不存在"),
        Some(x) => {
            let mut user = x;
            if user.password != item.pwd {
                return BaseResponse::<String>::err_result_msg(res, "旧密码不正确");
            }
            user.password = item.re_pwd;

            User::update_by_column(rb, &user, "id").await?;
            BaseResponse::<String>::ok_result(res)
        }
    }
}

/*
 *查询用户信息详情
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn query_sys_user_detail(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let item = req.parse_json::<QueryUserDetailReq>().await?;
    log::info!("query sys_user_detail params: {:?}", &item);

    let rb = &mut RB.clone();

    let x = match User::select_by_id(rb, item.id).await? {
        None => {
            return BaseResponse::<QueryUserDetailResp>::err_result_data(
                res,
                QueryUserDetailResp::new(),
                "用户不存在",
            )
        }
        Some(user) => user,
    };

    let dept = match Dept::select_by_id(rb, &x.dept_id).await? {
        None => {
            return BaseResponse::<QueryUserDetailResp>::err_result_data(
                res,
                QueryUserDetailResp::new(),
                "查询用户详细信息失败,部门不存在",
            )
        }

        Some(y) => {
            QueryDeptDetailResp {
                id: y.id.unwrap_or_default(),               //部门id
                parent_id: y.parent_id,                     //父部门id
                ancestors: y.ancestors,                     //祖级列表
                dept_name: y.dept_name,                     //部门名称
                sort: y.sort,                               //显示顺序
                leader: y.leader,                           //负责人
                phone: y.phone,                             //联系电话
                email: y.email,                             //邮箱
                status: y.status,                           //部状态（0：停用，1:正常）
                del_flag: y.del_flag.unwrap_or_default(),   //删除标志（0代表删除 1代表存在）
                create_time: time_to_string(y.create_time), //创建时间
                update_time: time_to_string(y.update_time), //修改时间
            }
        }
    };

    let post_ids = UserPost::select_by_column(rb, "user_id", item.id)
        .await?
        .iter()
        .map(|x| x.post_id)
        .collect::<Vec<i64>>();

    let sys_user = QueryUserDetailResp {
        id: x.id.unwrap_or_default(),                       //主键
        mobile: x.mobile,                                   //手机
        user_name: x.user_name,                             //姓名
        nick_name: x.nick_name,                             //用户昵称
        user_type: x.user_type.unwrap_or_default(),         //用户类型（00系统用户）
        email: x.email,                                     //用户邮箱
        avatar: x.avatar,                                   //头像路径
        status: x.status,                                   //状态(1:正常，0:禁用)
        dept_id: x.dept_id,                                 //部门ID
        login_ip: x.login_ip,                               //最后登录IP
        login_date: time_to_string(x.login_date),           //最后登录时间
        login_browser: x.login_browser,                     //浏览器类型
        login_os: x.login_os,                               //操作系统
        pwd_update_date: time_to_string(x.pwd_update_date), //密码最后更新时间
        remark: x.remark,                                   //备注
        del_flag: x.del_flag,                               //删除标志（0代表删除 1代表存在）
        create_time: time_to_string(x.create_time),         //创建时间
        update_time: time_to_string(x.update_time),         //修改时间
        dept_info: dept,
        post_ids,
    };

    BaseResponse::<QueryUserDetailResp>::ok_result_data(res, sys_user)
}

/*
 *查询用户信息列表
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn query_sys_user_list(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let item = req.parse_json::<QueryUserListReq>().await?;
    log::info!("query sys_user_list params: {:?}", &item);
    
    let page = &PageRequest::new(item.page_no, item.page_size);
    let rb = &mut RB.clone();

    let d = User::select_sys_user_list(rb, page, &item).await?;
    let total = d.total;
    let mut list: Vec<UserListDataResp> = Vec::new();
    for x in d.records {
        list.push(UserListDataResp {
            id: x.id.unwrap_or_default(),                       //主键
            mobile: x.mobile,                                   //手机
            user_name: x.user_name,                             //姓名
            nick_name: x.nick_name,                             //用户昵称
            user_type: x.user_type.unwrap_or_default(),         //用户类型（00系统用户）
            email: x.email,                                     //用户邮箱
            avatar: x.avatar,                                   //头像路径
            status: x.status,                                   //状态(1:正常，0:禁用)
            dept_id: x.dept_id,                                 //部门ID
            login_ip: x.login_ip,                               //最后登录IP
            login_date: time_to_string(x.login_date),           //最后登录时间
            login_browser: x.login_browser,                     //浏览器类型
            login_os: x.login_os,                               //操作系统
            pwd_update_date: time_to_string(x.pwd_update_date), //密码最后更新时间
            remark: x.remark,                                   //备注
            del_flag: x.del_flag,                               //删除标志（0代表删除 1代表存在）
            create_time: time_to_string(x.create_time),         //创建时间
            update_time: time_to_string(x.update_time),         //修改时间
        })
    }

    BaseResponse::<Vec<UserListDataResp>>::ok_result_page(res, list, total)
}

/*
 *用户登录
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn login(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let item = req.parse_json::<UserLoginReq>().await?;
    log::info!("user login params: {:?}", &item);

    let user_agent = req.header::<&str>("user-agent").unwrap_or_default();
    log::info!("user agent: {:?}", user_agent);
    let agent = UserAgentUtil::new(user_agent);

    let rb = &mut RB.clone();
    let user_result = User::select_by_mobile(rb, &item.mobile).await?;
    log::info!("query user by mobile: {:?}", user_result);

    match user_result {
        None => {
            add_login_log(item.mobile, 0, "用户不存在", agent).await;
            BaseResponse::<String>::err_result_msg(res, "用户不存在")
        }
        Some(user) => {
            let mut s_user = user.clone();
            let id = user.id.unwrap_or_default();
            let username = user.user_name;
            let password = user.password;

            if password.ne(&item.password) {
                add_login_log(item.mobile, 0, "密码不正确", agent).await;
                return BaseResponse::<String>::err_result_msg(res, "密码不正确");
            }

            let btn_menu = query_btn_menu(&id).await;

            if btn_menu.len() == 0 {
                add_login_log(item.mobile, 0, "用户没有分配角色或者菜单,不能登录", agent).await;
                return BaseResponse::<String>::err_result_msg(
                    res,
                    "用户没有分配角色或者菜单,不能登录",
                );
            }

            let token = JwtToken::new(id, &username, btn_menu).create_token("123")?;

            add_login_log(item.mobile, 1, "登录成功", agent.clone()).await;
            s_user.login_os = agent.os;
            s_user.login_browser = agent.browser;
            s_user.login_date = Some(DateTime::now());

            User::update_by_column(rb, &s_user, "id").await?;
            BaseResponse::<String>::ok_result_data(res, token)
        }
    }
}

/*
 *添加登录日志
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
async fn add_login_log(name: String, status: i8, msg: &str, agent: UserAgentUtil) {
    let sys_login_log = LoginLog {
        id: None,                             //访问ID
        login_name: name,                     //登录账号
        ipaddr: "todo".to_string(),           //登录IP地址
        login_location: "todo".to_string(),   //登录地点
        platform: agent.platform,             //平台信息
        browser: agent.browser,               //浏览器类型
        version: agent.version,               //浏览器版本
        os: agent.os,                         //操作系统
        arch: agent.arch,                     //体系结构信息
        engine: agent.engine,                 //渲染引擎信息
        engine_details: agent.engine_details, //渲染引擎详细信息
        extra: agent.extra,                   //其他信息（可选）
        status,                               //登录状态(0:失败,1:成功)
        msg: msg.to_string(),                 //提示消息
        login_time: None,                     //访问时间
    };

    match LoginLog::insert(&mut RB.clone(), &sys_login_log).await {
        Ok(_u) => log::info!("add_login_log success: {:?}", sys_login_log),
        Err(err) => log::error!(
            "add_login_log error params: {:?}, error message: {:?}",
            sys_login_log,
            err
        ),
    }
}

/*
 *查询按钮权限
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
async fn query_btn_menu(id: &i64) -> Vec<String> {
    let mut btn_menu: Vec<String> = Vec::new();
    let rb = &mut RB.clone();
    let count = is_admin(rb, id).await.unwrap_or_default();
    if count == 1 {
        for x in Menu::select_all(rb).await.unwrap_or_default() {
            btn_menu.push(x.api_url.unwrap_or_default());
        }
        log::info!("The current user is a super administrator");
        btn_menu
    } else {
        let sql_str = "select distinct u.api_url from sys_user_role t left join sys_role usr on t.role_id = usr.id left join sys_role_menu srm on usr.id = srm.role_id left join sys_menu u on srm.menu_id = u.id where t.user_id = ?";
        let btn_menu_map = rb
            .query_decode::<Vec<HashMap<String, String>>>(sql_str, vec![to_value!(id)])
            .await
            .unwrap_or_default();
        for x in btn_menu_map {
            btn_menu.push(x.get("api_url").unwrap().to_string());
        }
        log::info!("The current user is not a super administrator");
        btn_menu
    }
}

/*
 *查询用户角色
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn query_user_role(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let item = req.parse_json::<QueryUserRoleReq>().await?;
    log::info!("query_user_role params: {:?}", item);

    let rb = &mut RB.clone();
    let mut role_ids: Vec<i64> = Vec::new();

    let user_id = item.user_id;
    for x in UserRole::select_by_column(rb, "user_id", user_id).await? {
        role_ids.push(x.role_id);
    }

    let mut list: Vec<RoleList> = Vec::new();

    for x in Role::select_all(rb).await? {
        list.push(RoleList {
            id: x.id.unwrap_or_default(),               //主键
            role_name: x.role_name,                     //名称
            role_key: x.role_key,                       //角色权限字符串
            data_scope: x.data_scope, //数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）
            status: x.status,         //状态(1:正常，0:禁用)
            remark: x.remark,         //备注
            del_flag: x.del_flag.unwrap_or_default(), //删除标志（0代表删除 1代表存在）
            create_time: time_to_string(x.create_time), //创建时间
            update_time: time_to_string(x.update_time), //修改时间
        });
    }

    BaseResponse::ok_result_data(
        res,
        QueryUserRoleResp {
            sys_role_list: list,
            user_role_ids: role_ids,
        },
    )
}

/*
 *更新用户与角色的关联
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn update_user_role(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let item = req.parse_json::<UpdateUserRoleReq>().await?;
    log::info!("update_user_role params: {:?}", item);

    let user_id = item.user_id.clone();
    let role_ids = &item.role_ids;
    let len = item.role_ids.len();

    if user_id.clone() == 1 {
        return BaseResponse::<String>::err_result_msg(res, "不能修改超级管理员的角色");
    }

    let rb = &mut RB.clone();

    UserRole::delete_by_column(rb, "user_id", user_id.clone()).await?;

    let mut sys_role_user_list: Vec<UserRole> = Vec::new();
    for role_id in role_ids {
        let r_id = role_id.clone();
        sys_role_user_list.push(UserRole {
            id: None,
            create_time: Some(DateTime::now()),
            role_id: r_id,
            user_id: user_id.clone(),
        })
    }

    UserRole::insert_batch(rb, &sys_role_user_list, len as u64).await?;
    BaseResponse::<String>::ok_result(res)
}

/*
 *查询用户菜单
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn query_user_menu(depot: &mut Depot, res: &mut Response) -> AppResult<()> {
    log::info!("query user menu params {:?}", depot);
    let user_id = depot.get::<i64>("userId").copied().unwrap();
    let username = depot.get::<String>("username").unwrap();
    log::info!("query user menu params {:?}", user_id);
    log::info!("query user menu params {:?}", username);

    //根据id查询用户
    let rb = &mut RB.clone();

    match User::select_by_id(rb, user_id).await? {
        None => BaseResponse::<String>::err_result_msg(res, "用户不存在"),
        Some(user) => {
            //role_id为1是超级管理员--判断是不是超级管理员
            let count = is_admin(rb, &user_id).await?;

            let sys_menu_list: Vec<Menu>;

            if count == 1 {
                log::info!("The current user is a super administrator");
                sys_menu_list = Menu::select_all(rb).await?;
            } else {
                log::info!("The current user is not a super administrator");
                let sql_str = "select u.* from sys_user_role t left join sys_role usr on t.role_id = usr.id left join sys_role_menu srm on usr.id = srm.role_id left join sys_menu u on srm.menu_id = u.id where t.user_id = ?";
                sys_menu_list = RB.query_decode(sql_str, vec![to_value!(user.id)]).await?;
            }

            let mut sys_menu: Vec<MenuList> = Vec::new();
            let mut btn_menu: Vec<String> = Vec::new();
            let mut sys_menu_ids: HashSet<i64> = HashSet::new();

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
            for menu in Menu::select_by_ids(rb, &menu_ids).await? {
                let api_url = menu.api_url.unwrap_or_default();
                sys_menu.push(MenuList {
                    id: menu.id.unwrap_or_default(),
                    parent_id: menu.parent_id,
                    name: menu.menu_name,
                    icon: menu.menu_icon.unwrap_or_default(),
                    api_url: api_url.clone(),
                    menu_type: menu.menu_type,
                    path: menu.menu_url.unwrap_or_default(),
                });

                if api_url.len() > 0 {
                    btn_menu.push(api_url);
                }
            }

            BaseResponse::<QueryUserMenuResp>::ok_result_data(res, QueryUserMenuResp {
                sys_menu,
                btn_menu,
                avatar: "https://gw.alipayobjects.com/zos/antfincdn/XAosXuNZyF/BiazfanxmamNRoxxVxka.png".to_string(),
                name: user.user_name,
            })
        }
    }
}
