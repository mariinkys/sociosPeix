use async_trait::async_trait;

use crate::{
    adapters::persistence::PostgresPersistence, app_error::AppResult,
    entities::statistics::InterestPopularity, use_cases::statistics::StatisticsPersistence,
};

#[async_trait]
impl StatisticsPersistence for PostgresPersistence {
    async fn interest_popularity(&self) -> AppResult<Vec<InterestPopularity>> {
        let rows: Vec<(String, i64)> = sqlx::query_as(
            "SELECT i.name, COUNT(DISTINCT mi.member_id) AS member_count
             FROM interests i
             JOIN member_interests mi ON mi.interest_id = i.id
             GROUP BY i.id, i.name
             ORDER BY member_count DESC, i.name",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|(interest_name, member_count)| InterestPopularity {
                interest_name,
                member_count,
            })
            .collect())
    }
}
