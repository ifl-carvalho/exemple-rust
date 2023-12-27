-- Add migration script here
-- Drop the 'role' column from the 'users' table
ALTER TABLE
  "users" DROP COLUMN IF EXISTS role;

-- Drop the enum type 'enum_role'
DROP TYPE IF EXISTS enum_role;