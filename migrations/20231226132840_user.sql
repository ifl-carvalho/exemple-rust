-- Add migration script here
CREATE TABLE "users" (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ
);

SELECT trigger_updated_at('"users"');