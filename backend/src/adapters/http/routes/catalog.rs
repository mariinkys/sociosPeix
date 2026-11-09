use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::get};
use serde::{Serialize, ser::SerializeSeq};

use crate::{
    adapters::http::{app_state::AppState, extractors::AuthUser},
    app_error::AppResult,
    entities::catalog::{Country, Gender},
    use_cases::catalog::CatalogUseCases,
};

pub fn genders_router() -> Router<AppState> {
    Router::new().route("/", get(genders))
}

pub fn countries_router() -> Router<AppState> {
    Router::new().route("/", get(countries))
}

/// Serializes the cached catalog straight from the shared `Arc` as
/// `[{id, name}, ...]`, without copying it first.
struct CatalogList<T>(Arc<[T]>, fn(&T) -> (i32, &str));

#[derive(Serialize)]
struct Item<'a> {
    id: i32,
    name: &'a str,
}

impl<T> Serialize for CatalogList<T> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut seq = s.serialize_seq(Some(self.0.len()))?;
        for entry in self.0.iter() {
            let (id, name) = (self.1)(entry);
            seq.serialize_element(&Item { id, name })?;
        }
        seq.end()
    }
}

async fn genders(
    _user: AuthUser,
    State(catalog): State<Arc<CatalogUseCases>>,
) -> AppResult<Json<CatalogList<Gender>>> {
    Ok(Json(CatalogList(catalog.genders().await?, |g| {
        (g.id, &g.name)
    })))
}

async fn countries(
    _user: AuthUser,
    State(catalog): State<Arc<CatalogUseCases>>,
) -> AppResult<Json<CatalogList<Country>>> {
    Ok(Json(CatalogList(catalog.countries().await?, |c| {
        (c.id, &c.name)
    })))
}
