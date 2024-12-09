use crate::handler::system::menu_handler::*;
use salvo::Router;

pub fn build_menu_route() -> Router {
    Router::new()
        .push(Router::new().path("menu_list").post(menu_list))
        .push(Router::new().path("menu_save").post(menu_save))
        .push(Router::new().path("menu_update").post(menu_update))
        .push(Router::new().path("menu_delete").post(menu_delete))
}
