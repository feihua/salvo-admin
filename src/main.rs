#[macro_use]
extern crate rbatis;

use crate::app_config::load_config::init_config;
use crate::app_config::load_log::init_log;
use crate::routes::route;
use crate::utils::shutdown_signal::listen_shutdown_signal;
use once_cell::sync::Lazy;
use rbatis::rbdc::pool::{ConnectionManager, Pool};
use rbatis::RBatis;
use rbdc_mysql::MysqlDriver;
use rbdc_pool_fast::FastPool;
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

pub static RB: Lazy<RBatis> = Lazy::new(RBatis::new);

#[handler]
async fn hello() -> &'static str {
    "Hello World123123"
}
// 定义应用状态结构体，包含数据库连接池
pub struct AppState {
    pub batis: RBatis,
}

// 主函数，异步运行
#[tokio::main]
async fn main() {
    // 初始化日志
    let _guard = init_log("logs", "app.log");
    // 加载和解析配置文件
    let config = init_config("config.toml");

    // 初始化数据库连接
    let manager = ConnectionManager::new(MysqlDriver {}, config.db.url.as_str()).expect("create connection manager error");
    let pool = FastPool::new(manager).expect("create db pool error");

    RB.init_pool(pool).expect("init db pool error");

    // 创建TCP监听器并启动服务器
    let acceptor = TcpListener::new(config.server.addr).bind().await;

    let server = Server::new(acceptor);
    let handle = server.handle();

    tokio::spawn(listen_shutdown_signal(handle));

    server.serve(route(config.redis.url.as_str(), config.jwt.secret)).await;
}
