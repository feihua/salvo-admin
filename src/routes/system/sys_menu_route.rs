use crate::handler::system::sys_menu_handler::*;
use salvo::Router;
/*
 *构建菜单信息路由
 *author：刘飞华
 *date：2024/12/16 10:07:18
 */
pub fn build_sys_menu_route() -> Router {
    Router::new()
        .push(Router::new().path("system/menu/addMenu").post(add_sys_menu)) //添加菜单信息
        .push(
            Router::new()
                .path("system/menu/deleteMenu")
                .post(delete_sys_menu),
        ) //删除菜单信息
        .push(Router::new().path("system/menu/").post(update_sys_menu)) //更新菜单信息
        .push(
            Router::new()
                .path("system/menu/updateMenuStatus")
                .post(update_sys_menu_status),
        ) //更新菜单信息状态
        .push(
            Router::new()
                .path("system/menu/queryMenuDetail")
                .post(query_sys_menu_detail),
        ) //查询菜单信息详情
        .push(
            Router::new()
                .path("system/menu/queryMenuList")
                .post(query_sys_menu_list),
        ) //查询菜单信息列表
          //记得在main.rs中的route()函数中添加构建菜单信息路由build_sys_menu_route()
}
