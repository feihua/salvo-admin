use crate::handler::system::sys_role_handler::*;
use salvo::Router;
/*
 *构建角色信息路由
 *author：刘飞华
 *date：2024/12/16 10:07:18
 */
pub fn build_sys_role_route() -> Router {
    Router::new()
        .push(Router::new().path("add_role").post(add_sys_role)) //添加角色信息
        .push(Router::new().path("delete_role").post(delete_sys_role)) //删除角色信息
        .push(Router::new().path("update_role").post(update_sys_role)) //更新角色信息
        .push(
            Router::new()
                .path("update_role_status")
                .post(update_sys_role_status),
        ) //更新角色信息状态
        .push(
            Router::new()
                .path("query_role_detail")
                .post(query_sys_role_detail),
        ) //查询角色信息详情
        .push(
            Router::new()
                .path("query_role_list")
                .post(query_sys_role_list),
        ) //查询角色信息列表
        .push(Router::new().path("query_role_menu").post(query_role_menu)) //查询角色菜单列表
        .push(
            Router::new()
                .path("update_role_menu")
                .post(update_role_menu),
        ) //更新角色菜单
          //记得在main.rs中的route()函数中添加构建角色信息路由build_sys_role_route()
}
