
CREATE TABLE IF NOT EXISTS anything.auth_providers
(
    oauth_provider_id uuid unique NOT NULL DEFAULT uuid_generate_v4() primary key,
    -- If your model is owned by an account, you want to make sure you have an account_id column
    -- referencing the account table. Make sure you also set permissions appropriately
    -- account_id uuid not null references basejump.accounts(id),

    -- ADD YOUR COLUMNS HERE
    -- flow_name TEXT NOT NULL,
    provider_name TEXT NOT NULL, -- name of the provider used in actions
    provider_label TEXT NOT NULL, -- label for the provider for UI
    provider_icon TEXT NOT NULL, -- icon for the provider
    provider_description TEXT NOT NULL, -- description of the provider for users
    provider_readme TEXT NOT NULL, -- internal notes on managing this connection
    auth_type TEXT NOT NULL DEFAULT 'oauth2',
    auth_url TEXT NOT NULL,
    token_url TEXT NOT NULL,
    client_id TEXT NOT NULL,
    client_secret TEXT NOT NULL,
    scopes TEXT NOT NULL,
    public boolean not null default false, -- whether this provider is public just utility

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
CREATE TRIGGER set_auth_providers_timestamp
    BEFORE INSERT OR UPDATE ON anything.auth_providers
    FOR EACH ROW
EXECUTE PROCEDURE basejump.trigger_set_timestamps();

-- protect the updated_by and created_by columns by setting them to be read-only and managed by a trigger
CREATE TRIGGER set_auth_providers_user_tracking
    BEFORE INSERT OR UPDATE ON anything.auth_providers
    FOR EACH ROW
EXECUTE PROCEDURE basejump.trigger_set_user_tracking();


-- enable RLS on the table
ALTER TABLE anything.auth_providers ENABLE ROW LEVEL SECURITY;


-- Because RLS is enabled, this table will NOT be accessible to any users by default
-- You must create a policy for each user that should have access to the table
-- Here are a few example policies that you may find useful when working with Basejump

----------------
-- Authenticated users should be able to read all records regardless of account
----------------
-- create policy "All logged in users can select" on anything.auth_providers
--     for select
--     to authenticated
--     using (true);

----------------
-- Authenticated AND Anon users should be able to read all records regardless of account
----------------
-- create policy "All authenticated and anonymous users can select" on anything.auth_providers
--     for select
--     to authenticated, anon
--     using (true);

-------------
-- Users should be able to read records that are owned by an account they belong to
--------------
-- create policy "Account members can select" on anything.auth_providers
--     for select
--     to authenticated
--     using (
--     (account_id IN ( SELECT basejump.get_accounts_with_role()))
--     );


----------------
-- Users should be able to create records that are owned by an account they belong to
----------------
-- create policy "Account members can insert" on anything.auth_providers
--     for insert
--     to authenticated
--     with check (
--     (account_id IN ( SELECT basejump.get_accounts_with_role()))
--     );

---------------
-- Users should be able to update records that are owned by an account they belong to
---------------
-- create policy "Account members can update" on anything.auth_providers
--     for update
--     to authenticated
--     using (
--     (account_id IN ( SELECT basejump.get_accounts_with_role()))
--     );

----------------
-- Users should be able to delete records that are owned by an account they belong to
----------------
-- create policy "Account members can delete" on anything.auth_providers
--     for delete
--     to authenticated
--     using (
--     (account_id IN ( SELECT basejump.get_accounts_with_role()))
--     );

----------------
-- Only account OWNERS should be able to delete records that are owned by an account they belong to
----------------
-- create policy "Account owners can delete" on anything.auth_providers
--     for delete
--     to authenticated
--     using (
--     (account_id IN ( SELECT basejump.get_accounts_with_role("owner")))
--      );