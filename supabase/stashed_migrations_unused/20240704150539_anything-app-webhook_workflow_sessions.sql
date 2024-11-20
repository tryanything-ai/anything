-- Create the table to register webhooks for the account
CREATE TABLE IF NOT EXISTS anything.webhook_workflow_sessions
(   
    webhook_workflow_session_id unique NOT NULL DEFAULT uuid_generate_v4() primary key,
    webhook_inbox_id unique NOT NULL REFERENCES anything.webhooks_inbox(webhook_inbox_id),
    account_id uuid NOT NULL REFERENCES basejump.accounts(id),
    webhook_endpoint_id UUID NOT NULL REFERENCES anything.webhook_endpoints(webhook_endpoint_id),
    workflow_id UUID NOT NULL REFERENCES anything.workflows(workflow_id),
    workflow_version_id UUID NOT NULL REFERENCES anything.workflow_versions(workflow_version_id),
    task_id UUID NOT NULL REFERENCES anything.tasks(task_id),
  
    -- Basejump has some convenience functions defined below for automatically handling these
    updated_at timestamp with time zone,
    created_at timestamp with time zone,
    -- Useful for tracking who made changes to a record
    -- Basejump has some convenience functions defined below for automatically handling these
    updated_by uuid references auth.users(id),
    created_by uuid references auth.users(id)
);

-- protect the timestamps by setting created_at and updated_at to be read-only and managed by a trigger
CREATE TRIGGER set_account_webhook_workflow_sessions_timestamp
    BEFORE INSERT OR UPDATE ON anything.account_webhook_workflow_sessions
    FOR EACH ROW
EXECUTE PROCEDURE basejump.trigger_set_timestamps();

-- protect the updated_by and created_by columns by setting them to be read-only and managed by a trigger
CREATE TRIGGER set_account_webhook_workflow_sessions_user_tracking
    BEFORE INSERT OR UPDATE ON anything.account_webhook_workflow_sessions
    FOR EACH ROW
EXECUTE PROCEDURE basejump.trigger_set_user_tracking();

-- enable RLS on the table
ALTER TABLE anything.account_webhook_workflow_sessions ENABLE ROW LEVEL SECURITY;

-------------
-- Users should be able to read records that are owned by an account they belong to
--------------
create policy "Account members can select" on anything.account_webhook_workflow_sessions
    for select
    to authenticated
    using (
    (account_id IN ( SELECT basejump.get_accounts_with_role()))
    );
