use crate::handler::system::sys_user_handler::*;
use salvo::Router;
/*
 *构建用户信息路由
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
pub fn build_sys_user_route() -> Router {
    Router::new()
        .push(Router::new().path("/system/user/addUser").post(add_sys_user))
        .push(Router::new().path("/system/user/deleteUser").post(delete_sys_user))
        .push(Router::new().path("/system/user/updateUser").post(update_sys_user))
        .push(Router::new().path("/system/user/updateUserStatus").post(update_sys_user_status))
        .push(Router::new().path("/system/user/queryUserDetail").post(query_sys_user_detail))
        .push(Router::new().path("/system/user/queryUserList").post(query_sys_user_list))
        .push(Router::new().path("/system/user/queryUserMenu").get(query_user_menu))
        .push(Router::new().path("/system/user/queryUserRole").post(query_user_role))
        .push(Router::new().path("/system/user/updateUserRole").post(update_user_role))
        .push(Router::new().path("/system/user/updateUserPassword").post(update_sys_user_password))
    //记得在main.rs中的route()函数中添加构建用户信息路由build_sys_user_route()
}
