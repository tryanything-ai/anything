
CREATE TABLE IF NOT EXISTS marketplace.flow_templates
(
    flow_template_id uuid unique NOT NULL DEFAULT uuid_generate_v4() primary key,
    -- If your model is owned by an account, you want to make sure you have an account_id column
    -- referencing the account table. Make sure you also set permissions appropriately
    account_id uuid not null references basejump.accounts(id),

    -- ADD YOUR COLUMNS HERE
    -- flow_template_id uuid not null default gen_random_uuid (),
    -- created_at timestamp with time zone not null default now(),
    flow_template_name text not null,
    flow_template_description text null,
    public_template boolean not null,
    approved boolean not null default true, -- if the action template is evil we can flag it as not approved to hide at some point
    publisher_id uuid not null, -- kind like the same as the above account_id i think
    anonymous_publish boolean not null,
    slug text not null,
    archived boolean not null default false, 
    -- constraint flow_templates_pkey1 primary key (flow_template_id),
    constraint flow_templates_slug_key unique (slug),
    constraint flow_templates_publisher_id_fkey foreign key (publisher_id) references marketplace.profiles(profile_id),

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
CREATE TRIGGER set_flow_templates_timestamp
    BEFORE INSERT OR UPDATE ON marketplace.flow_templates
    FOR EACH ROW
EXECUTE PROCEDURE basejump.trigger_set_timestamps();

-- protect the updated_by and created_by columns by setting them to be read-only and managed by a trigger
CREATE TRIGGER set_flow_templates_user_tracking
    BEFORE INSERT OR UPDATE ON marketplace.flow_templates
    FOR EACH ROW
EXECUTE PROCEDURE basejump.trigger_set_user_tracking();


-- enable RLS on the table
ALTER TABLE marketplace.flow_templates ENABLE ROW LEVEL SECURITY;


-- Because RLS is enabled, this table will NOT be accessible to any users by default
-- You must create a policy for each user that should have access to the table
-- Here are a few example policies that you may find useful when working with Basejump

----------------
-- Authenticated users should be able to read all records regardless of account
----------------
-- create policy "All logged in users can select" on marketplace.flow_templates
--     for select
--     to authenticated
--     using (true);

----------------
-- Authenticated AND Anon users should be able to read all records regardless of account
----------------
-- create policy "All authenticated and anonymous users can select" on marketplace.flow_templates
--     for select
--     to authenticated, anon
--     using (true);

-------------
-- Users should be able to read records that are owned by an account they belong to
--------------
-- create policy "Account members can select" on marketplace.flow_templates
--     for select
--     to authenticated
--     using (
--     (account_id IN ( SELECT basejump.get_accounts_with_role()))
--     );


----------------
-- Users should be able to create records that are owned by an account they belong to
----------------
-- create policy "Account members can insert" on marketplace.flow_templates
--     for insert
--     to authenticated
--     with check (
--     (account_id IN ( SELECT basejump.get_accounts_with_role()))
--     );

---------------
-- Users should be able to update records that are owned by an account they belong to
---------------
-- create policy "Account members can update" on marketplace.flow_templates
--     for update
--     to authenticated
--     using (
--     (account_id IN ( SELECT basejump.get_accounts_with_role()))
--     );

----------------
-- Users should be able to delete records that are owned by an account they belong to
----------------
-- create policy "Account members can delete" on marketplace.flow_templates
--     for delete
--     to authenticated
--     using (
--     (account_id IN ( SELECT basejump.get_accounts_with_role()))
--     );

----------------
-- Only account OWNERS should be able to delete records that are owned by an account they belong to
----------------
-- create policy "Account owners can delete" on marketplace.flow_templates
--     for delete
--     to authenticated
--     using (
--     (account_id IN ( SELECT basejump.get_accounts_with_role("owner")))
--      );


-- From GPT

-- Policy to allow all authenticated users to view public templates
CREATE POLICY "Public templates are visible to all authenticated users" ON marketplace.flow_templates
    FOR SELECT
    TO authenticated
    USING (public_template IS TRUE);

-- Policy to allow account owners to view their own non-public templates
CREATE POLICY "Account owners can view non-public templates" ON marketplace.flow_templates
    FOR SELECT
    TO authenticated
    USING (
        account_id IN (SELECT basejump.get_accounts_with_role('owner'))
        AND public_template IS FALSE
    );

-- Example policies for other operations (create, update, delete)

-- Policy to allow account members to create records
CREATE POLICY "Account members can insert" ON marketplace.flow_templates
    FOR INSERT
    TO authenticated
    WITH CHECK (
        account_id IN (SELECT basejump.get_accounts_with_role())
    );

-- Policy to allow account members to update their own records
CREATE POLICY "Account members can update" ON marketplace.flow_templates
    FOR UPDATE
    TO authenticated
    USING (
        account_id IN (SELECT basejump.get_accounts_with_role())
    );

-- Policy to allow account members to delete their own records
CREATE POLICY "Account members can delete" ON marketplace.flow_templates
    FOR DELETE
    TO authenticated
    USING (
        account_id IN (SELECT basejump.get_accounts_with_role())
    );

-- Policy to allow account owners to delete their own records
CREATE POLICY "Account owners can delete" ON marketplace.flow_templates
    FOR DELETE
    TO authenticated
    USING (
        account_id IN (SELECT basejump.get_accounts_with_role('owner'))
    );