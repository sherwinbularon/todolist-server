-- Add migration script here
-- Add your SQL migration here
-- See https://docs.rs/sqlx/latest/sqlx/macro.migrate.html

-- Up
CREATE TABLE tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    completed BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Down
DROP TABLE tasks;