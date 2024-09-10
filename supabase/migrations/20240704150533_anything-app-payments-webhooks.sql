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

-- Optionally, you can add a check within the function to ensure it's being called by the service role
CREATE OR REPLACE FUNCTION anything.get_user_by_id(user_id uuid)
RETURNS json
SECURITY DEFINER
AS $$
DECLARE
  user_data json;
BEGIN

  SELECT row_to_json(au)
  INTO user_data
  FROM auth.users au
  WHERE au.id = user_id;

  RETURN user_data;
END;
$$ LANGUAGE plpgsql;

-- Revoke execute permission from public and authenticated roles
REVOKE EXECUTE ON FUNCTION anything.get_user_by_id(uuid) FROM anon, authenticated; 

-- Grant execute permission only to the service role
GRANT EXECUTE ON FUNCTION anything.get_user_by_id(uuid) TO service_role;

-- How to do this with real variables so we don't need to hard code urls and stuff
-- https://github.com/orgs/supabase/discussions/12813#discussioncomment-10422025
-- store in vault and make a vault access function


-- FOr internal links for testin locally
-- https://github.com/supabase/supabase/issues/13005#issuecomment-1765482443 "host.docker.internal" is the way to go for url