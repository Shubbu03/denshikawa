pub mod error;
pub mod jwt;
pub mod middleware;
pub mod password;

pub use error::AuthError;
pub use jwt::{AccessTokenClaims, RefreshTokenClaims, TokenPair};
pub use middleware::CurrentUser;
pub use password::{hash_password, verify_password};
