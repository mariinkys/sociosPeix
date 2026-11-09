//! Response bodies shared by several route modules. Field names are camelCase
//! to match the frontend's TypeScript types exactly.

use chrono::{NaiveDateTime, SecondsFormat};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    domain::pagination::Page,
    entities::{interest::Interest, user::User},
};

/// Stored timestamps are naive UTC; send them with an explicit `Z` so
/// browsers convert them to the viewer's local time.
pub fn utc_timestamp(t: NaiveDateTime) -> String {
    t.and_utc().to_rfc3339_opts(SecondsFormat::Millis, true)
}

pub fn serialize_utc<S: serde::Serializer>(t: &NaiveDateTime, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&utc_timestamp(*t))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResponse<T> {
    pub content: Vec<T>,
    pub page: u32,
    pub size: u32,
    pub total_elements: i64,
    pub total_pages: i64,
    pub last: bool,
}

impl<T> PageResponse<T> {
    pub fn from_page<U>(page: Page<U>, f: impl FnMut(U) -> T) -> Self {
        let total_pages = page.total_pages();
        let last = page.is_last();
        let (number, size, total) = (page.request.page, page.request.size, page.total);
        Self {
            content: page.items.into_iter().map(f).collect(),
            page: number,
            size,
            total_elements: total,
            total_pages,
            last,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: &'static str,
    #[serde(serialize_with = "serialize_utc")]
    pub created_at: NaiveDateTime,
}

impl From<User> for UserResponse {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            role: u.role.as_str(),
            name: u.name,
            email: u.email,
            created_at: u.created_at,
        }
    }
}

#[derive(Serialize)]
pub struct InterestResponse {
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl From<Interest> for InterestResponse {
    fn from(i: Interest) -> Self {
        Self {
            id: i.id,
            name: i.name,
            description: i.description,
        }
    }
}

impl From<&Interest> for InterestResponse {
    fn from(i: &Interest) -> Self {
        i.clone().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamps_are_utc_iso() {
        let t = chrono::NaiveDate::from_ymd_opt(2026, 11, 9)
            .unwrap()
            .and_hms_micro_opt(9, 5, 3, 123_456)
            .unwrap();
        assert_eq!(utc_timestamp(t), "2026-11-09T09:05:03.123Z");
    }
}
