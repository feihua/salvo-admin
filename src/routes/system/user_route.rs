use crate::handler::system::user_handler::*;
use salvo::Router;
pub fn build_user_route() -> Router {
    Router::new()
        .push(Router::new().path("query_user_role").post(query_user_role))
        .push(
            Router::new()
                .path("update_user_role")
                .post(update_user_role),
        )
        .push(Router::new().path("query_user_menu").get(query_user_menu))
        .push(Router::new().path("user_list").post(user_list))
        .push(Router::new().path("user_save").post(user_save))
        .push(Router::new().path("user_update").post(user_update))
        .push(Router::new().path("user_delete").post(user_delete))
        .push(
            Router::new()
                .path("update_user_password")
                .post(update_user_password),
        )
}
