CREATE TABLE emails (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    subject    VARCHAR(255) NOT NULL,
    body       TEXT NOT NULL,
    provider   VARCHAR(50) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE email_recipients (
    email_id UUID    NOT NULL REFERENCES emails(id) ON DELETE CASCADE,
    email    VARCHAR(255) NOT NULL
);

CREATE INDEX idx_email_recipients_email_id ON email_recipients(email_id);