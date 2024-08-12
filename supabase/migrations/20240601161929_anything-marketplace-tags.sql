
CREATE TABLE IF NOT EXISTS marketplace.tags
(
    id text unique NOT NULL primary key,
    -- If your model is owned by an account, you want to make sure you have an account_id column
    -- referencing the account table. Make sure you also set permissions appropriately
    -- account_id uuid not null references accounts(id),

    -- ADD YOUR COLUMNS HERE
    -- created_at timestamp with time zone not null default now(),
    tag_uuid uuid not null DEFAULT uuid_generate_v4(),
    tag_label text not null,
    tag_slug text not null,
    tag_icon text null,
    archived boolean not null default false, 
    constraint tags_tag_slug_key unique (tag_slug),
    constraint tags_tag_uuid_key unique (tag_uuid),

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
CREATE TRIGGER set_tags_timestamp
    BEFORE INSERT OR UPDATE ON marketplace.tags
    FOR EACH ROW
EXECUTE PROCEDURE basejump.trigger_set_timestamps();

-- protect the updated_by and created_by columns by setting them to be read-only and managed by a trigger
CREATE TRIGGER set_tags_user_tracking
    BEFORE INSERT OR UPDATE ON marketplace.tags
    FOR EACH ROW
EXECUTE PROCEDURE basejump.trigger_set_user_tracking();


-- enable RLS on the table
ALTER TABLE marketplace.tags ENABLE ROW LEVEL SECURITY;


-- Because RLS is enabled, this table will NOT be accessible to any users by default
-- You must create a policy for each user that should have access to the table
-- Here are a few example policies that you may find useful when working with Basejump

----------------
-- Authenticated users should be able to read all records regardless of account
----------------
create policy "All logged in users can select" on marketplace.tags
    for select
    to authenticated
    using (true);

----------------
-- Authenticated AND Anon users should be able to read all records regardless of account
----------------
create policy "All authenticated and anonymous users can select" on marketplace.tags
    for select
    to authenticated, anon
    using (true);

-------------
-- Users should be able to read records that are owned by an account they belong to
--------------
-- create policy "Account members can select" on marketplace.tags
--     for select
--     to authenticated
--     using (
--     (account_id IN ( SELECT basejump.get_accounts_with_role()))
--     );


----------------
-- Users should be able to create records that are owned by an account they belong to
----------------
-- create policy "Account members can insert" on marketplace.tags
--     for insert
--     to authenticated
--     with check (
--     (account_id IN ( SELECT basejump.get_accounts_with_role()))
--     );

---------------
-- Users should be able to update records that are owned by an account they belong to
---------------
-- create policy "Account members can update" on marketplace.tags
--     for update
--     to authenticated
--     using (
--     (account_id IN ( SELECT basejump.get_accounts_with_role()))
--     );

----------------
-- Users should be able to delete records that are owned by an account they belong to
----------------
-- create policy "Account members can delete" on marketplace.tags
--     for delete
--     to authenticated
--     using (
--     (account_id IN ( SELECT basejump.get_accounts_with_role()))
--     );

----------------
-- Only account OWNERS should be able to delete records that are owned by an account they belong to
----------------
-- create policy "Account owners can delete" on marketplace.tags
--     for delete
--     to authenticated
--     using (
--     (account_id IN ( SELECT basejump.get_accounts_with_role("owner")))
--      );





