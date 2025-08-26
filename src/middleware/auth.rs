use salvo::prelude::*;
use salvo::{Depot, FlowCtrl, Request, Response};
use std::collections::HashMap;

use crate::common::result::BaseResponse;
use crate::utils::jwt_util::JwtToken;

#[handler]
pub async fn auth_token(req: &mut Request, res: &mut Response, ctrl: &mut FlowCtrl, depot: &mut Depot) {
    let item = req.parse_headers::<HashMap<String, String>>().unwrap();
    let authorization = item.get("authorization");
    let path = req.uri().path().to_string();
    log::info!("req url:{}", path);

    match authorization {
        None => {
            let resp = BaseResponse {
                msg: "token 不能为空".to_string(),
                code: 1,
                data: Some("None"),
            };
            ctrl.skip_rest();
            res.status_code(StatusCode::UNAUTHORIZED);
            res.render(Json(resp));
        }
        Some(token) => {
            let split_vec = token.split_whitespace().collect::<Vec<_>>();
            if split_vec.len() != 2 || split_vec[0] != "Bearer" {
                let resp = BaseResponse {
                    msg: "token 格式错误".to_string(),
                    code: 1,
                    data: Some("None"),
                };
                ctrl.skip_rest();
                res.status_code(StatusCode::UNAUTHORIZED);
                return res.render(Json(resp));
            }
            let token = split_vec[1];
            let secret = depot.get::<String>("secret").unwrap();
            let jwt_token_e = JwtToken::verify(secret, &token);
            let jwt_token = match jwt_token_e {
                Ok(data) => data,
                Err(err) => {
                    let resp = BaseResponse {
                        msg: err.to_string(),
                        code: 1,
                        data: Some("None"),
                    };
                    ctrl.skip_rest();
                    res.status_code(StatusCode::UNAUTHORIZED);
                    return res.render(Json(resp));
                }
            };

            let pool = depot.get::<deadpool_redis::Pool>("pool").unwrap();
            let mut conn = pool.get().await.unwrap();
            let key = format!("salvo:admin:user:info:{:?}", jwt_token.id);
            let values: HashMap<String, String> = deadpool_redis::redis::cmd("HGETALL").arg(key).query_async(&mut conn).await.unwrap_or_default();
            let token_1 = values.get("token").cloned().unwrap_or_default();
            if token != token_1 {
                let resp = BaseResponse {
                    msg: "无效的token".to_string(),
                    code: 1,
                    data: Some(""),
                };
                ctrl.skip_rest();
                return res.render(Json(resp))
            }
            let permissions_str = values.get("permissions").cloned().unwrap_or_default();
            let permissions: Vec<String> = if permissions_str.is_empty() {
                Vec::new()
            } else {
                permissions_str.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
            };

            if permissions.len()==0 {
                let resp = BaseResponse {
                    msg: "你没有权限访问".to_string(),
                    code: 1,
                    data: Some(path),
                };
                ctrl.skip_rest();
                return res.render(Json(resp))
            }
            let is_admin = values.get("is_admin").map(|v| v == "1").unwrap_or(false);

            if is_admin || has_permission(&permissions, path.as_str()) {
                depot.insert("userId", jwt_token.id.clone());
                depot.insert("username", jwt_token.username.clone());
            } else {
                log::error!("你没有权限访问: {:?}", path);
                let resp = BaseResponse {
                    msg: "你没有权限访问".to_string(),
                    code: 1,
                    data: Some(path),
                };
                ctrl.skip_rest();
                res.render(Json(resp))
            }
        }
    }
}

fn has_permission(permissions: &[String], path: &str) -> bool {
    permissions.iter().any(|permission| permission == path)
}
