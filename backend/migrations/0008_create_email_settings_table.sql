CREATE TABLE email_settings (
    id INT PRIMARY KEY,
    active_provider VARCHAR(50) NOT NULL
);

INSERT INTO email_settings (id, active_provider) VALUES (1, 'RESEND');