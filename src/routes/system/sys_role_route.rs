use crate::handler::system::sys_role_handler::*;
use salvo::Router;
/*
 *构建角色信息路由
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
pub fn build_sys_role_route() -> Router {
    Router::new()
        .push(Router::new().path("/system/role/addRole").post(add_sys_role))
        .push(Router::new().path("/system/role/deleteRole").post(delete_sys_role))
        .push(Router::new().path("/system/role/updateRole").post(update_sys_role))
        .push(Router::new().path("/system/role/updateRoleStatus").post(update_sys_role_status))
        .push(Router::new().path("/system/role/queryRoleDetail").post(query_sys_role_detail))
        .push(Router::new().path("/system/role/queryRoleList").post(query_sys_role_list))
        .push(Router::new().path("/system/role/queryRoleMenu").post(query_role_menu))
        .push(Router::new().path("/system/role/updateRoleMenu").post(update_role_menu))
        .push(Router::new().path("/system/role/queryAllocatedList").post(query_allocated_list))
        .push(Router::new().path("/system/role/queryUnallocatedList").post(query_unallocated_list))
        .push(Router::new().path("/system/role/cancelAuthUser").post(cancel_auth_user))
        .push(Router::new().path("/system/role/batchCancelAuthUser").post(batch_cancel_auth_user))
        .push(Router::new().path("/system/role/batchAuthUser").post(batch_auth_user))
    //记得在main.rs中的route()函数中添加构建角色信息路由build_sys_role_route()
}
