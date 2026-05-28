CREATE TABLE members (
     id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
     name           VARCHAR(100) NOT NULL,
     surname        VARCHAR(100) NOT NULL,
     second_surname VARCHAR(100) NOT NULL DEFAULT '',
     email          VARCHAR(255) NOT NULL,
     birthdate      DATE,
     phone          VARCHAR(30) NOT NULL DEFAULT '',
     notes          TEXT NOT NULL DEFAULT '',
     created_at     TIMESTAMP NOT NULL DEFAULT now(),
     updated_at     TIMESTAMP NOT NULL DEFAULT now()
);