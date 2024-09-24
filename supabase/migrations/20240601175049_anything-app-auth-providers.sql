
CREATE TABLE IF NOT EXISTS anything.auth_providers
(   
    auth_provider_id TEXT NOT NULL PRIMARY KEY, 

    provider_name TEXT NOT NULL, -- name of the provider used in actions
    provider_label TEXT NOT NULL, -- label for the provider for UI
    provider_icon TEXT NOT NULL, -- icon for the provider
    provider_description TEXT NOT NULL, -- description of the provider for users
    provider_readme TEXT NOT NULL, -- internal notes on managing this connection
    auth_type TEXT NOT NULL DEFAULT 'oauth2',
    auth_url TEXT NOT NULL,
    token_url TEXT NOT NULL,
    provider_data jsonb,
    access_token_lifetime_seconds TEXT,
    refresh_token_lifetime_seconds TEXT,
    redirect_url TEXT NOT NULL,
    client_id_vault_id uuid, -- this is how we fetch from encrypted storage
    client_secret_vault_id uuid, -- this is how we fetch from encrypted storage
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

-- Grant all privileges to the service role
-- GRANT ALL ON TABLE anything.auth_providers TO service_role;

-- -- Revoke all privileges from other roles
-- REVOKE ALL ON TABLE anything.auth_providers FROM PUBLIC, authenticated, anon;

-- -- Create a policy that allows only the service role to access the table
-- CREATE POLICY "Service role can access auth_providers" ON anything.auth_providers
--     FOR ALL
--     TO service_role
--     USING (true);

-- Function to get decrypted auth providers
CREATE OR REPLACE FUNCTION anything.get_decrypted_auth_providers()
RETURNS TABLE (
    auth_provider_id TEXT,
    provider_name TEXT,
    provider_label TEXT,
    provider_icon TEXT,
    provider_description TEXT,
    provider_readme TEXT,
    auth_type TEXT,
    auth_url TEXT,
    token_url TEXT,
    provider_data JSONB,
    access_token_lifetime_seconds TEXT,
    refresh_token_lifetime_seconds TEXT,
    redirect_url TEXT,
    client_id TEXT,
    client_secret TEXT,
    scopes TEXT,
    public BOOLEAN,
    updated_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ,
    updated_by UUID,
    created_by UUID
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
        ap.auth_provider_id,
        ap.provider_name,
        ap.provider_label,
        ap.provider_icon,
        ap.provider_description,
        ap.provider_readme,
        ap.auth_type,
        ap.auth_url,
        ap.token_url,
        ap.provider_data,
        ap.access_token_lifetime_seconds,
        ap.refresh_token_lifetime_seconds,
        ap.redirect_url,
        (SELECT decrypted_secret FROM vault.decrypted_secrets WHERE id = ap.client_id_vault_id) AS client_id,
        (SELECT decrypted_secret FROM vault.decrypted_secrets WHERE id = ap.client_secret_vault_id) AS client_secret,
        ap.scopes,
        ap.public,
        ap.updated_at,
        ap.created_at,
        ap.updated_by,
        ap.created_by
    FROM 
        anything.auth_providers ap;
END;
$$;

-- Function to get a decrypted auth provider by name
CREATE OR REPLACE FUNCTION anything.get_decrypted_auth_provider_by_name(provider_name_param TEXT)
RETURNS TABLE (
    auth_provider_id TEXT,
    provider_name TEXT,
    provider_label TEXT,
    provider_icon TEXT,
    provider_description TEXT,
    provider_readme TEXT,
    auth_type TEXT,
    auth_url TEXT,
    token_url TEXT,
    provider_data JSONB,
    access_token_lifetime_seconds TEXT,
    refresh_token_lifetime_seconds TEXT,
    redirect_url TEXT,
    client_id TEXT,
    client_secret TEXT,
    client_id_vault_id UUID,
    client_secret_vault_id UUID,
    scopes TEXT,
    public BOOLEAN,
    updated_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ,
    updated_by UUID,
    created_by UUID
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
        ap.auth_provider_id,
        ap.provider_name,
        ap.provider_label,
        ap.provider_icon,
        ap.provider_description,
        ap.provider_readme,
        ap.auth_type,
        ap.auth_url,
        ap.token_url,
        ap.provider_data,
        ap.access_token_lifetime_seconds,
        ap.refresh_token_lifetime_seconds,
        ap.redirect_url,
        (SELECT decrypted_secret FROM vault.decrypted_secrets WHERE id = ap.client_id_vault_id) AS client_id,
        (SELECT decrypted_secret FROM vault.decrypted_secrets WHERE id = ap.client_secret_vault_id) AS client_secret,
        ap.client_id_vault_id,
        ap.client_secret_vault_id,
        ap.scopes,
        ap.public,
        ap.updated_at,
        ap.created_at,
        ap.updated_by,
        ap.created_by
    FROM 
        anything.auth_providers ap
    WHERE
        ap.provider_name = provider_name_param;
END;
$$;
