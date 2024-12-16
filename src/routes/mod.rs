use crate::routes::system::sys_menu_route::build_sys_menu_route;
use crate::routes::system::sys_role_route::build_sys_role_route;
use crate::routes::system::sys_user_route::build_sys_user_route;
use salvo::Router;

pub mod other;
pub mod system;

pub fn build_system_route() -> Router {
    Router::new()
        .push(build_sys_user_route())
        .push(build_sys_role_route())
        .push(build_sys_menu_route())
}

pub fn build_other_route() -> Router {
    Router::new()
}
