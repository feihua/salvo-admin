use salvo::Router;
use crate::handler::system::sys_operate_log_handler::*;
/*
 *构建操作日志记录路由
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
pub fn build_sys_operate_log_route() -> Router {
    Router::new()
        .push(Router::new().path("/system/operateLog/deleteOperateLog").post(delete_sys_operate_log))
        .push(Router::new().path("/system/operateLog/queryOperateLogDetail").post(query_sys_operate_log_detail))
        .push(Router::new().path("/system/operateLog/queryOperateLogList").post(query_sys_operate_log_list))
        //记得在main.rs中的route()函数中添加构建操作日志记录路由build_sys_operate_log_route()
}
