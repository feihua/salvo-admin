#[macro_use]
extern crate rbatis;

pub mod model;
pub mod vo;
pub mod handler;
pub mod utils;

use once_cell::sync::Lazy;
use salvo::prelude::*;
use rbatis::RBatis;
use crate::handler::menu_handler::{*};
use crate::handler::role_handler::{*};
use crate::handler::user_handler::{*};
use crate::handler::banner_handler::{*};
use crate::handler::member_handler::{*};
use crate::handler::title_handler::{*};
use crate::handler::type_handler::{*};
use crate::utils::auth::auth_token;

pub static RB: Lazy<RBatis> = Lazy::new(RBatis::new);

#[handler]
async fn hello() -> &'static str {
    "Hello World123123"
}

#[tokio::main]
async fn main() {
    log4rs::init_file("src/config/log4rs.yaml", Default::default()).unwrap();
    // tracing_subscriber::fmt().init();

    let mysql_url = "mysql://root:ad879037-c7a4-4063-9236-6bfc35d54b7d@139.159.180.129:3306/rustdb";
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
                .push(Router::new().path("banner_list").post(banner_list))
                .push(Router::new().path("banner_save").post(banner_save))
                .push(Router::new().path("banner_update").post(banner_update))
                .push(Router::new().path("banner_delete").post(banner_delete))
                .push(Router::new().path("member_list").post(member_list))
                .push(Router::new().path("member_save").post(member_save))
                .push(Router::new().path("member_update").post(member_update))
                .push(Router::new().path("member_delete").post(member_delete))
                .push(Router::new().path("title_list").post(title_list))
                .push(Router::new().path("title_save").post(title_save))
                .push(Router::new().path("title_update").post(title_update))
                .push(Router::new().path("title_delete").post(title_delete))
                .push(Router::new().path("type_list").post(type_list))
                .push(Router::new().path("type_save").post(type_save))
                .push(Router::new().path("type_update").post(type_update))
                .push(Router::new().path("type_delete").post(type_delete))
        )
}