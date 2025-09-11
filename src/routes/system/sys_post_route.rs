use crate::handler::system::sys_post_handler::*;
use salvo::Router;
/*
 *构建岗位信息表路由
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
pub fn build_sys_post_route() -> Router {
    Router::new()
        .push(Router::new().path("/system/post/addPost").post(add_sys_post))
        .push(Router::new().path("/system/post/deletePost").post(delete_sys_post))
        .push(Router::new().path("/system/post/updatePost").post(update_sys_post))
        .push(Router::new().path("/system/post/updatePostStatus").post(update_sys_post_status))
        .push(Router::new().path("/system/post/queryPostDetail").post(query_sys_post_detail))
        .push(Router::new().path("/system/post/queryPostList").post(query_sys_post_list))
    //记得在main.rs中的route()函数中添加构建岗位信息表路由build_sys_post_route()
}
