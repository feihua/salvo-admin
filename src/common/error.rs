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
}
