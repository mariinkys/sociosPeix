-- Indexes backing the hot query paths of the Rust backend.
CREATE INDEX IF NOT EXISTS idx_emails_created_at          ON emails (created_at DESC);
CREATE INDEX IF NOT EXISTS idx_emails_provider_created_at ON emails (provider, created_at);
CREATE INDEX IF NOT EXISTS idx_email_recipients_email     ON email_recipients (email);
CREATE INDEX IF NOT EXISTS idx_member_interests_interest  ON member_interests (interest_id);
CREATE INDEX IF NOT EXISTS idx_refresh_tokens_expires_at  ON refresh_tokens (expires_at);
-- Already covered by the UNIQUE constraint on refresh_tokens.token.
DROP INDEX IF EXISTS idx_refresh_tokens_token;
