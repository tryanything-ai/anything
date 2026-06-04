-- SeaORM Seed Script for Development Environment
-- This script creates test users, organizations, and basic data for local development

-- Enable UUID extension if not already enabled
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create test users in the anything.users table
INSERT INTO anything.users (
    user_id,
    email,
    username,
    password_hash,
    created_at,
    updated_at
) VALUES
    (
        '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8',
        'user1@example.com',
        'user1',
        '$argon2id$v=19$m=65536,t=3,p=4$dGVzdHNhbHQ$testhash', -- This is a placeholder hash
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP
    ),
    (
        '5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f',
        'user2@example.com',
        'user2',
        '$argon2id$v=19$m=65536,t=3,p=4$dGVzdHNhbHQ$testhash',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP
    ),
    (
        '1e4f12a7-3c55-4e6d-9b4d-2a1f0403a2a6',
        'user3@example.com',
        'user3',
        '$argon2id$v=19$m=65536,t=3,p=4$dGVzdHNhbHQ$testhash',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP
    ),
    (
        '3d8b144c-1e9d-4a8c-8234-4e5c9b3d5c2f',
        'admin@example.com',
        'admin',
        '$argon2id$v=19$m=65536,t=3,p=4$dGVzdHNhbHQ$testhash',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP
    )
ON CONFLICT (user_id) DO NOTHING;

-- Create test organizations/accounts
INSERT INTO anything.accounts (
    id,
    name,
    slug,
    created_at,
    updated_at
) VALUES
    (
        'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332',
        'Acme Corporation',
        'acme-corp',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP
    ),
    (
        '7df12345-a5d3-4b13-9e3a-2f5c3e6a7b91',
        'Tech Startup Inc',
        'tech-startup',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP
    ),
    (
        '2a7b3d8e-2f3c-4b5d-8e3a-4a7c3e6a7c8d',
        'Freelance Developer',
        'freelance-dev',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP
    )
ON CONFLICT (id) DO NOTHING;

-- Link users to accounts with different roles
INSERT INTO anything.user_accounts (
    user_id,
    account_id,
    role,
    created_at
) VALUES
    -- User 1 is owner of Acme Corp
    ('0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8', 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'owner', CURRENT_TIMESTAMP),
    -- User 2 is owner of Tech Startup
    ('5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f', '7df12345-a5d3-4b13-9e3a-2f5c3e6a7b91', 'owner', CURRENT_TIMESTAMP),
    -- User 3 is member of Acme Corp
    ('1e4f12a7-3c55-4e6d-9b4d-2a1f0403a2a6', 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'member', CURRENT_TIMESTAMP),
    -- Admin user has access to all accounts
    ('3d8b144c-1e9d-4a8c-8234-4e5c9b3d5c2f', 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'admin', CURRENT_TIMESTAMP),
    ('3d8b144c-1e9d-4a8c-8234-4e5c9b3d5c2f', '7df12345-a5d3-4b13-9e3a-2f5c3e6a7b91', 'admin', CURRENT_TIMESTAMP),
    ('3d8b144c-1e9d-4a8c-8234-4e5c9b3d5c2f', '2a7b3d8e-2f3c-4b5d-8e3a-4a7c3e6a7c8d', 'owner', CURRENT_TIMESTAMP)
ON CONFLICT (user_id, account_id) DO NOTHING;

-- Create some test secrets
INSERT INTO anything.secrets (
    id,
    account_id,
    secret_name,
    secret_value,
    description,
    created_at,
    updated_at
) VALUES
    (
        uuid_generate_v4(),
        'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332',
        'OPENAI_API_KEY',
        'sk-test-openai-key-for-development',
        'OpenAI API key for development testing',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP
    ),
    (
        uuid_generate_v4(),
        'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332',
        'DATABASE_URL',
        'postgresql://user:password@localhost:5432/anything_dev',
        'Database connection string',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP
    ),
    (
        uuid_generate_v4(),
        '7df12345-a5d3-4b13-9e3a-2f5c3e6a7b91',
        'STRIPE_SECRET_KEY',
        'sk_test_stripe_key_for_development',
        'Stripe secret key for payment processing',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP
    )
ON CONFLICT (account_id, secret_name) DO NOTHING;

