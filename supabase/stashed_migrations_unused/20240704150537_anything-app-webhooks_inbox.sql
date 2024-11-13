-- Create the table to register webhooks for the account
CREATE TABLE IF NOT EXISTS anything.webhooks_inbox
(
    webhook_inbox_id unique NOT NULL DEFAULT uuid_generate_v4() primary key,
    account_id uuid NOT NULL REFERENCES basejump.accounts(id),
    webhook_endpoint_id UUID NOT NULL REFERENCES anything.webhook_endpoints(webhook_endpoint_id),
    webhook_path TEXT NOT NULL, -- we may need ways to target specific versions for testing etc
    payload JSONB NOT NULL,
    headers JSONB NOT NULL,
    stage TEXT NOT NULL,
    processing_status TEXT NOT NULL DEFAULT 'PENDING',
    requested_response_payload BOOLEAN NOT NULL DEFAULT FALSE,
    response_payload JSONB, --for if we generate and return a response
    response_headers JSONB, --for if we genreate and return a response
    responded_at timestamp with time zone,
    -- timestamps are useful for auditing
    received_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
   
    -- Basejump has some convenience functions defined below for automatically handling these
    updated_at timestamp with time zone,
    created_at timestamp with time zone,
    -- Useful for tracking who made changes to a record
    -- Basejump has some convenience functions defined below for automatically handling these
    updated_by uuid references auth.users(id),
    created_by uuid references auth.users(id)
);


-- protect the timestamps by setting created_at and updated_at to be read-only and managed by a trigger
CREATE TRIGGER set_account_webhook_endpoints_timestamp
    BEFORE INSERT OR UPDATE ON anything.account_webhook_endpoints
    FOR EACH ROW
EXECUTE PROCEDURE basejump.trigger_set_timestamps();

-- protect the updated_by and created_by columns by setting them to be read-only and managed by a trigger
CREATE TRIGGER set_account_webhook_endpoints_user_tracking
    BEFORE INSERT OR UPDATE ON anything.account_webhook_endpoints
    FOR EACH ROW
EXECUTE PROCEDURE basejump.trigger_set_user_tracking();


-- enable RLS on the table
ALTER TABLE anything.account_webhook_endpoints ENABLE ROW LEVEL SECURITY;

-------------
-- Users should be able to read records that are owned by an account they belong to
--------------
create policy "Account members can select" on anything.account_webhook_endpoints
    for select
    to authenticated
    using (
    (account_id IN ( SELECT basejump.get_accounts_with_role()))
    );
