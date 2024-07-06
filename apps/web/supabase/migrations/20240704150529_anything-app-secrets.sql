
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

    -- timestamps are useful for auditing
    -- Basejump has some convenience functions defined below for automatically handling these
    updated_at timestamp with time zone,
    created_at timestamp with time zone,
    -- Useful for tracking who made changes to a record
    -- Basejump has some convenience functions defined below for automatically handling these
    updated_by uuid references auth.users(id),
    created_by uuid references auth.users(id),

    -- Ensure secret_name is unique per account_id //TODO: this may be wrong with like how ords work. Technically we need an id unique per org
    CONSTRAINT unique_secret_name_per_account UNIQUE (account_id, secret_name)
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




-- FUNCTIONS FOR MANAGING SECRET VAULT
-- //https://supabase.com/docs/guides/api/securing-your-api
-- Create Functions for Managing Secrets
create or replace function anything.insert_secret(name text, secret text, description text)
returns uuid
language plpgsql
security invoker
as $$
begin
  if current_setting('role') != 'service_role' then
    raise exception 'authentication required';
  end if;
 
  return vault.create_secret(secret, name, description);
end;
$$;




create function anything.update_secret(id uuid, secret text, name text, description text)
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
 
  return vault.update_secret(id, secret, name, description);
end;
$$;

CREATE OR REPLACE FUNCTION anything.delete_secret(secret_id UUID)
RETURNS UUID
LANGUAGE plpgsql
SECURITY INVOKER
AS $$
DECLARE
  deleted_secret_id UUID;
BEGIN
  IF current_setting('role') != 'service_role' THEN
    RAISE EXCEPTION 'authentication required';
  END IF;

  DELETE FROM vault.decrypted_secrets
  WHERE id = secret_id
  RETURNING id INTO deleted_secret_id;

  RETURN deleted_secret_id;
END;
$$;
-- create function delete_secret(secret_name text)
-- returns text
-- language plpgsql
-- security definer set search_path = public
-- as $$
-- begin
--   if current_setting('role') != 'service_role' then
--     raise exception 'authentication required';
--   end if;
 
--   return delete from vault.decrypted_secrets where name = secret_name;
-- end;
-- $$;

-- CREATE OR REPLACE FUNCTION anything.delete_secret(p_secret_id UUID, p_user_account_id UUID)
-- RETURNS TEXT
-- LANGUAGE plpgsql
-- SECURITY INVOKER
-- AS $$
-- DECLARE
--   v_secret_account_id UUID;
-- BEGIN
--   -- Check if the current setting role is 'service_role'
--   IF current_setting('role') != 'service_role' THEN
--     RAISE EXCEPTION 'Authentication required';
--   END IF;

--   -- Check if the secret exists
--   IF NOT EXISTS (SELECT 1 FROM vault.decrypted_secrets WHERE id = p_secret_id) THEN
--     RAISE EXCEPTION 'Secret not found';
--   END IF;

--   -- Retrieve the account_id of the secret
--   SELECT s.account_id INTO v_secret_account_id
--   FROM anything.secrets s
--   WHERE s.secret_id = p_secret_id;

--   -- Check if the user is part of the organization associated with the secret
--   IF NOT EXISTS (
--     SELECT 1
--     FROM basejump.account_user au
--     WHERE au.user_id = p_user_account_id
--     AND au.account_id = v_secret_account_id
--   ) THEN
--     RAISE EXCEPTION 'User is not authorized to delete this secret';
--   END IF;

--   -- Delete the secret
--   DELETE FROM vault.decrypted_secrets WHERE id = p_secret_id;

--   RETURN 'Secret deleted successfully';
-- END;
-- $$;

-- CREATE FUNCTION anything.delete_secret(secret_id UUID, user_account_id UUID)
-- RETURNS TEXT
-- LANGUAGE plpgsql
-- SECURITY INVOKER
-- AS $$
-- BEGIN
--   -- Check if the current setting role is 'service_role'
--   IF current_setting('role') != 'service_role' THEN
--     RAISE EXCEPTION 'authentication required';
--   END IF;

--   -- Check if the user is part of the specified organization
--   IF NOT EXISTS (
--     SELECT 1
--     FROM basejump.account_user wu
--     WHERE wu.user_id = user_account_id
--   ) THEN
--     RAISE EXCEPTION 'user is not part of the specified organization';
--   END IF;

--   -- Check if the secret belongs to the user's organization
--   IF NOT EXISTS (
--     SELECT 1
--     FROM vault.decrypted_secrets ds
--     JOIN basejump.account_user wu ON ds.account_id = wu.account_id
--     WHERE ds.id = secret_id
--     AND wu.user_id = user_account_id
--   ) THEN
--     RAISE EXCEPTION 'user is not authorized to delete this secret';
--   END IF;

