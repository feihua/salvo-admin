use crate::routes::system::menu_route::build_menu_route;
use crate::routes::system::role_route::build_role_route;
use crate::routes::system::user_route::build_user_route;
use salvo::Router;

pub mod other;
pub mod system;

pub fn build_system_route() -> Router {
    Router::new()
        .push(build_user_route())
        .push(build_role_route())
        .push(build_menu_route())
}

pub fn build_other_route() -> Router {
    Router::new()
}
