use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    app_error::{AppError, AppResult},
    entities::statistics::{EmailPreferenceStats, InterestPopularity},
    use_cases::member::SpreadsheetExporter,
};

#[async_trait]
pub trait StatisticsPersistence: Send + Sync {
    /// Members per interest, most popular first.
    async fn interest_popularity(&self) -> AppResult<Vec<InterestPopularity>>;
    /// Members with their active email information.
    async fn email_preference_stats(&self) -> AppResult<EmailPreferenceStats>;
}

pub struct StatisticsUseCases {
    persistence: Arc<dyn StatisticsPersistence>,
    exporter: Arc<dyn SpreadsheetExporter>,
}

impl StatisticsUseCases {
    pub fn new(
        persistence: Arc<dyn StatisticsPersistence>,
        exporter: Arc<dyn SpreadsheetExporter>,
    ) -> Self {
        Self {
            persistence,
            exporter,
        }
    }

    pub async fn interest_popularity(&self) -> AppResult<Vec<InterestPopularity>> {
        self.persistence.interest_popularity().await
    }

    pub async fn export_interest_popularity(&self) -> AppResult<Vec<u8>> {
        let rows = self.interest_popularity().await?;
        let exporter = self.exporter.clone();
        tokio::task::spawn_blocking(move || exporter.interest_popularity(&rows))
            .await
            .map_err(AppError::internal)?
    }

    pub async fn email_preference_stats(&self) -> AppResult<EmailPreferenceStats> {
        self.persistence.email_preference_stats().await
    }
}
