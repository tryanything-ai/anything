
CREATE TABLE IF NOT EXISTS marketplace.flow_template_versions
(
    flow_template_version_id uuid unique NOT NULL DEFAULT uuid_generate_v4() primary key,
    -- If your model is owned by an account, you want to make sure you have an account_id column
    -- referencing the account table. Make sure you also set permissions appropriately
    account_id uuid not null references basejump.accounts(id),

    -- ADD YOUR COLUMNS HERE
    -- flow_template_version_id uuid not null default gen_random_uuid (),
    -- created_at timestamp with time zone not null default now(),
    flow_template_version_name text not null,
    flow_definition jsonb not null,
    public boolean not null default false,
    flow_template_version text not null default ''::text,
    publisher_id uuid not null,
    flow_template_id uuid not null,
    commit_message text null,
    anything_flow_version text not null,
    recommended_version boolean not null default false,
    archived boolean not null default false, 
    -- constraint flow_templates_pkey primary key (flow_template_version_id),
    constraint flow_template_versions_flow_template_id_fkey foreign key (flow_template_id) references marketplace.flow_templates(flow_template_id),
    constraint flow_template_versions_publisher_id_fkey foreign key (publisher_id) references marketplace.profiles(profile_id),
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
CREATE TRIGGER set_flow_template_versions_timestamp
    BEFORE INSERT OR UPDATE ON marketplace.flow_template_versions
    FOR EACH ROW
EXECUTE PROCEDURE basejump.trigger_set_timestamps();

-- protect the updated_by and created_by columns by setting them to be read-only and managed by a trigger
CREATE TRIGGER set_flow_template_versions_user_tracking
    BEFORE INSERT OR UPDATE ON marketplace.flow_template_versions
    FOR EACH ROW
EXECUTE PROCEDURE basejump.trigger_set_user_tracking();

-- enable RLS on the table
ALTER TABLE marketplace.flow_template_versions ENABLE ROW LEVEL SECURITY;


-- Because RLS is enabled, this table will NOT be accessible to any users by default
-- You must create a policy for each user that should have access to the table
-- Here are a few example policies that you may find useful when working with Basejump

----------------
-- Authenticated users should be able to read all records regardless of account
----------------
-- create policy "All logged in users can select" on marketplace.flow_template_versions
--     for select
--     to authenticated
--     using (true);

-- -- Authenticated AND Anon users should be able to read all public records
-- create policy "All authenticated and anonymous users can select public templates" on marketplace.flow_template_versions
--     for select
--     to authenticated, anon
--     using (public = true);

-- Policy to allow all authenticated users to view public templates
CREATE POLICY "Public templates are visible to anyone" ON marketplace.flow_template_versions
    FOR SELECT
    TO authenticated, anon
    USING (public IS TRUE);

-- --------------
-- Authenticated AND Anon users should be able to read all records regardless of account
-- -- --------------
-- create policy "All authenticated and anonymous users can select" on marketplace.flow_template_versions
--     for select
--     to authenticated, anon
--     using (true);

-------------
-- Users should be able to read records that are owned by an account they belong to
--------------
-- create policy "Account members can select" on marketplace.flow_template_versions
--     for select
--     to authenticated
--     using (
--     (account_id IN ( SELECT basejump.get_accounts_with_role()))
--     );


----------------
-- Users should be able to create records that are owned by an account they belong to
----------------
-- create policy "Account members can insert" on marketplace.flow_template_versions
--     for insert
--     to authenticated
--     with check (
--     (account_id IN ( SELECT basejump.get_accounts_with_role()))
--     );

---------------
-- Users should be able to update records that are owned by an account they belong to
---------------
-- create policy "Account members can update" on marketplace.flow_template_versions
--     for update
--     to authenticated
--     using (
--     (account_id IN ( SELECT basejump.get_accounts_with_role()))
--     );

----------------
-- Users should be able to delete records that are owned by an account they belong to
----------------
-- create policy "Account members can delete" on marketplace.flow_template_versions
--     for delete
--     to authenticated
--     using (
--     (account_id IN ( SELECT basejump.get_accounts_with_role()))
--     );

----------------
-- Only account OWNERS should be able to delete records that are owned by an account they belong to
----------------
-- create policy "Account owners can delete" on marketplace.flow_template_versions
--     for delete
--     to authenticated
--     using (
--     (account_id IN ( SELECT basejump.get_accounts_with_role("owner")))
--      );



