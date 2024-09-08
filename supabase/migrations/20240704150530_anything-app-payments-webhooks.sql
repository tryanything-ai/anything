create trigger "new_account_webhook" after insert
on "basejump"."accounts" for each row
execute function "supabase_functions"."http_request"(
--   'http://localhost:3001/billing/webhooks/new_account_webhook', 0.0.0.0:3001
  'http://host.docker.internal:3001/billing/webhooks/new_account_webhook', --https://github.com/supabase/supabase/issues/13005#issuecomment-1765482443
  'POST',
  '{"Content-Type":"application/json"}',
  '{}',
  '5000'
);

-- How to do this with real variables so we don't need to hard code urls and stuff
-- https://github.com/orgs/supabase/discussions/12813#discussioncomment-10422025
-- store in vault and make a vault access function


-- FOr internal links for testin locally
-- https://github.com/supabase/supabase/issues/13005#issuecomment-1765482443 "host.docker.internal" is the way to go for url