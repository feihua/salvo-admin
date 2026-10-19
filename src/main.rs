#[macro_use]
extern crate rbatis;

use crate::app_config::load_config::init_config;
use crate::app_config::load_log::init_log;
use crate::middleware::auth::auth_token;
use crate::routes::{build_other_route, build_system_route};
use handler::system::sys_user_handler::*;
use once_cell::sync::Lazy;
use rbatis::rbdc::pool::{ConnectionManager, Pool};
use rbatis::RBatis;
use rbdc_mysql::MysqlDriver;
use rbdc_pool_fast::FastPool;
use salvo::prelude::*;
use salvo::server::ServerHandle;
use tokio::signal;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

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

// 定义路由配置函数
fn route(url: &str, secret: String) -> Router {
    let cfg = deadpool_redis::Config::from_url(url);
    let pool = cfg.create_pool(Some(deadpool_redis::Runtime::Tokio1)).unwrap();

    // 创建路由实例，配置API路径和处理函数
    Router::new()
        .hoop(affix_state::insert("pool", pool).insert("secret", secret))
        .path("/api")
        .get(hello)
        .push(Router::new().path("/system/user/login").post(login))
        .push(Router::new().hoop(auth_token).push(build_system_route()).push(build_other_route()))
        .hoop(Logger::new())
}

async fn listen_shutdown_signal(handle: ServerHandle) {
    // Wait Shutdown Signal
    let ctrl_c = async {
        // Handle Ctrl+C signal
        signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        // Handle SIGTERM on Unix systems
        signal::unix::signal(signal::unix::SignalKind::terminate()).expect("failed to install signal handler").recv().await;
    };

    #[cfg(windows)]
    let terminate = async {
        // Handle Ctrl+C on Windows (alternative implementation)
        signal::windows::ctrl_c().expect("failed to install signal handler").recv().await;
    };

    tokio::select! {
        _ = ctrl_c => println!("ctrl_c signal received"),
        _ = terminate => println!("terminate signal received"),
    };

    handle.stop_graceful(None);
}
