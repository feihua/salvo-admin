use crate::common::result::BaseResponse;
use salvo::prelude::Json;
use salvo::{Depot, Request, Response, Writer};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    // #[error("Failed to complete an HTTP request")]
    // Http { #[from] source: reqwest::Error },
    //
    #[error("Failed to read the cache file")]
    DiskCacheRead { source: std::io::Error },
    //
    // #[error("Failed to update the cache file")]
    // DiskCacheWrite { source: std::io::Error },
    #[error("")]
    JwtTokenError(String),

    #[error("解析请求参数错误: {0}")]
    ParseError(#[from] salvo::http::ParseError),

    #[error("数据库错误: {0}")]
    DbError(#[from] rbatis::Error),

    #[error("业务异常: {0}")]
    BusinessError(&'static str),
}
pub type AppResult<T> = Result<T, AppError>;

#[async_trait]
impl Writer for AppError {
    async fn write(mut self, _req: &mut Request, depot: &mut Depot, res: &mut Response) {
        let user_id = depot.get::<i64>("userId").copied().unwrap();
        let username = depot.get::<String>("username").unwrap();
        log::info!("query user user_id params {:?}", user_id);
        log::info!("query user username params {:?}", username);

        res.render(Json(BaseResponse {
            msg: self.to_string(),
            code: 0,
            data: Some("None".to_string()),
        }))
    }
}
