
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