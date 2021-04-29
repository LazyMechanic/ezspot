pub mod handlers;
pub mod middleware;
pub mod models;

pub use handlers::*;
pub use middleware::*;
pub use models::*;

const REFRESH_TOKEN_COOKIE_NAME: &str = "refreshToken";
const ACCESS_TOKEN_HEADER_NAME: &str = "Authorization";
const ACCESS_TOKEN_PREFIX: &str = "Bearer ";
