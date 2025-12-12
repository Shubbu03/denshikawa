use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use uuid::Uuid;
use crate::auth::{error::AuthError, jwt};
use crate::AppState;

#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub id: Uuid,
    pub email: String,
    pub role: String,
}

impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        let auth_header = parts
            .headers
            .get("Authorization")
            .ok_or(AuthError::MissingAuthHeader)?
            .to_str()
            .map_err(|_| AuthError::InvalidAuthHeader)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AuthError::InvalidAuthHeader)?;

        let claims = jwt::verify_access_token(token, &app_state.auth_config.jwt_secret)?;

        if claims.token_type != "access" {
            return Err(AuthError::TokenInvalid);
        }

        Ok(CurrentUser {
            id: claims.sub,
            email: claims.email,
            role: claims.role,
        })
    }
}
