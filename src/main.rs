#[macro_use]
extern crate rbatis;

use crate::app_config::load_config::init_config;
use crate::app_config::load_db::init_db;
use crate::app_config::load_log::init_log;
use crate::routes::route;
use crate::utils::shutdown_signal::listen_shutdown_signal;
use salvo::prelude::*;

pub mod app_config;
pub mod common;
pub mod handler;
pub mod middleware;
pub mod model;
pub mod routes;
pub mod service;
pub mod utils;
pub mod vo;

#[tokio::main]
async fn main() {
    // 1.加载和解析配置文件
    let config = init_config("config.toml");
    // 2.初始化日志
    let _guard = init_log(config.log);
    // 初始化数据库连接
    init_db(config.db);
    // 3.创建TCP监听器并启动服务器
    let acceptor = TcpListener::new(config.server.addr).bind().await;

    let server = Server::new(acceptor);
    let handle = server.handle();

    tokio::spawn(listen_shutdown_signal(handle));

    server.serve(route(config.redis, config.jwt)).await;
}
