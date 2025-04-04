#[macro_use]
extern crate rbatis;

use crate::middleware::auth::auth_token;
use crate::routes::{build_other_route, build_system_route};
use config::{Config, File};
use handler::system::sys_user_handler::*;
use once_cell::sync::Lazy;
use rbatis::RBatis;
use salvo::prelude::*;
use serde::Deserialize;

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
// 定义应用状态结构体，包含数据库连接池
pub struct AppState {
    pub batis: RBatis,
}

// 定义配置结构体，包含服务器和数据库配置
#[derive(Debug, Deserialize)]
struct Config1 {
    server: ServerConfig,
    db: DbConfig,
}

// 定义服务器配置结构体
#[derive(Debug, Deserialize)]
struct ServerConfig {
    addr: String,
}

// 定义数据库配置结构体
#[derive(Debug, Deserialize)]
struct DbConfig {
    url: String,
}

// 主函数，异步运行
#[tokio::main]
async fn main() {
    // 初始化日志系统
    log4rs::init_file("src/config/log4rs.yaml", Default::default()).unwrap();
    // tracing_subscriber::fmt().init();
    
    // 加载和解析配置文件
    let config = Config::builder()
        .add_source(File::with_name("config.toml"))
        .build()
        .unwrap()
        .try_deserialize::<Config1>()
        .unwrap();
    println!("Config: {:?}", config);

    // 初始化数据库连接
    RB.init(rbdc_mysql::driver::MysqlDriver {}, config.db.url.as_str())
        .unwrap();

    // 创建TCP监听器并启动服务器
    let acceptor = TcpListener::new(config.server.addr).bind().await;
    Server::new(acceptor).serve(route()).await;
}

// 定义路由配置函数
fn route() -> Router {
    // 创建路由实例，配置API路径和处理函数
    Router::new()
        .path("/api")
        .get(hello)
        .push(Router::new().path("/system/user/login").post(login))
        .push(
            Router::new()
                .hoop(auth_token)
                .push(build_system_route())
                .push(build_other_route()),
        )
}
