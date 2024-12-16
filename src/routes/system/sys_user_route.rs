use crate::handler::system::sys_user_handler::*;
use salvo::Router;
/*
 *构建用户信息路由
 *author：刘飞华
 *date：2024/12/16 10:07:18
 */
pub fn build_sys_user_route() -> Router {
    Router::new()
        .push(Router::new().path("add_user").post(add_sys_user)) //添加用户信息
        .push(Router::new().path("delete_user").post(delete_sys_user)) //删除用户信息
        .push(Router::new().path("update_user").post(update_sys_user)) //更新用户信息
        .push(
            Router::new()
                .path("update_user_status")
                .post(update_sys_user_status),
        ) //更新用户信息状态
        .push(
            Router::new()
                .path("query_user_detail")
                .post(query_sys_user_detail),
        ) //查询用户信息详情
        .push(
            Router::new()
                .path("query_user_list")
                .post(query_sys_user_list),
        ) //查询用户信息列表
        .push(
            Router::new()
                .path("update_user_password")
                .post(update_user_password),
        ) //修改密码
        .push(Router::new().path("query_user_role").post(query_user_role)) //查询用户角色信息列表
        .push(
            Router::new()
                .path("update_user_role")
                .post(update_user_role),
        )
        .push(Router::new().path("query_user_menu").get(query_user_menu)) //查询用户菜单列表
                                                                           //记得在main.rs中的route()函数中添加构建用户信息路由build_sys_user_route()
}
