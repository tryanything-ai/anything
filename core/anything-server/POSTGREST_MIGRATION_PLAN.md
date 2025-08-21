# Complete Postgrest to SeaORM Migration Plan

## Overview

Since we're doing a complete migration away from Supabase, we need to replace all Postgrest client calls with SeaORM operations. This involves:

1. ✅ **Created SeaORM helper modules** (`db_operations/`)
2. 🔄 **Replace all `anything_client`, `marketplace_client`, `public_client` usage**
3. ⏳ **Remove Postgrest from AppState**
4. ⏳ **Test all endpoints**

## Files That Need Conversion

Based on the grep search, here are all files using Postgrest clients:

### High Priority (Core Functionality)
- `src/workflows.rs` - ⚠️ **CRITICAL** - Main workflow management
- `src/tasks.rs` - ⚠️ **CRITICAL** - Task execution tracking
- `src/agents/` - 🔴 **HIGH** - Agent management (7 files)
- `src/secrets.rs` - ✅ **DONE** - Already replaced with pgsodium

### Medium Priority (Features)
- `src/testing.rs` - 🟡 **MEDIUM** - Workflow testing
- `src/variables.rs` - 🟡 **MEDIUM** - Variable management
- `src/charts.rs` - 🟡 **MEDIUM** - Analytics/charts
- `src/files/routes.rs` - 🟡 **MEDIUM** - File management

### Lower Priority (Admin/Setup)
- `src/auth/` - 🟢 **LOW** - Auth providers (already have custom auth)
- `src/billing/` - 🟢 **LOW** - Billing (5 files)
- `src/marketplace/` - 🟢 **LOW** - Marketplace (2 files)

## Conversion Strategy

### Phase 1: Core Database Operations ✅
- [x] Created `db_operations` module with SeaORM helpers
- [x] Implemented workflow operations
- [x] Implemented task operations
- [x] Implemented agent operations

### Phase 2: Replace Critical Endpoints 🔄
- [ ] Convert `workflows.rs` to use SeaORM
- [ ] Convert `tasks.rs` to use SeaORM
- [ ] Convert agent files to use SeaORM

### Phase 3: Replace Feature Endpoints
- [ ] Convert testing, variables, charts, files
- [ ] Update remaining auth endpoints
- [ ] Update marketplace endpoints

### Phase 4: Clean Up
- [ ] Remove Postgrest from dependencies
- [ ] Remove client references from AppState
- [ ] Update all remaining files

## Quick Conversion Pattern

### Before (Postgrest):
```rust
let client = &state.anything_client;
let response = match client
    .from("flows")
    .auth(user.jwt)
    .eq("account_id", &account_id)
    .select("*")
    .execute()
    .await
{
    Ok(response) => response,
    Err(e) => return Json(json!({"error": "Failed"})).into_response(),
};

let body = match response.text().await {
    Ok(body) => body,
    Err(_) => return Json(json!({"error": "Failed"})).into_response(),
};

Json(body).into_response()
```

### After (SeaORM):
```rust
let workflow_ops = crate::db_operations::workflows::WorkflowOperations::new(&state.db);

match workflow_ops.get_workflows(&account_id).await {
    Ok(workflows) => Json(workflows).into_response(),
    Err(e) => {
        println!("Error: {:?}", e);
        Json(json!({"error": "Failed to get workflows"})).into_response()
    }
}
```

## Implementation Steps

### 1. Replace workflows.rs (CRITICAL)
```bash
# Backup current file
cp src/workflows.rs src/workflows.rs.backup

# Replace with SeaORM version
mv src/workflows_seaorm.rs src/workflows.rs
```

### 2. Update main.rs routes
Replace the workflow routes to use the new SeaORM-based handlers.

### 3. Convert tasks.rs
Similar pattern - replace Postgrest calls with TaskOperations.

### 4. Convert agents/*.rs
Replace all 7 agent files with AgentOperations calls.

### 5. Test Each Conversion
After each major conversion:
```bash
cargo build
./test_auth.sh
# Test specific endpoints
```

## Current Status

### ✅ Completed
- Custom authentication system
- pgsodium secrets management
- SeaORM helper modules
- Database schema migration

### 🔄 In Progress
- Converting core workflow endpoints
- Removing Postgrest dependencies

### ⏳ Remaining
- Convert all 37 files using Postgrest
- Remove Postgrest from AppState
- Full testing of converted endpoints

## Estimated Effort

- **High Priority Files**: ~4-6 hours (workflows, tasks, agents)
- **Medium Priority Files**: ~3-4 hours (testing, variables, charts, files)
- **Low Priority Files**: ~2-3 hours (auth, billing, marketplace)
- **Testing & Cleanup**: ~2-3 hours

**Total**: ~11-16 hours for complete migration

## Benefits After Migration

1. **Zero Supabase Dependencies** - Completely self-contained
2. **Better Performance** - Direct database queries vs HTTP API calls
3. **Type Safety** - Full Rust type checking with SeaORM
4. **Easier Debugging** - No HTTP layer, direct SQL queries
5. **Cost Savings** - No Supabase subscription needed
6. **Full Control** - Own the entire data layer

## Risk Mitigation

1. **Backup Strategy** - Keep `.backup` files for all converted files
2. **Incremental Testing** - Test each conversion before moving to next
3. **Rollback Plan** - Can revert individual files if issues arise
4. **Parallel Development** - Keep both versions until fully tested

This migration will result in a completely Supabase-free, high-performance application with full control over the data layer.
