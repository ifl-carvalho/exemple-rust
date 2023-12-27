-- Add migration script here
create extension if not exists "uuid-ossp";

create or replace function set_updated_at()
    returns trigger as
$$
begin
    NEW.updated_at = now();
    return NEW;
end;
$$ language plpgsql;

create or replace function trigger_updated_at(tablename regclass)
    returns void as
$$
begin
    execute format('CREATE TRIGGER set_updated_at
        BEFORE UPDATE
        ON %s
        FOR EACH ROW
        WHEN (OLD is distinct from NEW)
    EXECUTE FUNCTION set_updated_at();', tablename);
end;
$$ language plpgsql;

-- Check if the collation "case_insensitive" already exists before creating it
DO $$ 
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_collation WHERE collname = 'case_insensitive') THEN
        CREATE COLLATION case_insensitive (provider = icu, locale = 'und-u-ks-level2', deterministic = false);
    END IF;
END $$;