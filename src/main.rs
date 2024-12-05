#[macro_use]
extern crate rbatis;

use once_cell::sync::Lazy;
use rbatis::RBatis;
use salvo::prelude::*;

use handler::system::menu_handler::{*};
use handler::system::role_handler::{*};
use handler::system::user_handler::{*};
use middleware::auth::auth_token;

pub mod model;
pub mod vo;
pub mod handler;
pub mod utils;
pub mod middleware;

pub static RB: Lazy<RBatis> = Lazy::new(RBatis::new);

#[handler]
async fn hello() -> &'static str {
    "Hello World123123"
}

#[tokio::main]
async fn main() {
    log4rs::init_file("src/config/log4rs.yaml", Default::default()).unwrap();
    // tracing_subscriber::fmt().init();

    let mysql_url = "mysql://root:oMbPi5munxCsBSsiLoPV@110.41.179.89:3306/salvodb";
    RB.init(rbdc_mysql::driver::MysqlDriver {}, mysql_url).unwrap();

    let acceptor = TcpListener::new("0.0.0.0:8100").bind().await;
    Server::new(acceptor).serve(route()).await;
}

fn route() -> Router {
    Router::new().path("/api").get(hello)
        .push(Router::new().path("login").post(login))
        .push(
            Router::new().hoop(auth_token)
                .push(Router::new().path("query_user_role").post(query_user_role))
                .push(Router::new().path("update_user_role").post(update_user_role))
                .push(Router::new().path("query_user_menu").get(query_user_menu))
                .push(Router::new().path("user_list").post(user_list))
                .push(Router::new().path("user_save").post(user_save))
                .push(Router::new().path("user_update").post(user_update))
                .push(Router::new().path("user_delete").post(user_delete))
                .push(Router::new().path("update_user_password").post(update_user_password))
                .push(Router::new().path("role_list").post(role_list))
                .push(Router::new().path("role_save").post(role_save))
                .push(Router::new().path("role_update").post(role_update))
                .push(Router::new().path("role_delete").post(role_delete))
                .push(Router::new().path("query_role_menu").post(query_role_menu))
                .push(Router::new().path("update_role_menu").post(update_role_menu))
                .push(Router::new().path("menu_list").post(menu_list))
                .push(Router::new().path("menu_save").post(menu_save))
                .push(Router::new().path("menu_update").post(menu_update))
                .push(Router::new().path("menu_delete").post(menu_delete))

        )
}