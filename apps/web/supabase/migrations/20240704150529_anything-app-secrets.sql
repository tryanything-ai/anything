
CREATE TABLE IF NOT EXISTS anything.secrets
(
    secret_id uuid unique NOT NULL DEFAULT uuid_generate_v4() primary key,
    -- If your model is owned by an account, you want to make sure you have an account_id column
    -- referencing the account table. Make sure you also set permissions appropriately
    account_id uuid not null references basejump.accounts(id),

    -- ADD YOUR COLUMNS HERE
    secret_name text not null,
    vault_secret_id uuid not null, -- this is how we fetch from encrypted storage
    secret_description text,
    archived boolean not null default false,

    -- timestamps are useful for auditing
    -- Basejump has some convenience functions defined below for automatically handling these
    updated_at timestamp with time zone,
    created_at timestamp with time zone,
    -- Useful for tracking who made changes to a record
    -- Basejump has some convenience functions defined below for automatically handling these
    updated_by uuid references auth.users(id),
    created_by uuid references auth.users(id)
);

-- protect the timestamps by setting created_at and updated_at to be read-only and managed by a trigger
CREATE TRIGGER set_secret_timestamp
    BEFORE INSERT OR UPDATE ON anything.secrets
    FOR EACH ROW
EXECUTE PROCEDURE basejump.trigger_set_timestamps();

-- protect the updated_by and created_by columns by setting them to be read-only and managed by a trigger
CREATE TRIGGER set_secret_user_tracking
    BEFORE INSERT OR UPDATE ON anything.secrets
    FOR EACH ROW
EXECUTE PROCEDURE basejump.trigger_set_user_tracking();

-- enable RLS on the table
ALTER TABLE anything.secrets ENABLE ROW LEVEL SECURITY;


-- Because RLS is enabled, this table will NOT be accessible to any users by default
-- You must create a policy for each user that should have access to the table
-- Here are a few example policies that you may find useful when working with Basejump

----------------
-- Authenticated users should be able to read all records regardless of account
----------------
-- create policy "All logged in users can select" on anything.secrets
--     for select
--     to authenticated
--     using (true);

----------------
-- Authenticated AND Anon users should be able to read all records regardless of account
----------------
-- create policy "All authenticated and anonymous users can select" on anything.secrets
--     for select
--     to authenticated, anon
--     using (true);

-------------
-- Users should be able to read records that are owned by an account they belong to
--------------
create policy "Account members can select" on anything.secrets
    for select
    to authenticated
    using (
    (account_id IN ( SELECT basejump.get_accounts_with_role()))
    );


----------------
-- Users should be able to create records that are owned by an account they belong to
----------------
create policy "Account members can insert" on anything.secrets
    for insert
    to authenticated
    with check (
    (account_id IN ( SELECT basejump.get_accounts_with_role()))
    );

---------------
-- Users should be able to update records that are owned by an account they belong to
---------------
create policy "Account members can update" on anything.secrets
    for update
    to authenticated
    using (
    (account_id IN ( SELECT basejump.get_accounts_with_role()))
    );

----------------
-- Users should be able to delete records that are owned by an account they belong to
----------------
create policy "Account members can delete" on anything.secrets
    for delete
    to authenticated
    using (
    (account_id IN ( SELECT basejump.get_accounts_with_role()))
    );

----------------
-- Only account OWNERS should be able to delete records that are owned by an account they belong to
----------------
-- create policy "Account owners can delete" on anything.secrets
--     for delete
--     to authenticated
--     using (
--     (account_id IN ( SELECT basejump.get_accounts_with_role("owner")))
--      );

-- //https://supabase.com/docs/guides/api/securing-your-api
-- Create Functions for Managing Secrets
create or replace function anything.insert_secret(name text, secret text)
returns uuid
language plpgsql
security invoker
as $$
begin
  if current_setting('role') != 'service_role' then
    raise exception 'authentication required';
  end if;
 
  return vault.create_secret(secret, name);
end;
$$;

create function anything.read_secret(secret_name text)
returns text
language plpgsql
security invoker
as $$
declare
  secret text;
begin
  if current_setting('role') != 'service_role' then
    raise exception 'authentication required';
  end if;
 
  select decrypted_secret from vault.decrypted_secrets where name =
  secret_name into secret;
  return secret;
end;
$$;

CREATE OR REPLACE FUNCTION anything.get_decrypted_secrets(account_id uuid)
RETURNS TABLE (
    secret_id uuid,
    secret_name text,
    secret_value text,
    secret_description text
) 
LANGUAGE plpgsql
SECURITY INVOKER
AS $$
BEGIN
    IF current_setting('role', true) IS DISTINCT FROM 'service_role' THEN
        RAISE EXCEPTION 'authentication required';
    END IF;

    RETURN QUERY
    SELECT 
        s.secret_id,
        s.secret_name,
        vs.decrypted_secret AS secret_value,
        s.secret_description
    FROM 
        anything.secrets s
    JOIN 
        vault.decrypted_secrets vs
    ON 
        s.vault_secret_id = vs.id
    WHERE
        s.account_id = get_decrypted_secrets.account_id; -- Use the function's parameter explicitly
END;
$$;
