# PostgREST to SeaORM Migration Plan

## Current Status
We've successfully migrated the **authentication system** from Supabase to custom JWT + SeaORM, but many files still use PostgREST clients for database operations.

## Why Keep PostgREST For Now?
For an unlaunched product, the pragmatic approach is:

1. **Phase 1**: Migrate auth and secrets (✅ DONE)
2. **Phase 2**: Gradually migrate core database operations to SeaORM
3. **Phase 3**: Remove PostgREST entirely

## PostgREST Files That Need Migration

### High Priority (Core functionality):
- `src/workflows.rs` - Workflow CRUD operations
- `src/tasks.rs` - Task management
- `src/actions.rs` - Action management
- `src/charts.rs` - Analytics/charts

### Medium Priority (Features):
- `src/agents/` - Agent management
- `src/marketplace/` - Marketplace functionality
- `src/auth/` - Auth provider management
- `src/billing/` - Billing operations

### Low Priority (Supporting):
- `src/bundler/` - Bundling operations
- `src/files/` - File management
- `src/processor/` - Background processing

## Migration Strategy

### Option 1: Gradual Migration (Recommended)
Keep PostgREST for now but ensure it connects to your own database:

```rust
// In main.rs - Point PostgREST to your own database
let anything_client = Arc::new(
    Postgrest::new(&database_url)  // Your own DB, not Supabase
        .schema("anything")
);
```

### Option 2: Complete Migration
Convert all database operations to SeaORM entities and queries.

## Environment Setup for Gradual Migration

```env
# Remove Supabase entirely:
# SUPABASE_URL=
# SUPABASE_API_KEY=
# SUPABASE_SERVICE_ROLE_API_KEY=

# Use your own database for everything:
DATABASE_URL=postgresql://postgres:password@localhost:5432/your_db

# Custom auth:
JWT_SECRET=your-secret-key
```

## Key Benefits Achieved
✅ **No Supabase auth dependency**
✅ **Custom JWT authentication**
✅ **Encrypted secrets with pgsodium**
✅ **Full control over user management**
✅ **Cost savings** (no Supabase auth billing)

## Next Steps for Complete Migration

1. **Start with workflows.rs** - Convert to SeaORM entities
2. **Then tasks.rs** - Core task management
3. **Gradually convert other files**
4. **Remove PostgREST dependency entirely**

## Current Working State

The system is currently in a **hybrid state**:
- ✅ Authentication: Custom JWT + SeaORM
- ✅ Secrets: pgsodium + SeaORM  
- 🔄 Other operations: PostgREST → Your own database (not Supabase)

This gives you:
- **Independence from Supabase** 
- **Working authentication system**
- **Encrypted secrets management**
- **Path to full SeaORM migration**
