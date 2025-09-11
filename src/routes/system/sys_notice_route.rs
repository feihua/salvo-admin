use crate::handler::system::sys_notice_handler::*;
use salvo::Router;
/*
 *构建通知公告表路由
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
pub fn build_sys_notice_route() -> Router {
    Router::new()
        .push(Router::new().path("/system/notice/addNotice").post(add_sys_notice))
        .push(Router::new().path("/system/notice/deleteNotice").post(delete_sys_notice))
        .push(Router::new().path("/system/notice/updateNotice").post(update_sys_notice))
        .push(Router::new().path("/system/notice/updateNoticeStatus").post(update_sys_notice_status))
        .push(Router::new().path("/system/notice/queryNoticeDetail").post(query_sys_notice_detail))
        .push(Router::new().path("/system/notice/queryNoticeList").post(query_sys_notice_list))
    //记得在main.rs中的route()函数中添加构建通知公告表路由build_sys_notice_route()
}
