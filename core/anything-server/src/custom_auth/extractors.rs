use axum::{
    async_trait,
    extract::{FromRequestParts, Request},
    http::{request::Parts, StatusCode},
    RequestPartsExt,
};
use std::convert::Infallible;

use super::jwt::Claims;
use crate::entities::users;

/// Extractor for JWT Claims from request extensions
pub struct AuthClaims(pub Claims);

#[async_trait]
impl<S> FromRequestParts<S> for AuthClaims
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let claims = parts
            .extensions
            .get::<Claims>()
            .ok_or(StatusCode::UNAUTHORIZED)?
            .clone();

        Ok(AuthClaims(claims))
    }
}

/// Extractor for authenticated user from request extensions
pub struct AuthUser(pub users::Model);

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let user = parts
            .extensions
            .get::<users::Model>()
            .ok_or(StatusCode::UNAUTHORIZED)?
            .clone();

        Ok(AuthUser(user))
    }
}
