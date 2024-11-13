-- Create the table to register webhooks for the account
CREATE TABLE IF NOT EXISTS anything.webhook_endpoints
(
    webhook_endpoint_id unique NOT NULL DEFAULT uuid_generate_v4() primary key,
    account_id uuid NOT NULL REFERENCES basejump.accounts(id),
    webhook_path text NOT NULL UNIQUE, -- maybe allow vanity paths? ( do that when you launch rest api)
    webhook_title text NOT NULL,
    webhook_description text,
    -- timestamps are useful for auditing
    -- Basejump has some convenience functions defined below for automatically handling these
    updated_at timestamp with time zone,
    created_at timestamp with time zone,
    -- Useful for tracking who made changes to a record
    -- Basejump has some convenience functions defined below for automatically handling these
    updated_by uuid references auth.users(id),
    created_by uuid references auth.users(id)
);


-- protect the timestamps by setting created_at and updated_at to be read-only and managed by a trigger
CREATE TRIGGER set_webhook_endpoints_timestamp
    BEFORE INSERT OR UPDATE ON anything.webhook_endpoints
    FOR EACH ROW
EXECUTE PROCEDURE basejump.trigger_set_timestamps();

-- protect the updated_by and created_by columns by setting them to be read-only and managed by a trigger
CREATE TRIGGER set_webhook_endpoints_user_tracking
    BEFORE INSERT OR UPDATE ON anything.webhook_endpoints
    FOR EACH ROW
EXECUTE PROCEDURE basejump.trigger_set_user_tracking();


-- enable RLS on the table
ALTER TABLE anything.webhook_endpoints ENABLE ROW LEVEL SECURITY;

-------------
-- Users should be able to read records that are owned by an account they belong to
--------------
create policy "Account members can select" on anything.webhook_endpoints
    for select
    to authenticated
    using (
    (account_id IN ( SELECT basejump.get_accounts_with_role()))
    );


----------------
-- Users should be able to create records that are owned by an account they belong to
----------------
create policy "Account members can insert" on anything.webhook_endpoints
    for insert
    to authenticated
    with check (
    (account_id IN ( SELECT basejump.get_accounts_with_role()))
    );

---------------
-- Users should be able to update records that are owned by an account they belong to
---------------
-- create policy "Account members can update" on anything.webhook_endpoints
--     for update
--     to authenticated
--     using (
--     (account_id IN ( SELECT basejump.get_accounts_with_role()))
--     );

----------------
-- Users should be able to delete records that are owned by an account they belong to
----------------
create policy "Account members can delete" on anything.webhook_endpoints
    for delete
    to authenticated
    using (
    (account_id IN ( SELECT basejump.get_accounts_with_role()))
    );