ALTER TABLE anything.secrets
ADD COLUMN anything_api_key boolean NOT NULL DEFAULT false;

-- Updating this function from _anyting-app-secrets.sql to not grab api keys also
-- this makes it reverse compatible with the previous function even though  nwo we hold api keys in this talbe too
CREATE OR REPLACE FUNCTION anything.get_decrypted_secrets(team_account_id uuid)
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
        s.account_id = team_account_id
        AND s.anything_api_key = false;
END;
$$;


CREATE OR REPLACE FUNCTION anything.get_decrypted_anything_api_keys(team_account_id uuid)
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
        s.account_id = team_account_id
        AND s.anything_api_key = true;
END;
$$;

-- For getting the user account when an API key is sent in the request
CREATE OR REPLACE FUNCTION anything.get_secret_by_secret_value(secret_value text)
RETURNS TABLE (
    secret_id uuid,
    account_id uuid,
    secret_name text,
    vault_secret_id uuid,
    secret_description text,
    anything_api_key boolean,
    updated_at timestamptz,
    created_at timestamptz,
    updated_by uuid,
    created_by uuid
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
        s.account_id,
        s.secret_name,
        s.vault_secret_id,
        s.secret_description,
        s.anything_api_key,
        s.updated_at,
        s.created_at,
        s.updated_by,
        s.created_by
    FROM 
        anything.secrets s
    JOIN 
        vault.decrypted_secrets vs
    ON 
        s.vault_secret_id = vs.id
    WHERE
        vs.decrypted_secret = secret_value;
END;
$$;

