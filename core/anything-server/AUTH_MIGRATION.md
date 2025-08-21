# Authentication Migration from Supabase to Custom Auth

This document outlines the migration from Supabase-based authentication to a custom JWT-based authentication system with pgsodium for secrets management.

## Overview

We've implemented a complete custom authentication system that replaces Supabase Auth with:

1. **Custom JWT-based authentication** with username/password login
2. **PostgreSQL with pgsodium** for encrypted secrets storage
3. **Session management** with database-backed JWT token validation
4. **Account lockout** for security (5 failed attempts = 1 hour lockout)

## Database Schema

### New Tables Created

1. **`users`** - User accounts with encrypted passwords
   - `user_id` (UUID, primary key)
   - `username` (unique)
   - `email` (unique)
   - `password_hash` (Argon2 hashed)
   - `is_active`, `email_verified`, `failed_login_attempts`, `locked_until`
   - Timestamps: `created_at`, `updated_at`, `last_login_at`

2. **`user_sessions`** - JWT session management
   - `session_id` (UUID, primary key)
   - `user_id` (foreign key to users)
   - `token_hash` (SHA256 hash of JWT token)
   - `expires_at`, `created_at`, `last_used_at`
   - `user_agent`, `ip_address` (for tracking)

3. **`user_accounts`** - Links users to existing account system
   - `user_account_id` (UUID, primary key)
   - `user_id` (foreign key to users)
   - `account_id` (links to existing account system)
   - `role` (owner, admin, member, etc.)

4. **`anything.secrets`** (updated) - Encrypted secrets with pgsodium
   - `secret_id` (UUID, primary key)
   - `account_id` (links to existing accounts)
   - `secret_name` (unique per account)
   - `secret_value_encrypted` (bytea - encrypted with pgsodium)
   - `nonce` (bytea - encryption nonce)
   - `description`, `is_api_key`, `archived`
   - User tracking: `created_by`, `updated_by` (links to users)

## API Endpoints

### Public Authentication Endpoints

- `POST /auth/register` - Register new user
- `POST /auth/login` - Login with username/password
- `POST /auth/logout` - Logout (invalidate session)
- `GET /auth/me` - Get current user info (protected)

### Protected Secrets Endpoints (JWT Auth Required)

- `GET /account/:account_id/secrets_new` - List secrets (names only)
- `POST /account/:account_id/secret_new` - Create encrypted secret
- `GET /account/:account_id/secret_new/:secret_id` - Get decrypted secret
- `PUT /account/:account_id/secret_new/:secret_id` - Update secret
- `DELETE /account/:account_id/secret_new/:secret_id` - Archive secret

## Security Features

### Password Security
- **Argon2** password hashing (industry standard)
- **Account lockout** after 5 failed login attempts (1 hour)
- **Password complexity** requirements (minimum 8 characters)

### JWT Security
- **24-hour token expiration**
- **Session tracking** in database for revocation
- **Token hash validation** (prevents token reuse if database is compromised)
- **User activity tracking** (last login, session management)

### Secrets Encryption
- **pgsodium** for database-level encryption
- **Separate nonce** per secret for security
- **User audit trail** (created_by, updated_by tracking)

## Implementation Details

### Code Structure

```
src/
├── custom_auth/
│   ├── mod.rs           # Module exports
│   ├── handlers.rs      # Auth endpoint handlers
│   ├── jwt.rs           # JWT token management
│   ├── password.rs      # Argon2 password hashing
│   ├── middleware.rs    # JWT authentication middleware
│   └── extractors.rs    # Axum extractors for auth data
├── pgsodium_secrets/
│   ├── mod.rs           # Module exports
│   ├── handlers.rs      # Secrets CRUD endpoints
│   └── encryption.rs    # pgsodium encryption functions
└── entities/
    ├── users.rs         # SeaORM entity for users
    ├── user_sessions.rs # SeaORM entity for sessions
    ├── user_accounts.rs # SeaORM entity for user-account links
    └── secrets.rs       # Updated SeaORM entity for encrypted secrets
```

### Authentication Flow

1. **Registration**: User creates account with username/email/password
2. **Login**: Validates credentials, creates session, returns JWT
3. **Request**: Client sends JWT in `Authorization: Bearer <token>` header
4. **Validation**: Middleware validates JWT, checks session in database
5. **Authorization**: Handler receives authenticated user info via extractors

### Migration Steps

1. ✅ Added dependencies (argon2, bcrypt, hex, rand_core)
2. ✅ Created database migration with pgsodium setup
3. ✅ Implemented SeaORM entities for new tables
4. ✅ Built JWT authentication system
5. ✅ Created auth middleware and extractors
6. ✅ Implemented encrypted secrets management
7. ✅ Added new routes alongside existing Supabase routes

## Environment Variables

Add these to your `.env` file:

```env
JWT_SECRET=your-very-secret-jwt-key-change-this-in-production
DATABASE_URL=postgresql://postgres:password@localhost:5432/your_db
```

## Next Steps

1. **Run migration**: Execute `migrations/001_setup_pgsodium_and_auth.sql`
2. **Test endpoints**: Use the new auth endpoints for frontend integration
3. **Migrate existing data**: Move existing secrets to encrypted format
4. **Update frontend**: Replace Supabase auth calls with new endpoints
5. **Deprecate Supabase**: Remove Supabase auth once migration is complete

## Migration Strategy

### Phase 1: Parallel Operation
- Keep existing Supabase auth working
- Add new custom auth endpoints
- Test thoroughly with new system

### Phase 2: Frontend Migration
- Update frontend to use new auth endpoints
- Implement user registration/login flows
- Migrate existing user accounts

### Phase 3: Data Migration
- Migrate existing secrets to encrypted format
- Update workflows to use new secrets endpoints
- Verify all functionality works

### Phase 4: Cleanup
- Remove Supabase auth dependencies
- Remove old auth middleware
- Clean up unused routes and code

## Production Considerations

1. **Key Management**: Implement proper encryption key rotation for pgsodium
2. **Rate Limiting**: Add rate limiting to auth endpoints
3. **Email Verification**: Implement email verification for new accounts
4. **Password Reset**: Add password reset functionality
5. **2FA**: Consider adding two-factor authentication
6. **Audit Logging**: Enhanced logging for security events
7. **Database Backup**: Ensure encrypted data is properly backed up

## Testing

The system includes:
- User registration and login
- JWT token validation
- Session management
- Encrypted secrets storage
- Account lockout protection
- User-account relationship management

Test the endpoints with tools like curl or Postman to verify functionality before frontend integration.
