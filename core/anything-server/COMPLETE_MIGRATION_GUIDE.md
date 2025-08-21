# Complete Supabase Migration Guide

## Overview

This guide covers the complete migration from Supabase authentication and vault to a custom PostgreSQL solution with pgsodium encryption. Since this is an unlaunched product, we're doing a full migration rather than gradual.

## ✅ What's Been Completed

### 1. Custom Authentication System
- JWT-based authentication with username/password
- Argon2 password hashing
- Session management with database-backed validation
- Account lockout protection (5 failed attempts = 1 hour)

### 2. Database Schema with pgsodium
- `users` table with encrypted passwords
- `user_sessions` table for JWT session tracking
- `user_accounts` table linking users to accounts
- `anything.secrets` table using pgsodium encryption

### 3. API Endpoints
- `POST /auth/register` - User registration
- `POST /auth/login` - User login  
- `GET /auth/me` - Current user info
- `POST /auth/logout` - Session invalidation
- Full CRUD for encrypted secrets at `/account/:id/secret*`

### 4. Security Features
- JWT middleware for route protection
- Encrypted secrets with pgsodium
- User audit trails
- Session management

## 🚧 Remaining Migration Steps

### Step 1: Database Migration
```bash
# Run the migration to set up pgsodium and auth tables
psql $DATABASE_URL -f migrations/001_setup_pgsodium_and_auth.sql
```

### Step 2: Environment Variables
Update your `.env` file:
```env
# Remove these Supabase variables:
# SUPABASE_URL=
# SUPABASE_API_KEY=
# SUPABASE_SERVICE_ROLE_API_KEY=
# SUPABASE_JWT_SECRET=

# Keep these for database access:
DATABASE_URL=postgresql://postgres:password@localhost:5432/your_db

# Add these for custom auth:
JWT_SECRET=your-very-secret-jwt-key-change-this-in-production
ANYTHING_BASE_URL=http://localhost:3000
```

### Step 3: Fix Compilation Issues
The main compilation issues are:

1. **Remove Supabase service role API key references** - Replace with direct database access
2. **Update vault calls** - Replace with pgsodium secret calls
3. **Fix HTTP response handling** - Some `.text()` calls need type annotations

### Step 4: Frontend Migration
Update your frontend to:

1. **Replace Supabase auth calls:**
```javascript
// OLD: Supabase auth
const { data, error } = await supabase.auth.signIn({ email, password })

// NEW: Custom auth
const response = await fetch('/auth/login', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({ username, password })
})
```

2. **Update secret management:**
```javascript
// OLD: Supabase vault
const { data } = await supabase.rpc('get_secret', { secret_name })

// NEW: Custom pgsodium
const response = await fetch(`/account/${accountId}/secret/${secretId}`, {
  headers: { 'Authorization': `Bearer ${token}` }
})
```

3. **Update session management:**
```javascript
// Store JWT token from login response
localStorage.setItem('auth_token', response.token)

// Include in requests
fetch('/api/endpoint', {
  headers: { 'Authorization': `Bearer ${token}` }
})
```

## 🔧 Quick Compilation Fixes

### Fix 1: Remove Supabase Service Role Dependencies
Replace all occurrences of:
```rust
let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
    .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");
```

With:
```rust
// Direct database access - no service role key needed
```

### Fix 2: Update Vault Calls
Replace vault operations with pgsodium secret operations:
```rust
// OLD: vault::get_secret(&client, secret_name).await
// NEW: pgsodium_secrets::get_secret(state, account_id, secret_id).await
```

### Fix 3: Fix HTTP Response Types
Add type annotations where needed:
```rust
let body: String = match response.text().await {
    Ok(body) => body,
    Err(e) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
};
```

## 📋 Migration Checklist

- [x] Custom auth system implemented
- [x] Database schema with pgsodium created
- [x] JWT middleware implemented  
- [x] Secrets CRUD with encryption
- [ ] Run database migration
- [ ] Fix compilation errors
- [ ] Update environment variables
- [ ] Test auth endpoints
- [ ] Update frontend auth calls
- [ ] Test secret management
- [ ] Remove old Supabase code

## 🧪 Testing

Use the provided test script:
```bash
./test_auth.sh
```

Or test manually:
```bash
# Register user
curl -X POST http://localhost:3001/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"test","email":"test@example.com","password":"testpass123"}'

# Login
curl -X POST http://localhost:3001/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"test","password":"testpass123"}'
```

## 🚀 Production Considerations

1. **Key Management**: Implement proper pgsodium key rotation
2. **Rate Limiting**: Add to auth endpoints
3. **Email Verification**: For new accounts
4. **Password Reset**: Implement flow
5. **2FA**: Consider adding
6. **Monitoring**: Auth events and failures
7. **Backup**: Encrypted data backup strategy

## 📝 Code Changes Summary

### Files Modified:
- `src/main.rs` - Updated routes and middleware
- `src/custom_auth/` - New auth system
- `src/pgsodium_secrets/` - New secrets management
- `src/entities/` - New database entities
- `Cargo.toml` - Added auth dependencies

### Files to Remove:
- `src/supabase_jwt_middleware.rs` ✅ (removed)
- Any Supabase-specific configurations

### Environment Changes:
- Remove all `SUPABASE_*` variables
- Add `JWT_SECRET`
- Keep `DATABASE_URL` for direct access

This migration provides a complete replacement for Supabase functionality while maintaining security and adding database-level encryption for secrets.
