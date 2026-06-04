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

-- Create a simplified secrets table (without pgsodium for now)
-- TODO: Add pgsodium encryption when extension is available
CREATE TABLE IF NOT EXISTS anything.secrets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL,
    secret_name VARCHAR(255) NOT NULL,
    secret_value TEXT NOT NULL, -- This should be encrypted in production
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(account_id, secret_name)
);

-- Create core application tables that SeaORM expects

-- Accounts table
CREATE TABLE IF NOT EXISTS anything.accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Tasks table  
CREATE TABLE IF NOT EXISTS anything.tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES anything.accounts(id) ON DELETE CASCADE,
    task_name VARCHAR(255) NOT NULL,
    task_status VARCHAR(50) DEFAULT 'pending',
    execution_time_ms BIGINT,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Billing table
CREATE TABLE IF NOT EXISTS anything.accounts_billing (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES anything.accounts(id) ON DELETE CASCADE,
    tasks_used INTEGER DEFAULT 0,
    tasks_limit INTEGER,
    storage_used BIGINT DEFAULT 0,
    storage_limit BIGINT,
    stripe_customer_id VARCHAR(255),
    active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(account_id)
);

-- Files table
CREATE TABLE IF NOT EXISTS anything.files (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES anything.accounts(id) ON DELETE CASCADE,
    filename VARCHAR(255) NOT NULL,
    file_path TEXT,
    file_size BIGINT,
    content_type VARCHAR(255),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Add indexes for performance
CREATE INDEX IF NOT EXISTS idx_user_sessions_user_id ON anything.user_sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_user_sessions_expires_at ON anything.user_sessions(expires_at);
CREATE INDEX IF NOT EXISTS idx_tasks_account_id ON anything.tasks(account_id);
CREATE INDEX IF NOT EXISTS idx_tasks_status ON anything.tasks(task_status);
CREATE INDEX IF NOT EXISTS idx_files_account_id ON anything.files(account_id);
CREATE INDEX IF NOT EXISTS idx_secrets_account_id ON anything.secrets(account_id);

-- Create some sample data for testing
INSERT INTO anything.accounts (id, name, slug) 
VALUES ('00000000-0000-0000-0000-000000000000', 'Default Account', 'default')
ON CONFLICT (slug) DO NOTHING;

COMMIT;
