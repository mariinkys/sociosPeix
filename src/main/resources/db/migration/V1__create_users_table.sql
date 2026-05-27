CREATE TABLE users (
   id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
   name        VARCHAR(100) NOT NULL,
   email       VARCHAR(255) NOT NULL UNIQUE,
   password    VARCHAR(255) NOT NULL,
   role        VARCHAR(20) NOT NULL DEFAULT 'USER',
   created_at  TIMESTAMP NOT NULL DEFAULT now(),
   updated_at  TIMESTAMP NOT NULL DEFAULT now()
);

INSERT INTO users (
    name,
    email,
    password,
    role
) VALUES (
     'Admin',
     'admin@example.com',
     '$2y$10$Dm42cN7wQvU5L9fhOP4E5uOyogeKOvq4i7nuhhHcdPBW8ZD5Fd/0q', --admin
     'ADMIN'
 );