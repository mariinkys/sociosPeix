use std::sync::Arc;

use async_trait::async_trait;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    app_error::{AppError, AppResult},
    clock::Clock,
    domain::pagination::{Page, PageRequest, SortDirection},
    entities::{
        member::{Member, MemberData, MemberExportRow, MemberFilter, MemberSortField},
        statistics::InterestPopularity,
    },
    use_cases::catalog::CatalogUseCases,
};

#[async_trait]
pub trait MemberPersistence: Send + Sync {
    async fn find_member(&self, id: Uuid) -> AppResult<Option<Member>>;
    async fn list_members(
        &self,
        filter: &MemberFilter,
        sort: (MemberSortField, SortDirection),
        page: PageRequest,
    ) -> AppResult<Page<Member>>;
    async fn export_members(&self, filter: &MemberFilter) -> AppResult<Vec<MemberExportRow>>;
    async fn members_with_birthday(&self, day: u32, month: u32) -> AppResult<Vec<Member>>;
    /// Inserts the member and links its interests (unknown interest ids are ignored).
    async fn create_member(&self, data: &MemberData) -> AppResult<Uuid>;
    /// Returns `false` if the member doesn't exist.
    async fn update_member(&self, id: Uuid, data: &MemberData) -> AppResult<bool>;
    /// Returns `false` if the member doesn't exist.
    async fn delete_member(&self, id: Uuid) -> AppResult<bool>;
    /// The member's email address, if the member exists.
    async fn member_email(&self, id: Uuid) -> AppResult<Option<String>>;
    /// Distinct, non-blank email addresses of all members, or only of the
    /// members having *any* of the given interests. It excludes opted-out member s via the emails_disabled flag.
    async fn recipient_emails(&self, any_of_interests: Option<&[i32]>) -> AppResult<Vec<String>>;
}

/// Turns rows into a spreadsheet file (xlsx bytes). CPU-bound.
pub trait SpreadsheetExporter: Send + Sync {
    fn members(&self, rows: &[MemberExportRow]) -> AppResult<Vec<u8>>;
    fn interest_popularity(&self, rows: &[InterestPopularity]) -> AppResult<Vec<u8>>;
}

pub struct MemberUseCases {
    members: Arc<dyn MemberPersistence>,
    catalog: Arc<CatalogUseCases>,
    exporter: Arc<dyn SpreadsheetExporter>,
    clock: Clock,
}

fn not_found(id: Uuid) -> AppError {
    AppError::NotFound(format!("Member not found with id: {id}"))
}

impl MemberUseCases {
    pub fn new(
        members: Arc<dyn MemberPersistence>,
        catalog: Arc<CatalogUseCases>,
        exporter: Arc<dyn SpreadsheetExporter>,
        clock: Clock,
    ) -> Self {
        Self {
            members,
            catalog,
            exporter,
            clock,
        }
    }

    #[instrument(skip(self, data), fields(email = %data.email))]
    pub async fn create(&self, data: MemberData) -> AppResult<Member> {
        self.check_references(&data).await?;
        let id = self.members.create_member(&data).await?;
        self.get(id).await
    }

    pub async fn get(&self, id: Uuid) -> AppResult<Member> {
        self.members
            .find_member(id)
            .await?
            .ok_or_else(|| not_found(id))
    }

    pub async fn list(
        &self,
        filter: &MemberFilter,
        sort: (MemberSortField, SortDirection),
        page: PageRequest,
    ) -> AppResult<Page<Member>> {
        self.members.list_members(filter, sort, page).await
    }

    pub async fn todays_birthdays(&self) -> AppResult<Vec<Member>> {
        let (day, month) = self.clock.today_day_month();
        self.members.members_with_birthday(day, month).await
    }

    #[instrument(skip(self, data))]
    pub async fn update(&self, id: Uuid, data: MemberData) -> AppResult<Member> {
        self.check_references(&data).await?;
        if !self.members.update_member(id, &data).await? {
            return Err(not_found(id));
        }
        self.get(id).await
    }

    #[instrument(skip(self))]
    pub async fn delete(&self, id: Uuid) -> AppResult<()> {
        if !self.members.delete_member(id).await? {
            return Err(not_found(id));
        }
        Ok(())
    }

    pub async fn export(&self, filter: &MemberFilter) -> AppResult<Vec<u8>> {
        let rows = self.members.export_members(filter).await?;
        let exporter = self.exporter.clone();
        tokio::task::spawn_blocking(move || exporter.members(&rows))
            .await
            .map_err(AppError::internal)?
    }

    /// Gender and country are optional, but if given they must exist.
    async fn check_references(&self, data: &MemberData) -> AppResult<()> {
        if let Some(id) = data.gender_id
            && !self.catalog.gender_exists(id).await?
        {
            return Err(AppError::Validation(format!(
                "Gender not found with id: {id}"
            )));
        }
        if let Some(id) = data.country_id
            && !self.catalog.country_exists(id).await?
        {
            return Err(AppError::Validation(format!(
                "Country not found with id: {id}"
            )));
        }
        Ok(())
    }
}
