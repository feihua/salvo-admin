use salvo::Router;
pub mod sys_dept_route;
pub mod sys_dict_data_route;
pub mod sys_dict_type_route;
pub mod sys_login_log_route;
pub mod sys_menu_route;
pub mod sys_notice_route;
pub mod sys_operate_log_route;
pub mod sys_post_route;
pub mod sys_role_route;
pub mod sys_user_route;

use crate::routes::system::sys_dept_route::build_sys_dept_route;
use crate::routes::system::sys_dict_data_route::build_sys_dict_data_route;
use crate::routes::system::sys_dict_type_route::build_sys_dict_type_route;
use crate::routes::system::sys_login_log_route::build_sys_login_log_route;
use crate::routes::system::sys_menu_route::build_sys_menu_route;
use crate::routes::system::sys_notice_route::build_sys_notice_route;
use crate::routes::system::sys_operate_log_route::build_sys_operate_log_route;
use crate::routes::system::sys_post_route::build_sys_post_route;
use crate::routes::system::sys_role_route::build_sys_role_route;
use crate::routes::system::sys_user_route::build_sys_user_route;

pub fn build_system_route() -> Router {
    Router::new()
        .push(build_sys_user_route())
        .push(build_sys_role_route())
        .push(build_sys_menu_route())
        .push(build_sys_dept_route())
        .push(build_sys_dict_data_route())
        .push(build_sys_dict_type_route())
        .push(build_sys_login_log_route())
        .push(build_sys_operate_log_route())
        .push(build_sys_post_route())
        .push(build_sys_notice_route())
}
