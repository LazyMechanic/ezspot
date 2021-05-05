pub mod handlers;
pub mod middleware;
pub mod models;

pub use handlers::*;
pub use middleware::*;
pub use models::*;

pub const REFRESH_TOKEN_COOKIE_NAME: &str = "refreshToken";
pub const ACCESS_TOKEN_HEADER_NAME: &str = "Authorization";
pub const ACCESS_TOKEN_PREFIX: &str = "Bearer ";