--   -- Delete the secret
--   DELETE FROM vault.decrypted_secrets WHERE id = secret_id;

--   RETURN 'Secret deleted successfully';
-- END;
-- $$;

--THis deletes but doesn't block otehrs from delting it seems
-- CREATE FUNCTION anything.delete_secret(secret_id UUID, user_account_id UUID)
-- RETURNS TEXT
-- LANGUAGE plpgsql
-- SECURITY INVOKER
-- AS $$
-- BEGIN
--   -- Check if the current setting role is 'service_role'
--   IF current_setting('role') != 'service_role' THEN
--     RAISE EXCEPTION 'authentication required';
--   END IF;

--   -- Check if the user is part of the specified organization
--   IF NOT EXISTS (
--     select 1
--     from basejump.account_user wu
--     where wu.user_id = user_account_id
--   ) THEN
--     RAISE EXCEPTION 'user is not part of the specified organization';
--   END IF;

--   -- Delete the secret
--   DELETE FROM vault.decrypted_secrets WHERE id = secret_id; 

--   RETURN 'Secret deleted successfully';
-- END;
-- $$;



-- CREATE OR REPLACE FUNCTION anything.delete_secret(user_account_id uuid, secret_id uuid)
-- RETURNS text
-- LANGUAGE plpgsql
-- SECURITY INVOKER
-- AS $$
-- DECLARE
--     secret_account_id uuid;
-- BEGIN
--     IF current_setting('role') != 'service_role' THEN
--         RAISE EXCEPTION 'authentication required';
--     END IF;

--     -- Check if the secret belongs to an account the user has access to
--     SELECT s.account_id
--     INTO secret_account_id
--     FROM anything.secrets s
--     WHERE s.secret_id = delete_secret.secret_id;

--     IF secret_account_id IS NULL THEN
--         RAISE EXCEPTION 'secret not found';
--     END IF;

--     -- Check if the user has access to this account
--     IF NOT EXISTS (
--         SELECT 1
--         FROM basejump.account_user wu
--         WHERE wu.user_id = user_account_id AND wu.account_id = secret_account_id
--     ) THEN
--         RAISE EXCEPTION 'permission denied';
--     END IF;

--     -- Delete the secret from vault.decrypted_secrets
--     DELETE FROM vault.decrypted_secrets
--     WHERE id = delete_secret.secret_id;

--     -- Optionally, delete the secret from anything.secrets
--     DELETE FROM anything.secrets
--     WHERE secret_id = delete_secret.secret_id;

--     RETURN 'secret deleted';
-- END;
-- $$;


-- CREATE OR REPLACE FUNCTION anything.delete_secret(user_account_id uuid, secret_id uuid)
-- RETURNS text
-- LANGUAGE plpgsql
-- SECURITY INVOKER
-- AS $$
-- DECLARE
--     secret_account_id uuid;
-- BEGIN
--     IF current_setting('role') != 'service_role' THEN
--         RAISE EXCEPTION 'authentication required';
--     END IF;

--     -- Check if the secret belongs to an account the user has access to
--     SELECT s.account_id
--     INTO secret_account_id
--     FROM anything.secrets s
--     WHERE s.secret_id = secret_id;

--     IF secret_account_id IS NULL THEN
--         RAISE EXCEPTION 'secret not found';
--     END IF;

--     -- Check if the user has access to this account
--     IF NOT EXISTS (
--         SELECT 1
--         FROM basejump.account_user wu
--         WHERE wu.user_id = user_account_id AND wu.account_id = secret_account_id
--     ) THEN
--         RAISE EXCEPTION 'permission denied';
--     END IF;

--     -- Delete the secret from vault.decrypted_secrets
--     DELETE FROM vault.decrypted_secrets
--     WHERE id = secret_id;

--     -- Optionally, delete the secret from anything.secrets
--     DELETE FROM anything.secrets
--     WHERE secret_id = secret_id;

--     RETURN 'secret deleted';
-- END;
-- $$;

-- create function anything.delete_secret(id uuid)
-- returns text
-- language plpgsql
-- security invoker
-- as $$
-- declare
--   secret text;
-- begin
--   if current_setting('role') != 'service_role' then
--     raise exception 'authentication required';
--   end if;

--   return delete from vault.decrypted_secrets where id = id;
-- end;
-- $$;

create function anything.read_secret(secret_id text)
returns text
language plpgsql
security invoker
as $$
begin
  if current_setting('role') != 'service_role' then
    raise exception 'authentication required';
  end if;
 
  select * from vault.decrypted_secrets where id =
  secret_id; 

end;
$$;

CREATE OR REPLACE FUNCTION anything.get_decrypted_secrets(user_account_id uuid)
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
        s.account_id IN (
            SELECT account_id
            FROM basejump.account_user wu
            WHERE wu.user_id = user_account_id
        );
END;
$$;