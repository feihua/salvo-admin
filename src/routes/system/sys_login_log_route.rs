use salvo::Router;
use crate::handler::system::sys_login_log_handler::*;
/*
 *构建系统访问记录路由
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
pub fn build_sys_login_log_route() -> Router {
    Router::new()
        .push(Router::new().path("/system/loginLog/deleteLoginLog").post(delete_sys_login_log))
        .push(Router::new().path("/system/loginLog/queryLoginLogDetail").post(query_sys_login_log_detail))
        .push(Router::new().path("/system/loginLog/queryLoginLogList").post(query_sys_login_log_list))
        //记得在main.rs中的route()函数中添加构建系统访问记录路由build_sys_login_log_route()
}
