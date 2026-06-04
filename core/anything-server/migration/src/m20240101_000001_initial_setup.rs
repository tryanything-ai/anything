use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // This migration corresponds to 001_setup_pgsodium_and_auth.sql
        // For now, we'll use raw SQL since pgsodium functions aren't available in SeaORM schema builder
        
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                -- Enable pgsodium extension for encryption
                CREATE EXTENSION IF NOT EXISTS pgsodium;

                -- Create the anything schema first
                CREATE SCHEMA IF NOT EXISTS anything;

                -- Create users table for custom authentication
                CREATE TABLE IF NOT EXISTS anything.users (
                    user_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                    email VARCHAR(255) UNIQUE NOT NULL,
                    username VARCHAR(255) UNIQUE NOT NULL,
                    password_hash VARCHAR(255) NOT NULL,
                    failed_login_attempts INTEGER DEFAULT 0,
                    locked_until TIMESTAMP WITH TIME ZONE,
                    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
                );

                -- Create user sessions table for JWT session management
                CREATE TABLE IF NOT EXISTS anything.user_sessions (
                    session_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                    user_id UUID NOT NULL REFERENCES anything.users(user_id) ON DELETE CASCADE,
                    jwt_token_hash VARCHAR(255) NOT NULL,
                    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
                    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                    last_used_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
                );

                -- Create user_accounts table to link users to accounts
                CREATE TABLE IF NOT EXISTS anything.user_accounts (
                    user_id UUID NOT NULL REFERENCES anything.users(user_id) ON DELETE CASCADE,
                    account_id UUID NOT NULL,
                    role VARCHAR(50) DEFAULT 'member',
                    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                    PRIMARY KEY (user_id, account_id)
                );

                -- Create secrets table with pgsodium encryption
                CREATE TABLE IF NOT EXISTS anything.secrets (
                    secret_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                    account_id UUID NOT NULL,
                    secret_name VARCHAR(255) NOT NULL,
                    secret_value BYTEA,
                    nonce BYTEA,
                    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                    UNIQUE(account_id, secret_name)
                );

                -- Create indexes for performance
                CREATE INDEX IF NOT EXISTS idx_user_sessions_user_id ON anything.user_sessions(user_id);
                CREATE INDEX IF NOT EXISTS idx_user_sessions_expires_at ON anything.user_sessions(expires_at);
                CREATE INDEX IF NOT EXISTS idx_user_accounts_account_id ON anything.user_accounts(account_id);
                CREATE INDEX IF NOT EXISTS idx_secrets_account_id ON anything.secrets(account_id);

                -- Create function to encrypt secrets
                CREATE OR REPLACE FUNCTION anything.encrypt_secret(secret_text TEXT)
                RETURNS RECORD AS $$
                DECLARE
                    result RECORD;
                    key_id BIGINT;
                    nonce BYTEA;
                    encrypted BYTEA;
                BEGIN
                    -- Get or create a key for encryption
                    SELECT id INTO key_id FROM pgsodium.key WHERE name = 'anything_secrets_key';
                    
                    IF key_id IS NULL THEN
                        INSERT INTO pgsodium.key (name) VALUES ('anything_secrets_key') RETURNING id INTO key_id;
                    END IF;
                    
                    -- Generate a random nonce
                    nonce := pgsodium.crypto_aead_xchacha20poly1305_ietf_npubbytes();
                    
                    -- Encrypt the secret
                    encrypted := pgsodium.crypto_aead_xchacha20poly1305_ietf_encrypt(
                        secret_text::BYTEA,
                        NULL,
                        nonce,
                        pgsodium.crypto_aead_xchacha20poly1305_ietf_keygen()
                    );
                    
                    SELECT encrypted, nonce INTO result;
                    RETURN result;
                END;
                $$ LANGUAGE plpgsql SECURITY DEFINER;

                -- Create function to decrypt secrets
                CREATE OR REPLACE FUNCTION anything.decrypt_secret(encrypted_value BYTEA, secret_nonce BYTEA)
                RETURNS TEXT AS $$
                DECLARE
                    key_id BIGINT;
                    decrypted BYTEA;
                BEGIN
                    -- Get the key for decryption
                    SELECT id INTO key_id FROM pgsodium.key WHERE name = 'anything_secrets_key';
                    
                    IF key_id IS NULL THEN
                        RAISE EXCEPTION 'Encryption key not found';
                    END IF;
                    
                    -- Decrypt the secret
                    decrypted := pgsodium.crypto_aead_xchacha20poly1305_ietf_decrypt(
                        encrypted_value,
                        NULL,
                        secret_nonce,
                        (SELECT raw_key FROM pgsodium.key WHERE id = key_id)
                    );
                    
                    RETURN convert_from(decrypted, 'UTF8');
                END;
                $$ LANGUAGE plpgsql SECURITY DEFINER;
                "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop everything in reverse order
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                DROP FUNCTION IF EXISTS anything.decrypt_secret(BYTEA, BYTEA);
                DROP FUNCTION IF EXISTS anything.encrypt_secret(TEXT);
                DROP TABLE IF EXISTS anything.secrets;
                DROP TABLE IF EXISTS anything.user_accounts;
                DROP TABLE IF EXISTS anything.user_sessions;
                DROP TABLE IF EXISTS anything.users;
                DROP SCHEMA IF EXISTS anything CASCADE;
                "#,
            )
            .await?;

        Ok(())
    }
}
