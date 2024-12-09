use crate::handler::system::role_handler::*;
use salvo::Router;

pub fn build_role_route() -> Router {
    Router::new()
        .push(Router::new().path("role_list").post(role_list))
        .push(Router::new().path("role_save").post(role_save))
        .push(Router::new().path("role_update").post(role_update))
        .push(Router::new().path("role_delete").post(role_delete))
        .push(Router::new().path("query_role_menu").post(query_role_menu))
        .push(
            Router::new()
                .path("update_role_menu")
                .post(update_role_menu),
        )
}
