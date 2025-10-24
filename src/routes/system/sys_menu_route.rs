use crate::handler::system::sys_menu_handler::*;
use salvo::Router;
/*
 *构建菜单信息路由
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
pub fn build_sys_menu_route() -> Router {
    Router::new()
        .push(Router::new().path("/system/menu/addMenu").post(add_sys_menu))
        .push(Router::new().path("/system/menu/deleteMenu").post(delete_sys_menu))
        .push(Router::new().path("/system/menu/updateMenu").post(update_sys_menu))
        .push(Router::new().path("/system/menu/updateMenuStatus").post(update_sys_menu_status))
        .push(Router::new().path("/system/menu/queryMenuDetail").post(query_sys_menu_detail))
        .push(Router::new().path("/system/menu/queryMenuList").post(query_sys_menu_list))
        .push(Router::new().path("/system/menu/queryMenuListSimple").post(query_sys_menu_list_simple))
        .push(Router::new().path("/system/menu/queryMenuResourceList").post(query_sys_menu_resource_list))
    //记得在main.rs中的route()函数中添加构建菜单信息路由build_sys_menu_route()
}
