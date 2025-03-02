CREATE TABLE IF NOT EXISTS anything.campaigns
(
    campaign_id uuid unique NOT NULL DEFAULT uuid_generate_v4() primary key,
    -- If your model is owned by an account, you want to make sure you have an account_id column
    -- referencing the account table. Make sure you also set permissions appropriately
    account_id uuid not null references basejump.accounts(id),

    -- ADD YOUR COLUMNS HERE
    agent_id uuid not null references anything.agents(agent_id),
    campaign_name TEXT NOT NULL,
    campaign_description TEXT NOT NULL,
    campaign_status TEXT NOT NULL, -- e.g. 'active', 'inactive', 'completed', etc
    
    -- Campaign scheduling settings - include all days as options
    schedule_days_of_week TEXT[] DEFAULT ARRAY['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday']::TEXT[],
    schedule_start_time TIME DEFAULT '09:00:00',
    schedule_end_time TIME DEFAULT '17:00:00',
    timezone TEXT DEFAULT 'America/New_York',
    
    active boolean not null default true,
    archived boolean not null default false,

    -- timestamps are useful for auditing
    -- Basejump has some convenience functions defined below for automatically handling these
    updated_at timestamp with time zone,
    created_at timestamp with time zone,
    -- Useful for tracking who made changes to a record
    -- Basejump has some convenience functions defined below for automatically handling these
    updated_by uuid references auth.users(id),
    created_by uuid references auth.users(id)
);

-- Add comments to explain the scheduling columns
COMMENT ON COLUMN anything.campaigns.schedule_days_of_week IS 'Array of days when the campaign is allowed to run, including weekends';
COMMENT ON COLUMN anything.campaigns.schedule_start_time IS 'Time of day when the campaign can start making calls (local time in specified timezone)';
COMMENT ON COLUMN anything.campaigns.schedule_end_time IS 'Time of day when the campaign should stop making calls (local time in specified timezone)';
COMMENT ON COLUMN anything.campaigns.timezone IS 'Timezone for interpreting the schedule times (e.g., "America/New_York")';

-- protect the timestamps by setting created_at and updated_at to be read-only and managed by a trigger
CREATE TRIGGER set_campaigns_timestamp
    BEFORE INSERT OR UPDATE ON anything.campaigns
    FOR EACH ROW
EXECUTE PROCEDURE basejump.trigger_set_timestamps();

-- protect the updated_by and created_by columns by setting them to be read-only and managed by a trigger
CREATE TRIGGER set_campaigns_user_tracking
    BEFORE INSERT OR UPDATE ON anything.campaigns
    FOR EACH ROW
EXECUTE PROCEDURE basejump.trigger_set_user_tracking();


-- enable RLS on the table
ALTER TABLE anything.campaigns ENABLE ROW LEVEL SECURITY;

-- Because RLS is enabled, this table will NOT be accessible to any users by default
-- You must create a policy for each user that should have access to the table
-- Here are a few example policies that you may find useful when working with Basejump

-------------
-- Users should be able to read records that are owned by an account they belong to
--------------
create policy "Account members can select" on anything.campaigns
    for select
    to authenticated
    using (
    (account_id IN ( SELECT basejump.get_accounts_with_role()))
    );


----------------
-- Users should be able to create records that are owned by an account they belong to
----------------
create policy "Account members can insert" on anything.campaigns
    for insert
    to authenticated
    with check (
    (account_id IN ( SELECT basejump.get_accounts_with_role()))
    );

---------------
-- Users should be able to update records that are owned by an account they belong to
---------------
create policy "Account members can update" on anything.campaigns
    for update
    to authenticated
    using (
    (account_id IN ( SELECT basejump.get_accounts_with_role()))
    );

----------------
-- Users should be able to delete records that are owned by an account they belong to
----------------
create policy "Account members can delete" on anything.campaigns
    for delete
    to authenticated
    using (
    (account_id IN ( SELECT basejump.get_accounts_with_role()))
    );