-- Add migration script here
-- File name: 20231227120000_add_role_to_users.sql
-- Create the enum type 'enum_role'
CREATE TYPE enum_role AS ENUM ('admin', 'user');

-- Add a new field 'role' to the 'users' table
ALTER TABLE
  "users"
ADD
  COLUMN role enum_role NOT NULL DEFAULT 'user';

-- Update the 'role' column with the default value 'user' for existing rows
UPDATE
  "users"
SET
  role = 'user';

-- Add a check constraint to enforce the enum values for the 'role' column
ALTER TABLE
  "users"
ADD
  CONSTRAINT chk_valid_role CHECK (role IN ('admin', 'user'));