use chrono::{NaiveDate, NaiveDateTime};
use uuid::Uuid;

use crate::domain::entities::interest::Interest;

/// A hotel club member.
#[derive(Debug, Clone)]
pub struct Member {
    pub id: Uuid,
    pub name: String,
    pub surname: String,
    pub second_surname: String,
    pub email: String,
    pub birthdate: Option<NaiveDate>,
    pub phone: String,
    pub notes: String,
    pub gender_id: Option<i32>,
    pub country_id: Option<i32>,
    pub interests: Vec<Interest>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Member {
    pub fn full_name(&self) -> String {
        full_name(&self.name, &self.surname, &self.second_surname)
    }
}

pub fn full_name(name: &str, surname: &str, second_surname: &str) -> String {
    format!("{name} {surname} {second_surname}")
        .trim()
        .to_owned()
}

/// Data needed to create or update a member.
#[derive(Debug, Clone)]
pub struct MemberData {
    pub name: String,
    pub surname: String,
    pub second_surname: String,
    pub email: String,
    pub birthdate: Option<NaiveDate>,
    pub phone: String,
    pub notes: String,
    pub gender_id: Option<i32>,
    pub country_id: Option<i32>,
    pub interest_ids: Vec<i32>,
}

/// Search + interest filter shared by the paged list and the Excel export.
/// A member matches the interest filter only if it has *all* the given interests.
#[derive(Debug, Clone, Default)]
pub struct MemberFilter {
    pub search: String,
    pub interest_ids: Vec<i32>,
}

impl MemberFilter {
    pub fn new(search: impl Into<String>, mut interest_ids: Vec<i32>) -> Self {
        interest_ids.sort_unstable();
        interest_ids.dedup();
        Self {
            search: search.into().trim().to_owned(),
            interest_ids,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MemberSortField {
    Name,
    #[default]
    Surname,
    SecondSurname,
    Email,
    Phone,
    Birthdate,
    CreatedAt,
    UpdatedAt,
}

impl MemberSortField {
    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "name" => Self::Name,
            "surname" => Self::Surname,
            "secondSurname" => Self::SecondSurname,
            "email" => Self::Email,
            "phone" => Self::Phone,
            "birthdate" => Self::Birthdate,
            "createdAt" => Self::CreatedAt,
            "updatedAt" => Self::UpdatedAt,
            _ => return None,
        })
    }
}

/// A flattened member row for spreadsheet export.
#[derive(Debug, Clone)]
pub struct MemberExportRow {
    pub id: Uuid,
    pub name: String,
    pub surname: String,
    pub second_surname: String,
    pub email: String,
    pub birthdate: Option<NaiveDate>,
    pub phone: String,
    pub gender: Option<String>,
    pub country: Option<String>,
    pub interests: Vec<String>,
    pub notes: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_name_trims_missing_second_surname() {
        assert_eq!(full_name("Ana", "García", ""), "Ana García");
        assert_eq!(full_name("Ana", "García", "López"), "Ana García López");
    }

    #[test]
    fn filter_dedups_interests() {
        let f = MemberFilter::new("  x ", vec![3, 1, 3]);
        assert_eq!(f.search, "x");
        assert_eq!(f.interest_ids, vec![1, 3]);
    }
}
