use std::collections::HashMap;

use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::{AssertSqlSafe, PgExecutor};
use uuid::Uuid;

use crate::{
    adapters::persistence::{PostgresPersistence, like_pattern},
    app_error::AppResult,
    domain::pagination::{Page, PageRequest, SortDirection},
    entities::{
        interest::Interest,
        member::{Member, MemberData, MemberExportRow, MemberFilter, MemberSortField},
    },
    use_cases::member::MemberPersistence,
};

#[derive(sqlx::FromRow)]
struct MemberDb {
    id: Uuid,
    name: String,
    surname: String,
    second_surname: String,
    email: String,
    birthdate: Option<NaiveDate>,
    phone: String,
    notes: String,
    emails_disabled: bool,
    gender_id: Option<i32>,
    country_id: Option<i32>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl MemberDb {
    fn into_member(self, interests: Vec<Interest>) -> Member {
        Member {
            id: self.id,
            name: self.name,
            surname: self.surname,
            second_surname: self.second_surname,
            email: self.email,
            birthdate: self.birthdate,
            phone: self.phone,
            notes: self.notes,
            emails_disabled: self.emails_disabled,
            gender_id: self.gender_id,
            country_id: self.country_id,
            interests,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct MemberExportDb {
    id: Uuid,
    name: String,
    surname: String,
    second_surname: String,
    email: String,
    birthdate: Option<NaiveDate>,
    phone: String,
    gender: Option<String>,
    country: Option<String>,
    interests: Vec<String>,
    notes: String,
    emails_disabled: bool,
}

const COLUMNS: &str = "m.id, m.name, m.surname, m.second_surname, m.email, m.birthdate, m.phone, \
                       m.notes, m.gender_id, m.country_id, m.emails_disabled, m.created_at, m.updated_at";

/// Shared WHERE clause. Binds: $1 search, $2 ILIKE pattern, $3 interest ids.
/// A member must have *all* the requested interests to match.
const FILTER: &str = "($1 = '' OR m.name ILIKE $2 OR m.surname ILIKE $2 \
                       OR m.second_surname ILIKE $2 OR m.email ILIKE $2)
    AND (cardinality($3::int4[]) = 0 OR
         (SELECT COUNT(*) FROM member_interests mi
          WHERE mi.member_id = m.id AND mi.interest_id = ANY($3)) = cardinality($3::int4[]))";

fn sort_column(field: MemberSortField) -> &'static str {
    match field {
        MemberSortField::Name => "m.name",
        MemberSortField::Surname => "m.surname",
        MemberSortField::SecondSurname => "m.second_surname",
        MemberSortField::Email => "m.email",
        MemberSortField::Phone => "m.phone",
        MemberSortField::Birthdate => "m.birthdate",
        MemberSortField::CreatedAt => "m.created_at",
        MemberSortField::UpdatedAt => "m.updated_at",
    }
}

/// Loads the interests of many members in one query.
async fn interests_by_member(
    executor: impl PgExecutor<'_>,
    member_ids: &[Uuid],
) -> AppResult<HashMap<Uuid, Vec<Interest>>> {
    if member_ids.is_empty() {
        return Ok(HashMap::new());
    }
    let rows: Vec<(Uuid, i32, String, String)> = sqlx::query_as(
        "SELECT mi.member_id, i.id, i.name, i.description
         FROM member_interests mi JOIN interests i ON i.id = mi.interest_id
         WHERE mi.member_id = ANY($1)
         ORDER BY i.id",
    )
    .bind(member_ids)
    .fetch_all(executor)
    .await?;

    let mut map: HashMap<Uuid, Vec<Interest>> = HashMap::with_capacity(member_ids.len());
    for (member_id, id, name, description) in rows {
        map.entry(member_id).or_default().push(Interest {
            id,
            name,
            description,
        });
    }
    Ok(map)
}

impl PostgresPersistence {
    async fn with_interests(&self, rows: Vec<MemberDb>) -> AppResult<Vec<Member>> {
        let ids: Vec<Uuid> = rows.iter().map(|r| r.id).collect();
        let mut interests = interests_by_member(&self.pool, &ids).await?;
        Ok(rows
            .into_iter()
            .map(|r| {
                let i = interests.remove(&r.id).unwrap_or_default();
                r.into_member(i)
            })
            .collect())
    }
}

async fn link_interests(
    executor: impl PgExecutor<'_>,
    member_id: Uuid,
    interest_ids: &[i32],
) -> AppResult<()> {
    if interest_ids.is_empty() {
        return Ok(());
    }
    // Unknown ids are silently ignored, as before.
    sqlx::query(
        "INSERT INTO member_interests (member_id, interest_id)
         SELECT $1, id FROM interests WHERE id = ANY($2)
         ON CONFLICT DO NOTHING",
    )
    .bind(member_id)
    .bind(interest_ids)
    .execute(executor)
    .await?;
    Ok(())
}

#[async_trait]
impl MemberPersistence for PostgresPersistence {
    async fn find_member(&self, id: Uuid) -> AppResult<Option<Member>> {
        let sql = format!("SELECT {COLUMNS} FROM members m WHERE m.id = $1");
        let Some(row) = sqlx::query_as::<_, MemberDb>(AssertSqlSafe(sql))
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
        else {
            return Ok(None);
        };
        Ok(self.with_interests(vec![row]).await?.pop())
    }

    async fn list_members(
        &self,
        filter: &MemberFilter,
        (field, dir): (MemberSortField, SortDirection),
        page: PageRequest,
    ) -> AppResult<Page<Member>> {
        let pattern = like_pattern(&filter.search);

        let total: i64 = sqlx::query_scalar(AssertSqlSafe(format!(
            "SELECT COUNT(*) FROM members m WHERE {FILTER}"
        )))
        .bind(&filter.search)
        .bind(&pattern)
        .bind(&filter.interest_ids)
        .fetch_one(&self.pool)
        .await?;

        // Sorting and paging happen in the database (the old backend sorted
        // only within the already-sliced page, so pages were out of order).
        // The ORDER BY column comes from a closed enum, never from user input.
        let sql = format!(
            "SELECT {COLUMNS} FROM members m WHERE {FILTER}
             ORDER BY {} {}, m.id LIMIT $4 OFFSET $5",
            sort_column(field),
            dir.as_sql()
        );
        let rows = sqlx::query_as::<_, MemberDb>(AssertSqlSafe(sql))
            .bind(&filter.search)
            .bind(&pattern)
            .bind(&filter.interest_ids)
            .bind(page.limit())
            .bind(page.offset())
            .fetch_all(&self.pool)
            .await?;

        Ok(Page {
            items: self.with_interests(rows).await?,
            request: page,
            total,
        })
    }

    async fn export_members(&self, filter: &MemberFilter) -> AppResult<Vec<MemberExportRow>> {
        let sql = format!(
            "SELECT m.id, m.name, m.surname, m.second_surname, m.email, m.birthdate, m.phone,
                    g.name AS gender, c.name AS country, m.notes, m.emails_disabled,
                    COALESCE((SELECT array_agg(i.name ORDER BY i.id)
                              FROM member_interests mi JOIN interests i ON i.id = mi.interest_id
                              WHERE mi.member_id = m.id), '{{}}') AS interests
             FROM members m
             LEFT JOIN genders g ON g.id = m.gender_id
             LEFT JOIN countries c ON c.id = m.country_id
             WHERE {FILTER}
             ORDER BY m.surname, m.name, m.id"
        );
        let rows = sqlx::query_as::<_, MemberExportDb>(AssertSqlSafe(sql))
            .bind(&filter.search)
            .bind(like_pattern(&filter.search))
            .bind(&filter.interest_ids)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .into_iter()
            .map(|r| MemberExportRow {
                id: r.id,
                name: r.name,
                surname: r.surname,
                second_surname: r.second_surname,
                email: r.email,
                birthdate: r.birthdate,
                phone: r.phone,
                gender: r.gender,
                country: r.country,
                interests: r.interests,
                notes: r.notes,
                emails_disabled: r.emails_disabled,
            })
            .collect())
    }

    async fn members_with_birthday(&self, day: u32, month: u32) -> AppResult<Vec<Member>> {
        let sql = format!(
            "SELECT {COLUMNS} FROM members m
             WHERE m.birthdate IS NOT NULL
               AND EXTRACT(DAY FROM m.birthdate) = $1
               AND EXTRACT(MONTH FROM m.birthdate) = $2
             ORDER BY m.name"
        );
        let rows = sqlx::query_as::<_, MemberDb>(AssertSqlSafe(sql))
            .bind(day as i32)
            .bind(month as i32)
            .fetch_all(&self.pool)
            .await?;
        self.with_interests(rows).await
    }

    async fn create_member(&self, d: &MemberData) -> AppResult<Uuid> {
        let mut tx = self.pool.begin().await?;
        let id: Uuid = sqlx::query_scalar(
            "INSERT INTO members (name, surname, second_surname, email, birthdate, phone, notes,
                                  emails_disabled, gender_id, country_id, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                     (now() AT TIME ZONE 'UTC'), (now() AT TIME ZONE 'UTC'))
             RETURNING id",
        )
        .bind(&d.name)
        .bind(&d.surname)
        .bind(&d.second_surname)
        .bind(&d.email)
        .bind(d.birthdate)
        .bind(&d.phone)
        .bind(&d.notes)
        .bind(d.emails_disabled)
        .bind(d.gender_id)
        .bind(d.country_id)
        .fetch_one(&mut *tx)
        .await?;
        link_interests(&mut *tx, id, &d.interest_ids).await?;
        tx.commit().await?;
        Ok(id)
    }

    async fn update_member(&self, id: Uuid, d: &MemberData) -> AppResult<bool> {
        let mut tx = self.pool.begin().await?;
        let updated = sqlx::query(
            "UPDATE members SET name = $2, surname = $3, second_surname = $4, email = $5,
                    birthdate = $6, phone = $7, notes = $8, gender_id = $9, country_id = $10,
                    emails_disabled = $11,
                    updated_at = (now() AT TIME ZONE 'UTC')
             WHERE id = $1",
        )
        .bind(id)
        .bind(&d.name)
        .bind(&d.surname)
        .bind(&d.second_surname)
        .bind(&d.email)
        .bind(d.birthdate)
        .bind(&d.phone)
        .bind(&d.notes)
        .bind(d.gender_id)
        .bind(d.country_id)
        .bind(d.emails_disabled)
        .execute(&mut *tx)
        .await?
        .rows_affected();
        if updated == 0 {
            return Ok(false);
        }
        sqlx::query("DELETE FROM member_interests WHERE member_id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        link_interests(&mut *tx, id, &d.interest_ids).await?;
        tx.commit().await?;
        Ok(true)
    }

    async fn delete_member(&self, id: Uuid) -> AppResult<bool> {
        let result = sqlx::query("DELETE FROM members WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() == 1)
    }

    async fn member_email(&self, id: Uuid) -> AppResult<Option<String>> {
        Ok(
            sqlx::query_scalar("SELECT email FROM members WHERE id = $1")
                .bind(id)
                .fetch_optional(&self.pool)
                .await?,
        )
    }

    async fn recipient_emails(&self, any_of_interests: Option<&[i32]>) -> AppResult<Vec<String>> {
        let rows =
            match any_of_interests {
                None => sqlx::query_scalar(
                    "SELECT DISTINCT email FROM members WHERE btrim(email) <> '' AND NOT m.emails_disabled ORDER BY email",
                )
                .fetch_all(&self.pool)
                .await?,
                Some(ids) => {
                    sqlx::query_scalar(
                        "SELECT DISTINCT m.email FROM members m
                     WHERE btrim(m.email) <> ''
                       AND EXISTS (SELECT 1 FROM member_interests mi
                                   WHERE mi.member_id = m.id AND mi.interest_id = ANY($1))
                       AND NOT m.emails_disabled
                     ORDER BY m.email",
                    )
                    .bind(ids)
                    .fetch_all(&self.pool)
                    .await?
                }
            };
        Ok(rows)
    }
}
