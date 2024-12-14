# How to start

```bash
cd apps/web
supabase start
cd ...
pnpm dev --filter=web
```

# How to reset supabase db

```bash
supabase db reset
```
  
# Supabase guide on local development

https://supabase.com/docs/guides/cli/local-development

Supabase CLI Reference
https://supabase.com/docs/reference/cli/supabase-stop?queryGroups=example&example=supabase-stop-clean-up

Supabase Local Dashboard URL
http://127.0.0.1:54323/project/default

# How we dev migrations for DB.

-> Locally. We do Supabase DB Reset on Dev Locally a bunch
-> When ready -> Staging -> Github actions deploy DB migrations
-> When good -> Main -> Github actions deploy db migrations again to main project

# Vercel Setup

Previews all run with "Staging Supabase Variables"
Production runs with "Real Supabase Variables" and Railway Server Prod also

# Reset Supabase Staging DB Command

```bash
npx supabase db reset --linked
```

# How to set Auth Providers Secrets in Supabase Vault

```bash
curl -X POST 'http://localhost:3001/auth/providers/airtable/client_id/set' \
-H 'Content-Type: application/json' \
-d '{
  "client_id": "test-vault-secret-id-for-airtable",
  "cli_secret": "carls-secret"
}'
```
