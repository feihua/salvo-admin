use crate::handler::system::sys_user_handler::login;
use crate::middleware::auth::auth_token;
use crate::routes::other::build_other_route;
use crate::routes::system::build_system_route;
use salvo::logging::Logger;
use salvo::{affix_state, Router};

pub mod other;
pub mod system;

// 定义路由配置函数
pub fn route(url: &str, secret: String) -> Router {
    let cfg = deadpool_redis::Config::from_url(url);
    let pool = cfg.create_pool(Some(deadpool_redis::Runtime::Tokio1)).unwrap();

    // 创建路由实例，配置API路径和处理函数
    Router::new()
        .hoop(affix_state::insert("pool", pool).insert("secret", secret))
        .path("/api")
        .push(Router::new().path("/system/user/login").post(login))
        .push(Router::new().hoop(auth_token).push(build_system_route()).push(build_other_route()))
        .hoop(Logger::new())
}
