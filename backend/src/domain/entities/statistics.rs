#[derive(Debug, Clone)]
pub struct InterestPopularity {
    pub interest_name: String,
    pub member_count: i64,
}

#[derive(Debug, Clone, Copy)]
pub struct EmailPreferenceStats {
    pub enabled: i64,
    pub disabled: i64,
}

impl EmailPreferenceStats {
    pub fn total(&self) -> i64 {
        self.enabled + self.disabled
    }
}
