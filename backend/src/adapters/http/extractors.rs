use std::sync::Arc;

use axum::{
    Json,
    extract::{FromRef, FromRequest, FromRequestParts, Query, Request},
    http::request::Parts,
};
use axum_extra::extract::CookieJar;
use serde::de::DeserializeOwned;

use crate::{
    adapters::http::{app_state::AppState, cookies::ACCESS_TOKEN, validation::Validate},
    app_error::AppError,
    entities::user::{Requester, User},
    use_cases::auth::AuthUseCases,
};

/// The signed-in user, resolved from the `access_token` cookie.
/// Rejects with 401 when there's no valid session.
pub struct AuthUser(pub User);

impl AuthUser {
    pub fn requester(&self) -> Requester {
        Requester {
            id: self.0.id,
            is_admin: self.0.is_admin(),
        }
    }
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        if let Some(user) = parts.extensions.get::<CachedUser>() {
            return Ok(AuthUser(user.0.clone()));
        }
        let jar = CookieJar::from_headers(&parts.headers);
        let token = jar.get(ACCESS_TOKEN).ok_or(AppError::Unauthenticated)?;
        let auth: Arc<AuthUseCases> = AppState::from_ref(state).auth;
        let user = auth.authenticate(token.value()).await?;
        parts.extensions.insert(CachedUser(user.clone()));
        Ok(AuthUser(user))
    }
}

/// Avoids a second DB lookup if a handler extracts the user twice.
#[derive(Clone)]
struct CachedUser(User);

/// Like [`AuthUser`] but additionally requires the ADMIN role (403 otherwise).
pub struct AdminUser(pub User);

impl<S> FromRequestParts<S> for AdminUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let AuthUser(user) = AuthUser::from_request_parts(parts, state).await?;
        if !user.is_admin() {
            return Err(AppError::Forbidden("Access denied".into()));
        }
        Ok(AdminUser(user))
    }
}

/// JSON body that is deserialized *and* validated; any problem becomes a 400
/// with the usual `{status, message}` body.
pub struct ValidJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e| AppError::Validation(e.body_text()))?;
        value.validate().map_err(AppError::Validation)?;
        Ok(ValidJson(value))
    }
}

/// Query string extractor whose errors are 400s in the usual JSON format.
pub struct Q<T>(pub T);

impl<S, T> FromRequestParts<S> for Q<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Query(v) = Query::<T>::from_request_parts(parts, state)
            .await
            .map_err(|e| AppError::Validation(e.body_text()))?;
        Ok(Q(v))
    }
}

/// Collects an integer list from the query string, accepting every shape
/// clients send: `ids=1&ids=2`, `ids[]=1&ids[]=2` (axios' default) and
/// `ids=1,2`.
pub fn int_list(query: Option<&str>, key: &str) -> Result<Vec<i32>, AppError> {
    let Some(query) = query else {
        return Ok(Vec::new());
    };
    let bracketed = format!("{key}[]");
    let mut out = Vec::new();
    for (k, v) in form_urlencoded::parse(query.as_bytes()) {
        if k != key && k != bracketed {
            continue;
        }
        for part in v.split(',').map(str::trim).filter(|p| !p.is_empty()) {
            out.push(part.parse().map_err(|_| {
                AppError::Validation(format!("Invalid value '{part}' for parameter '{key}'"))
            })?);
        }
    }
    Ok(out)
}

/// Path extractor whose errors are 400s in the usual JSON format.
pub struct P<T>(pub T);

impl<S, T> FromRequestParts<S> for P<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Send,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let axum::extract::Path(v) = axum::extract::Path::<T>::from_request_parts(parts, state)
            .await
            .map_err(|e| AppError::Validation(e.body_text()))?;
        Ok(P(v))
    }
}

#[cfg(test)]
mod tests {
    use super::int_list;

    #[test]
    fn int_list_accepts_all_shapes() {
        assert_eq!(
            int_list(Some("interestIds=1&interestIds=2"), "interestIds").unwrap(),
            [1, 2]
        );
        assert_eq!(
            int_list(Some("interestIds%5B%5D=3&interestIds[]=4"), "interestIds").unwrap(),
            [3, 4]
        );
        assert_eq!(
            int_list(Some("interestIds=5,6&page=0"), "interestIds").unwrap(),
            [5, 6]
        );
        assert!(int_list(Some("interestIds=x"), "interestIds").is_err());
        assert!(int_list(None, "interestIds").unwrap().is_empty());
    }
}
