use once_cell::sync::Lazy;
use rbatis::rbdc::pool::{ConnectionManager, Pool};
use rbatis::RBatis;
use rbdc_mysql::MysqlDriver;
use rbdc_pool_fast::FastPool;

pub static RB: Lazy<RBatis> = Lazy::new(RBatis::new);

///初始化数据库连接
pub fn init_db(db_url: &str) {
    //
    let manager = ConnectionManager::new(MysqlDriver {}, db_url).expect("create connection manager error");
    let pool = FastPool::new(manager).expect("create db pool error");

    RB.init_pool(pool).expect("init db pool error")
}
