-- Add migration script here
CREATE TABLE IF NOT EXISTS "users" (
  id UUID PRIMARY KEY,
  email VARCHAR(100) NOT NULL UNIQUE,
  password VARCHAR(100) NOT NULL,
  pseudo VARCHAR(30) NULL UNIQUE
);

CREATE INDEX idx_user_email ON users(email);
CREATE INDEX idx_user_pseudo ON users(pseudo);
