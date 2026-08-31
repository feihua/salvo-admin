// author：刘飞华
// createTime：2024/12/12 14:41:44

use rbatis::RBatis;
use rbs::value;
use serde::{Deserialize, Serialize};
use tracing::error;
/*
 *角色用户关联表
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserRole {
    pub id: Option<i64>, //主键
    pub user_id: i64,    //用户ID
    pub role_id: i64,    //角色ID
}

/*
 *角色用户关联表基本操作
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
rbatis::crud!(UserRole {}, "sys_user_role");

impl UserRole {
    /*
     *查询是否为超级管理员(role_id=1是预设超级管理的id)
     *author：刘飞华
     *date：2024/12/12 14:41:44
     */
    pub async fn is_admin(rb: &RBatis, user_id: &i64) -> rbatis::Result<bool> {
        UserRole::select_by_map(rb, value! {"user_id": user_id, "role_id": 1}).await.map(|x| Ok(x.len() > 0)).map_err(|e| {
            error!("Database error update sys_user: {:?}", e);
            e
        })?
    }

    /*
     *通过角色id查询角色使用数量
     *author：刘飞华
     *date：2024/12/12 14:41:44
     */
    #[sql("select count(1) from sys_user_role where role_id = ?")]
    pub async fn count_user_role_by_role_id(rb: &RBatis, role_id: &i64) -> rbatis::Result<i64> {
        impled!()
    }

    /*
     *通过角色id和用户id删除
     *author：刘飞华
     *date：2024/12/12 14:41:44
     */
    #[sql("delete from sys_user_role where role_id = ? and user_id = ?")]
    pub async fn delete_user_role_by_role_id_user_id(rb: &RBatis, role_id: i64, user_id: i64) -> Option<i64> {
        impled!()
    }
}
