#[macro_use]
extern crate rbatis;

use once_cell::sync::Lazy;
use rbatis::RBatis;
use salvo::prelude::*;

use crate::middleware::auth::auth_token;
use crate::routes::{build_other_route, build_system_route};
use handler::system::sys_user_handler::*;

pub mod common;
pub mod handler;
pub mod middleware;
pub mod model;
pub mod routes;
pub mod utils;
pub mod vo;

pub static RB: Lazy<RBatis> = Lazy::new(RBatis::new);

#[handler]
async fn hello() -> &'static str {
    "Hello World123123"
}

#[tokio::main]
async fn main() {
    log4rs::init_file("src/config/log4rs.yaml", Default::default()).unwrap();
    // tracing_subscriber::fmt().init();

    let mysql_url = "mysql://root:oMbPi5munxCsBSsiLoPV1@110.41.179.89:3306/axum";
    RB.init(rbdc_mysql::driver::MysqlDriver {}, mysql_url)
        .unwrap();

    let acceptor = TcpListener::new("0.0.0.0:8100").bind().await;
    Server::new(acceptor).serve(route()).await;
}

fn route() -> Router {
    Router::new()
        .path("/api")
        .get(hello)
        .push(Router::new().path("login").post(login))
        .push(
            Router::new()
                .hoop(auth_token)
                .push(build_system_route())
                .push(build_other_route()),
        )
}
