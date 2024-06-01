
CREATE TABLE IF NOT EXISTS public.events
(
    event_id uuid unique NOT NULL DEFAULT uuid_generate_v4() primary key,
    -- If your model is owned by an account, you want to make sure you have an account_id column
    -- referencing the account table. Make sure you also set permissions appropriately
    account_id uuid not null references basejump.accounts(id),

    -- ADD YOUR COLUMNS HERE
    event_status TEXT NOT NULL,
    flow_id TEXT NOT NULL, -- the flow that was running UUID ( root flow name and stuff)
    flow_version_id TEXT NOT NULL, -- the version of the flow that was running UUID
    flow_version_name TEXT, -- the name of the flow version that was running example 0.0.1
    trigger_id TEXT NOT NULL, -- the trigger that caused the event
    trigger_session_id TEXT NOT NULL, -- anything that is triggered by a single trigger including nested flow runs
    trigger_session_status TEXT NOT NULL, -- the status of the trigger session
    flow_session_id TEXT NOT NULL, -- a single instance of a flow running
    flow_session_status TEXT NOT NULL, -- the status of the flow session
    node_id TEXT NOT NULL, -- the node that defined this event
    is_trigger BOOLEAN NOT NULL DEFAULT FALSE, -- if this event is a trigger event
    extension_id TEXT NOT NULL, -- the extension that processed this event
    stage TEXT NOT NULL, -- the stage of the event DEV OR PROD etc
    config json NOT NULL, -- the config used to run the flow
    context json, -- the bundle of args used for the action to process
    -- created_at timestamp with time zone DEFAULT (CURRENT_TIMESTAMP), --stats for action run time
    started_at timestamp with time zone, --stats for action run time
    ended_at timestamp with time zone, --stats for action run time
    debug_result json, -- debug info, a place where we can store extra data if we want like intermediate steps in the flow
    result json, -- the result of the action

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
CREATE TRIGGER set_events_timestamp
    BEFORE INSERT OR UPDATE ON public.events
    FOR EACH ROW
EXECUTE PROCEDURE basejump.trigger_set_timestamps();

-- protect the updated_by and created_by columns by setting them to be read-only and managed by a trigger
CREATE TRIGGER set_events_user_tracking
    BEFORE INSERT OR UPDATE ON public.events
    FOR EACH ROW
EXECUTE PROCEDURE basejump.trigger_set_user_tracking();


-- enable RLS on the table
ALTER TABLE public.events ENABLE ROW LEVEL SECURITY;


-- Because RLS is enabled, this table will NOT be accessible to any users by default
-- You must create a policy for each user that should have access to the table
-- Here are a few example policies that you may find useful when working with Basejump

----------------
-- Authenticated users should be able to read all records regardless of account
----------------
-- create policy "All logged in users can select" on public.events
--     for select
--     to authenticated
--     using (true);

----------------
-- Authenticated AND Anon users should be able to read all records regardless of account
----------------
-- create policy "All authenticated and anonymous users can select" on public.events
--     for select
--     to authenticated, anon
--     using (true);

-------------
-- Users should be able to read records that are owned by an account they belong to
--------------
-- create policy "Account members can select" on public.events
--     for select
--     to authenticated
--     using (
--     (account_id IN ( SELECT basejump.get_accounts_with_role()))
--     );


----------------
-- Users should be able to create records that are owned by an account they belong to
----------------
-- create policy "Account members can insert" on public.events
--     for insert
--     to authenticated
--     with check (
--     (account_id IN ( SELECT basejump.get_accounts_with_role()))
--     );

---------------
-- Users should be able to update records that are owned by an account they belong to
---------------
-- create policy "Account members can update" on public.events
--     for update
--     to authenticated
--     using (
--     (account_id IN ( SELECT basejump.get_accounts_with_role()))
--     );

----------------
-- Users should be able to delete records that are owned by an account they belong to
----------------
-- create policy "Account members can delete" on public.events
--     for delete
--     to authenticated
--     using (
--     (account_id IN ( SELECT basejump.get_accounts_with_role()))
--     );

----------------
-- Only account OWNERS should be able to delete records that are owned by an account they belong to
----------------
-- create policy "Account owners can delete" on public.events
--     for delete
--     to authenticated
--     using (
--     (account_id IN ( SELECT basejump.get_accounts_with_role("owner")))
--      );



