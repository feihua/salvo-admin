use std::collections::HashMap;

use salvo::{Depot, FlowCtrl, Request, Response};
use salvo::prelude::*;

use crate::utils::jwt_util::JWTToken;
use crate::common::result::BaseResponse;

#[handler]
pub async fn auth_token(req: &mut Request, res: &mut Response, ctrl: &mut FlowCtrl, depot: &mut Depot) {
    let item = req.parse_headers::<HashMap<String, String>>().unwrap();
    let authorization = item.get("authorization");

    match authorization {
        None => {
            let resp = BaseResponse {
                msg: "token 不能为空".to_string(),
                code: 1,
                data: Some("None"),
            };
            ctrl.skip_rest();
            res.status_code(StatusCode::UNAUTHORIZED);
            return res.render(Json(resp));
        }
        Some(token) => {
            let split_vec = token.split_whitespace().collect::<Vec<_>>();
            if split_vec.len() != 2 || split_vec[0] != "Bearer" {
                let resp = BaseResponse {
                    msg: "the token format wrong".to_string(),
                    code: 1,
                    data: Some("None"),
                };
                ctrl.skip_rest();
                res.status_code(StatusCode::UNAUTHORIZED);
                return res.render(Json(resp));
            }
            let token = split_vec[1];
            let jwt_token_e = JWTToken::verify("123", &token);
            let jwt_token = match jwt_token_e {
                Ok(data) => { data }
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

            let path = req.uri().path().to_string();
            let mut flag: bool = false;
            for token_permission in &jwt_token.permissions {
                if token_permission.to_string() == path {
                    flag = true;
                    break;
                }
            }
            if flag {
                depot.insert("userId", jwt_token.id.clone());
                depot.insert("username", jwt_token.username.clone());
            } else {
                log::error!("Hi from start. You requested path: {:?}", path);
                let resp = BaseResponse {
                    msg: "你没有权限访问".to_string(),
                    code: 1,
                    data: Some(path),
                };
                ctrl.skip_rest();
                return res.render(Json(resp));
            }
        }
    }
}
