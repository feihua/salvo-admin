use crate::handler::system::sys_dept_handler::*;
use salvo::Router;
/*
 *构建部门表路由
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
pub fn build_sys_dept_route() -> Router {
    Router::new()
        .push(Router::new().path("/system/dept/addDept").post(add_sys_dept))
        .push(Router::new().path("/system/dept/deleteDept").post(delete_sys_dept))
        .push(Router::new().path("/system/dept/updateDept").post(update_sys_dept))
        .push(Router::new().path("/system/dept/updateDeptStatus").post(update_sys_dept_status))
        .push(Router::new().path("/system/dept/queryDeptDetail").post(query_sys_dept_detail))
        .push(Router::new().path("/system/dept/queryDeptList").post(query_sys_dept_list))
    //记得在main.rs中的route()函数中添加构建部门表路由build_sys_dept_route()
}
