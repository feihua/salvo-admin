use crate::common::result::BaseResponse;
use crate::utils::jwt_util::JwtToken;
use salvo::prelude::*;
use salvo::{Depot, FlowCtrl, Request, Response};
use std::collections::HashMap;

#[handler]
pub async fn auth_token(req: &mut Request, res: &mut Response, ctrl: &mut FlowCtrl, depot: &mut Depot) {
    let item = match req.parse_headers::<HashMap<String, String>>() {
        Ok(item) => item,
        Err(e) => {
            return er_res(res, ctrl, e.to_string().as_str());
        }
    };

    let authorization = item.get("authorization");
    let path = req.uri().path().to_string();
    log::info!("req url:{}", path);

    match authorization {
        None => er_res(res, ctrl, "token不能为空"),
        Some(token) => {
            let split_vec = token.split_whitespace().collect::<Vec<_>>();
            if split_vec.len() != 2 || split_vec[0] != "Bearer" {
                return er_res(res, ctrl, "token格式错误");
            }
            let token = split_vec[1];

            match depot.get::<String>("secret") {
                Ok(secret) => {
                    let jwt_token_e = JwtToken::verify(secret, &token);
                    let jwt_token = match jwt_token_e {
                        Ok(data) => data,
                        Err(err) => {
                            return er_res(res, ctrl, err.to_string().as_str());
                        }
                    };

                    if let Ok(pool) = depot.get::<deadpool_redis::Pool>("pool") {
                        if let Ok(mut conn) = pool.get().await {
                            let key = format!("salvo:admin:user:info:{:?}", jwt_token.id);
                            let values: HashMap<String, String> = deadpool_redis::redis::cmd("HGETALL").arg(key).query_async(&mut conn).await.unwrap_or_default();
                            let token_1 = values.get("token").cloned().unwrap_or_default();
                            if token != token_1 {
                                return er_res(res, ctrl, "无效的token");
                            }
                            let permissions_str = values.get("permissions").cloned().unwrap_or_default();
                            let permissions: Vec<String> = if permissions_str.is_empty() {
                                Vec::new()
                            } else {
                                permissions_str.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
                            };

                            if permissions.len() == 0 {
                                return er_res(res, ctrl, format!("你没有权限访问: {}", path).as_str());
                            }
                            let is_admin = values.get("is_admin").map(|v| v == "1").unwrap_or(false);

                            if is_admin || has_permission(&permissions, path.as_str()) {
                                depot.insert("userId", jwt_token.id.clone());
                                depot.insert("username", jwt_token.username.clone());
                            } else {
                                log::error!("你没有权限访问: {:?}", path);
                                er_res(res, ctrl, format!("你没有权限访问: {}", path).as_str())
                            }
                        } else {
                            er_res(res, ctrl, "获取redis conn失败")
                        }
                    } else {
                        er_res(res, ctrl, "获取redis pool异常")
                    }
                }
                Err(_) => er_res(res, ctrl, "获取jwt密钥异常"),
            }
        }
    }
}

fn er_res(res: &mut Response, ctrl: &mut FlowCtrl, msg: &str) {
    let resp = BaseResponse {
        msg: msg.to_string(),
        code: 1,
        data: Some("None"),
    };
    ctrl.skip_rest();
    res.render(Json(resp));
}

fn has_permission(permissions: &[String], path: &str) -> bool {
    permissions.iter().any(|permission| permission == path)
}
