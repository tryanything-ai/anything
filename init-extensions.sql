-- Initialize PostgreSQL extensions and auth schema for Anything
-- This replaces the Supabase-specific functionality with standard PostgreSQL

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgsodium";

-- Create auth schema (simplified version of Supabase auth)
CREATE SCHEMA IF NOT EXISTS auth;

-- Create auth.users table (simplified version)
CREATE TABLE IF NOT EXISTS auth.users (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    email text UNIQUE,
    encrypted_password text,
    email_confirmed_at timestamp with time zone,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    raw_app_meta_data jsonb,
    raw_user_meta_data jsonb,
    is_super_admin boolean DEFAULT false,
    confirmed_at timestamp with time zone,
    last_sign_in_at timestamp with time zone,
    phone text,
    phone_confirmed_at timestamp with time zone,
    confirmation_sent_at timestamp with time zone,
    email_change_confirm_status smallint DEFAULT 0,
    banned_until timestamp with time zone,
    reauthentication_sent_at timestamp with time zone,
    recovery_sent_at timestamp with time zone
);

-- Create auth.identities table (simplified version)
CREATE TABLE IF NOT EXISTS auth.identities (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id uuid REFERENCES auth.users(id) ON DELETE CASCADE,
    identity_data jsonb,
    provider text,
    last_sign_in_at timestamp with time zone,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    UNIQUE(provider, identity_data->>'sub')
);

-- Create auth.sessions table (simplified version)
CREATE TABLE IF NOT EXISTS auth.sessions (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id uuid REFERENCES auth.users(id) ON DELETE CASCADE,
    access_token text,
    refresh_token text,
    expires_at timestamp with time zone,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);

-- Create auth.uid() function (simplified version)
CREATE OR REPLACE FUNCTION auth.uid()
RETURNS uuid
LANGUAGE sql
STABLE
AS $$
  -- This is a simplified version - in a real implementation, you'd get this from the current session
  -- For now, we'll return a default user ID or you can modify this based on your authentication system
  SELECT '00000000-0000-0000-0000-000000000000'::uuid;
$$;

-- Grant necessary permissions
GRANT USAGE ON SCHEMA auth TO postgres, anon, authenticated, service_role;
GRANT ALL ON ALL TABLES IN SCHEMA auth TO postgres, anon, authenticated, service_role;
GRANT ALL ON ALL SEQUENCES IN SCHEMA auth TO postgres, anon, authenticated, service_role;
GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA auth TO postgres, anon, authenticated, service_role;

-- Create roles if they don't exist
DO $$
BEGIN
    IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'anon') THEN
        CREATE ROLE anon;
    END IF;
    IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'authenticated') THEN
        CREATE ROLE authenticated;
    END IF;
    IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'service_role') THEN
        CREATE ROLE service_role;
    END IF;
END
$$;

-- Grant role permissions
GRANT anon TO postgres;
GRANT authenticated TO postgres;
GRANT service_role TO postgres;

-- pgsodium extension is now available with the real implementation

-- Create pgsodium schema and key table
CREATE SCHEMA IF NOT EXISTS pgsodium;

CREATE TABLE IF NOT EXISTS pgsodium.key (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name text UNIQUE NOT NULL,
    status text DEFAULT 'valid',
    key_type text NOT NULL,
    key_id bigint NOT NULL,
    key_context text,
    created_at timestamp with time zone DEFAULT now(),
    expires_at timestamp with time zone
);

-- Grant permissions for pgsodium functions
GRANT EXECUTE ON FUNCTION pgsodium.crypto_secretbox(bytea, bytea, bytea) TO PUBLIC;
GRANT EXECUTE ON FUNCTION pgsodium.crypto_secretbox_open(bytea, bytea, bytea) TO PUBLIC;
GRANT EXECUTE ON FUNCTION pgsodium.randombytes_buf(integer) TO PUBLIC;