-- Create some test tasks
INSERT INTO anything.tasks (
    id,
    account_id,
    task_name,
    task_status,
    execution_time_ms,
    completed_at,
    created_at,
    updated_at
) VALUES
    (
        uuid_generate_v4(),
        'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332',
        'Data Processing Job',
        'completed',
        1500,
        CURRENT_TIMESTAMP - INTERVAL '1 hour',
        CURRENT_TIMESTAMP - INTERVAL '2 hours',
        CURRENT_TIMESTAMP - INTERVAL '1 hour'
    ),
    (
        uuid_generate_v4(),
        'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332',
        'Email Campaign',
        'pending',
        NULL,
        NULL,
        CURRENT_TIMESTAMP - INTERVAL '30 minutes',
        CURRENT_TIMESTAMP - INTERVAL '30 minutes'
    ),
    (
        uuid_generate_v4(),
        '7df12345-a5d3-4b13-9e3a-2f5c3e6a7b91',
        'API Integration Test',
        'failed',
        5000,
        NULL,
        CURRENT_TIMESTAMP - INTERVAL '15 minutes',
        CURRENT_TIMESTAMP - INTERVAL '15 minutes'
    )
ON CONFLICT (id) DO NOTHING;

-- Create some test files
INSERT INTO anything.files (
    id,
    account_id,
    filename,
    file_path,
    file_size,
    content_type,
    created_at,
    updated_at
) VALUES
    (
        uuid_generate_v4(),
        'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332',
        'report.pdf',
        '/uploads/reports/report.pdf',
        1024000,
        'application/pdf',
        CURRENT_TIMESTAMP - INTERVAL '1 day',
        CURRENT_TIMESTAMP - INTERVAL '1 day'
    ),
    (
        uuid_generate_v4(),
        'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332',
        'data.csv',
        '/uploads/data/data.csv',
        51200,
        'text/csv',
        CURRENT_TIMESTAMP - INTERVAL '6 hours',
        CURRENT_TIMESTAMP - INTERVAL '6 hours'
    ),
    (
        uuid_generate_v4(),
        '7df12345-a5d3-4b13-9e3a-2f5c3e6a7b91',
        'logo.png',
        '/uploads/images/logo.png',
        256000,
        'image/png',
        CURRENT_TIMESTAMP - INTERVAL '2 days',
        CURRENT_TIMESTAMP - INTERVAL '2 days'
    )
ON CONFLICT (id) DO NOTHING;

-- Create billing records for accounts
INSERT INTO anything.accounts_billing (
    id,
    account_id,
    tasks_used,
    tasks_limit,
    storage_used,
    storage_limit,
    stripe_customer_id,
    active,
    created_at,
    updated_at
) VALUES
    (
        uuid_generate_v4(),
        'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332',
        150,
        1000,
        1075200, -- 1MB + 512KB
        10485760, -- 10MB limit
        'cus_test_acme_corp',
        true,
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP
    ),
    (
        uuid_generate_v4(),
        '7df12345-a5d3-4b13-9e3a-2f5c3e6a7b91',
        75,
        500,
        256000, -- 256KB
        5242880, -- 5MB limit
        'cus_test_tech_startup',
        true,
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP
    ),
    (
        uuid_generate_v4(),
        '2a7b3d8e-2f3c-4b5d-8e3a-4a7c3e6a7c8d',
        25,
        100,
        0,
        1048576, -- 1MB limit
        'cus_test_freelance',
        true,
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP
    )
ON CONFLICT (account_id) DO NOTHING;

-- Create some test user sessions (for development testing)
INSERT INTO anything.user_sessions (
    session_id,
    user_id,
    created_at,
    expires_at,
    is_active
) VALUES
    (
        uuid_generate_v4(),
        '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP + INTERVAL '24 hours',
        true
    ),
    (
        uuid_generate_v4(),
        '3d8b144c-1e9d-4a8c-8234-4e5c9b3d5c2f',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP + INTERVAL '24 hours',
        true
    )
ON CONFLICT (session_id) DO NOTHING;

-- Display summary of created data
SELECT 'Seed data created successfully!' as message;

-- Show summary counts
SELECT 
    'Users' as table_name,
    COUNT(*) as count
FROM anything.users
UNION ALL
SELECT 
    'Accounts' as table_name,
    COUNT(*) as count
FROM anything.accounts
UNION ALL
SELECT 
    'User Accounts' as table_name,
    COUNT(*) as count
FROM anything.user_accounts
UNION ALL
SELECT 
    'Secrets' as table_name,
    COUNT(*) as count
FROM anything.secrets
UNION ALL
SELECT 
    'Tasks' as table_name,
    COUNT(*) as count
FROM anything.tasks
UNION ALL
SELECT 
    'Files' as table_name,
    COUNT(*) as count
FROM anything.files
UNION ALL
SELECT 
    'Billing Records' as table_name,
    COUNT(*) as count
FROM anything.accounts_billing
UNION ALL
SELECT 
    'Active Sessions' as table_name,
    COUNT(*) as count
FROM anything.user_sessions
WHERE is_active = true;
