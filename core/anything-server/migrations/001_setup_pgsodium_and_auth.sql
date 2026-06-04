-- Enable pgsodium extension for encryption
CREATE EXTENSION IF NOT EXISTS pgsodium;

-- Create the anything schema first
CREATE SCHEMA IF NOT EXISTS anything;

-- Create users table for custom authentication
CREATE TABLE IF NOT EXISTS anything.users (
    user_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(255) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Create user_sessions table for session management
CREATE TABLE IF NOT EXISTS anything.user_sessions (
    session_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES anything.users(user_id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMPTZ NOT NULL,
    is_active BOOLEAN DEFAULT true
);

-- Create user_accounts table for linking users to accounts
CREATE TABLE IF NOT EXISTS anything.user_accounts (
    user_id UUID NOT NULL REFERENCES anything.users(user_id) ON DELETE CASCADE,
    account_id UUID NOT NULL,
    role VARCHAR(50) DEFAULT 'member',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, account_id)
);

-- Update secrets table to use pgsodium encryption
-- First drop the existing table if it exists
DROP TABLE IF EXISTS anything.secrets CASCADE;

-- Create the new secrets table with pgsodium encryption
CREATE TABLE anything.secrets (
    secret_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL,
    secret_name VARCHAR(255) NOT NULL,
    secret_value_encrypted BYTEA NOT NULL, -- Encrypted using pgsodium
    nonce BYTEA NOT NULL, -- Nonce for encryption
    description TEXT,
    is_api_key BOOLEAN DEFAULT false,
    archived BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_by UUID,
    updated_by UUID,
    UNIQUE(account_id, secret_name)
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_user_sessions_user_id ON anything.user_sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_user_sessions_expires_at ON anything.user_sessions(expires_at);
CREATE INDEX IF NOT EXISTS idx_user_accounts_user_id ON anything.user_accounts(user_id);
CREATE INDEX IF NOT EXISTS idx_user_accounts_account_id ON anything.user_accounts(account_id);
CREATE INDEX IF NOT EXISTS idx_secrets_account_id ON anything.secrets(account_id);
CREATE INDEX IF NOT EXISTS idx_secrets_archived ON anything.secrets(archived);

-- Create a function to auto-update the updated_at timestamp
CREATE OR REPLACE FUNCTION anything.update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Add triggers to auto-update updated_at columns
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON anything.users 
    FOR EACH ROW EXECUTE FUNCTION anything.update_updated_at_column();

CREATE TRIGGER update_secrets_updated_at BEFORE UPDATE ON anything.secrets 
    FOR EACH ROW EXECUTE FUNCTION anything.update_updated_at_column();

-- Grant necessary permissions for pgsodium functions
-- These permissions are needed for the encryption/decryption operations
GRANT EXECUTE ON FUNCTION pgsodium.crypto_secretbox(bytea, bytea, bytea) TO PUBLIC;
GRANT EXECUTE ON FUNCTION pgsodium.crypto_secretbox_open(bytea, bytea, bytea) TO PUBLIC;
GRANT EXECUTE ON FUNCTION pgsodium.randombytes_buf(integer) TO PUBLIC;

-- Create a default master key for encryption (in production, this should be more secure)
INSERT INTO pgsodium.key (name, key_type, key_context)
VALUES ('anything_master_key', 'aead-det', 'anything_secrets')
ON CONFLICT (name) DO NOTHING;

COMMIT;