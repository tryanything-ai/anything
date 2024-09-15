
CREATE TABLE IF NOT EXISTS anything.account_auth_provider_accounts
(
    account_auth_provider_account_id uuid unique NOT NULL DEFAULT uuid_generate_v4() primary key,
    -- If your model is owned by an account, you want to make sure you have an account_id column
    -- referencing the account table. Make sure you also set permissions appropriately
    account_id uuid not null references basejump.accounts(id),
    auth_provider_id TEXT NOT NULL references anything.auth_providers(auth_provider_id),
    -- ADD YOUR COLUMNS HERE
    -- flow_name TEXT NOT NULL,
    account_auth_provider_account_label TEXT NOT NULL, -- what users see
    account_auth_provider_account_slug TEXT NOT NULL, -- what bundler uses ( will standardize to airtable airtable_2, google google_2) etc
    account_data jsonb,
    access_token TEXT NOT NULL, -- figure out how to put these in the secrets table
    access_token_expires_at TIMESTAMP WITH TIME ZONE,
    refresh_token TEXT, -- figure out how to put these in secrets table
    refresh_token_expires_at TIMESTAMP WITH TIME ZONE,
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
CREATE TRIGGER set_account_auth_provider_accounts_timestamp
    BEFORE INSERT OR UPDATE ON anything.account_auth_provider_accounts
    FOR EACH ROW
EXECUTE PROCEDURE basejump.trigger_set_timestamps();

-- protect the updated_by and created_by columns by setting them to be read-only and managed by a trigger
CREATE TRIGGER set_account_auth_provider_accounts_user_tracking
    BEFORE INSERT OR UPDATE ON anything.account_auth_provider_accounts
    FOR EACH ROW
EXECUTE PROCEDURE basejump.trigger_set_user_tracking();


-- enable RLS on the table
ALTER TABLE anything.account_auth_provider_accounts ENABLE ROW LEVEL SECURITY;


-- Because RLS is enabled, this table will NOT be accessible to any users by default
-- You must create a policy for each user that should have access to the table
-- Here are a few example policies that you may find useful when working with Basejump

----------------
-- Authenticated users should be able to read all records regardless of account
----------------
-- create policy "All logged in users can select" on anything.account_auth_provider_accounts
--     for select
--     to authenticated
--     using (true);

----------------
-- Authenticated AND Anon users should be able to read all records regardless of account
----------------
-- create policy "All authenticated and anonymous users can select" on anything.account_auth_provider_accounts
--     for select
--     to authenticated, anon
--     using (true);

-------------
-- Users should be able to read records that are owned by an account they belong to
--------------
create policy "Account members can select" on anything.account_auth_provider_accounts
    for select
    to authenticated
    using (
    (account_id IN ( SELECT basejump.get_accounts_with_role()))
    );


----------------
-- Users should be able to create records that are owned by an account they belong to
----------------
create policy "Account members can insert" on anything.account_auth_provider_accounts
    for insert
    to authenticated
    with check (
    (account_id IN ( SELECT basejump.get_accounts_with_role()))
    );

---------------
-- Users should be able to update records that are owned by an account they belong to
---------------
create policy "Account members can update" on anything.account_auth_provider_accounts
    for update
    to authenticated
    using (
    (account_id IN ( SELECT basejump.get_accounts_with_role()))
    );

----------------
-- Users should be able to delete records that are owned by an account they belong to
----------------
-- create policy "Account members can delete" on anything.account_auth_provider_accounts
--     for delete
--     to authenticated
--     using (
--     (account_id IN ( SELECT basejump.get_accounts_with_role()))
--     );

----------------
-- Only account OWNERS should be able to delete records that are owned by an account they belong to
----------------
-- create policy "Account owners can delete" on anything.account_auth_provider_accounts
--     for delete
--     to authenticated
--     using (
--     (account_id IN ( SELECT basejump.get_accounts_with_role("owner")))
--      );

-- Function to get account auth provider accounts with decrypted auth provider details
CREATE OR REPLACE FUNCTION anything.get_account_auth_provider_accounts_with_decrypted_providers(p_account_id UUID)
RETURNS TABLE (
    account_auth_provider_account_id UUID,
    account_id UUID,
    auth_provider_id TEXT,
    account_auth_provider_account_label TEXT,
    account_auth_provider_account_slug TEXT,
    account_data JSONB,
    access_token TEXT,
    access_token_expires_at TIMESTAMPTZ,
    refresh_token TEXT,
    refresh_token_expires_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ,
    updated_by UUID,
    created_by UUID,
    auth_provider JSONB
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
        aapa.account_auth_provider_account_id,
        aapa.account_id,
        aapa.auth_provider_id,
        aapa.account_auth_provider_account_label,
        aapa.account_auth_provider_account_slug,
        aapa.account_data,
        aapa.access_token,
        aapa.access_token_expires_at,
        aapa.refresh_token,
        aapa.refresh_token_expires_at,
        aapa.updated_at,
        aapa.created_at,
        aapa.updated_by,
        aapa.created_by,
        jsonb_build_object(
            'auth_provider_id', ap.auth_provider_id,
            'provider_name', ap.provider_name,
            'provider_label', ap.provider_label,
            'provider_icon', ap.provider_icon,
            'provider_description', ap.provider_description,
            'provider_readme', ap.provider_readme,
            'auth_type', ap.auth_type,
            'auth_url', ap.auth_url,
            'token_url', ap.token_url,
            'provider_data', ap.provider_data,
            'access_token_lifetime_seconds', ap.access_token_lifetime_seconds,
            'refresh_token_lifetime_seconds', ap.refresh_token_lifetime_seconds,
            'redirect_url', ap.redirect_url,
            'client_id', (SELECT decrypted_secret FROM vault.decrypted_secrets WHERE id = ap.client_id_vault_id),
            'client_secret', (SELECT decrypted_secret FROM vault.decrypted_secrets WHERE id = ap.client_secret_vault_id),
            'scopes', ap.scopes,
            'public', ap.public
        ) AS auth_provider
    FROM 
        anything.account_auth_provider_accounts aapa
    JOIN 
        anything.auth_providers ap ON aapa.auth_provider_id = ap.auth_provider_id
    WHERE 
        aapa.account_id = p_account_id;
END;
$$;
