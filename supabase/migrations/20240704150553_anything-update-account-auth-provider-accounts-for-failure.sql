-- First, drop existing functions
DROP FUNCTION IF EXISTS anything.get_decrypted_account_and_provider(UUID);
DROP FUNCTION IF EXISTS anything.get_account_auth_provider_accounts(UUID);

ALTER TABLE anything.account_auth_provider_accounts
ADD COLUMN failed_at TIMESTAMP WITH TIME ZONE,
ADD COLUMN failed BOOLEAN NOT NULL DEFAULT false,
ADD COLUMN failed_reason TEXT,
ADD COLUMN failure_retries INTEGER NOT NULL DEFAULT 0,
ADD COLUMN last_failure_retry TIMESTAMP WITH TIME ZONE;

-- Update the get_decrypted_account_and_provider function to include new columns
CREATE OR REPLACE FUNCTION anything.get_decrypted_account_and_provider(p_account_id UUID)
RETURNS TABLE (
    account_auth_provider_account_id UUID,
    account_id UUID,
    auth_provider_id TEXT,
    account_auth_provider_account_label TEXT,
    account_auth_provider_account_slug TEXT,
    account_data JSONB,
    access_token TEXT,
    access_token_vault_id UUID,
    access_token_expires_at TIMESTAMPTZ,
    refresh_token TEXT,
    refresh_token_vault_id UUID,
    refresh_token_expires_at TIMESTAMPTZ,
    failed_at TIMESTAMPTZ,
    failed BOOLEAN,
    failed_reason TEXT,
    failure_retries INTEGER,
    last_failure_retry TIMESTAMPTZ,
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
        (SELECT decrypted_secret FROM vault.decrypted_secrets WHERE id = aapa.access_token_vault_id) AS access_token,
        aapa.access_token_vault_id,
        aapa.access_token_expires_at,
        (SELECT decrypted_secret FROM vault.decrypted_secrets WHERE id = aapa.refresh_token_vault_id) AS refresh_token,
        aapa.refresh_token_vault_id,
        aapa.refresh_token_expires_at,
        aapa.failed_at,
        aapa.failed,
        aapa.failed_reason,
        aapa.failure_retries,
        aapa.last_failure_retry,
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
            'client_id_vault_id', ap.client_id_vault_id,
            'client_secret_vault_id', ap.client_secret_vault_id,
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

-- Update the get_account_auth_provider_accounts function to include new columns
CREATE OR REPLACE FUNCTION anything.get_account_auth_provider_accounts(p_account_id UUID)
RETURNS TABLE (
    account_auth_provider_account_id UUID,
    account_id UUID,
    auth_provider_id TEXT,
    account_auth_provider_account_label TEXT,
    account_auth_provider_account_slug TEXT,
    account_data JSONB,
    access_token TEXT,
    access_token_vault_id UUID,
    access_token_expires_at TIMESTAMPTZ,
    refresh_token TEXT,
    refresh_token_vault_id UUID,
    refresh_token_expires_at TIMESTAMPTZ,
    failed_at TIMESTAMPTZ,
    failed BOOLEAN,
    failed_reason TEXT,
    failure_retries INTEGER,
    last_failure_retry TIMESTAMPTZ,
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
        (SELECT decrypted_secret FROM vault.decrypted_secrets WHERE id = aapa.access_token_vault_id) AS access_token,
        aapa.access_token_vault_id,
        aapa.access_token_expires_at,
        (SELECT decrypted_secret FROM vault.decrypted_secrets WHERE id = aapa.refresh_token_vault_id) AS refresh_token,
        aapa.refresh_token_vault_id,
        aapa.refresh_token_expires_at,
        aapa.failed_at,
        aapa.failed,
        aapa.failed_reason,
        aapa.failure_retries,
        aapa.last_failure_retry,
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
            'client_id_vault_id', ap.client_id_vault_id,
            'client_secret_vault_id', ap.client_secret_vault_id,
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
