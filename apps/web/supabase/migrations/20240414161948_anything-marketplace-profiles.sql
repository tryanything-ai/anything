
-- Create Table For Profiles
CREATE TABLE IF NOT EXISTS marketplace.profiles
(
    id uuid unique NOT NULL DEFAULT uuid_generate_v4() primary key,
    -- If your model is owned by an account, you want to make sure you have an account_id column
    -- referencing the account table. Make sure you also set permissions appropriately
    account_id uuid not null references basejump.accounts(id),

    -- ADD YOUR COLUMNS HERE
    username text null,
    full_name text null,
    avatar_url text null default 'https://fokcbrnvhnwnwwpiqkdc.supabase.co/storage/v1/object/marketplace/mocks/botttsNeutral-1698715092376.png'::text,
    website text null,
    twitter text null,
    tiktok text null,
    instagram text null,
    youtube text null,
    linkedin text null,
    github text null,
    public boolean not null default false,
    bio text null,
    archived boolean not null default false, 
    constraint profiles_username_key unique (username),
    constraint profiles_id_fkey foreign key (id) references auth.users(id) on delete cascade,
    constraint username_length check ((char_length(username) >= 3)),
   
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
CREATE TRIGGER set_profiles_timestamp
    BEFORE INSERT OR UPDATE ON marketplace.profiles
    FOR EACH ROW
EXECUTE PROCEDURE basejump.trigger_set_timestamps();

-- protect the updated_by and created_by columns by setting them to be read-only and managed by a trigger
CREATE TRIGGER set_profiles_user_tracking
    BEFORE INSERT OR UPDATE ON marketplace.profiles
    FOR EACH ROW
EXECUTE PROCEDURE basejump.trigger_set_user_tracking();


-- enable RLS on the table
ALTER TABLE marketplace.profiles ENABLE ROW LEVEL SECURITY;


-- Because RLS is enabled, this table will NOT be accessible to any users by default
-- You must create a policy for each user that should have access to the table
-- Here are a few example policies that you may find useful when working with Basejump

----------------
-- Authenticated users should be able to read all records regardless of account
----------------
create policy "All logged in users can select" on marketplace.profiles
    for select
    to authenticated
    USING (public IS TRUE);

----------------
-- Authenticated AND Anon users should be able to read all records regardless of account
----------------
create policy "All authenticated and anonymous users can select" on marketplace.profiles
    for select
    to authenticated, anon
     USING (public IS TRUE);

-------------
-- Users should be able to read records that are owned by an account they belong to
--------------
create policy "Account members can select" on marketplace.profiles
    for select
    to authenticated
    using (
    (account_id IN ( SELECT basejump.get_accounts_with_role()))
    );


----------------
-- Users should be able to create records that are owned by an account they belong to
----------------
create policy "Account members can insert" on marketplace.profiles
    for insert
    to authenticated
    with check (
    (account_id IN ( SELECT basejump.get_accounts_with_role()))
    );

---------------
-- Users should be able to update records that are owned by an account they belong to
---------------
create policy "Account members can update" on marketplace.profiles
    for update
    to authenticated
    using (
    (account_id IN ( SELECT basejump.get_accounts_with_role()))
    );

----------------
-- Users should be able to delete records that are owned by an account they belong to
----------------
-- create policy "Account members can delete" on marketplace.profiles
--     for delete
--     to authenticated
--     using (
--     (account_id IN ( SELECT basejump.get_accounts_with_role()))
--     );

----------------
-- Only account OWNERS should be able to delete records that are owned by an account they belong to
----------------
-- create policy "Account owners can delete" on marketplace.profiles
--     for delete
--     to authenticated
--     using (
--     (account_id IN ( SELECT basejump.get_accounts_with_role("owner")))
--      );


-- From GPT
-- To create a marketplace profile for every new user when they sign up so they can publish public stuff
-- Function to insert a new profile when a new user is created
-- CREATE OR REPLACE FUNCTION marketplace.create_profile_for_new_user()
-- RETURNS TRIGGER AS
-- $$
-- BEGIN
--     INSERT INTO marketplace.profiles (id, account_id, username, full_name, avatar_url, website, twitter, tiktok, instagram, youtube, linkedin, github, public, bio, archived, created_by, updated_by)
--     VALUES (NEW.id, NEW.id, NULL, NULL, 'https://fokcbrnvhnwnwwpiqkdc.supabase.co/storage/v1/object/marketplace/mocks/botttsNeutral-1698715092376.png', NULL, NULL, NULL, NULL, NULL, NULL, NULL, false, NULL, false, NEW.id, NEW.id);
--     RETURN NEW;
-- END;
-- $$ LANGUAGE plpgsql;

-- -- -- -- Trigger to call the function after a new user is created
-- CREATE TRIGGER create_profile_after_user_creation
-- AFTER INSERT ON auth.users
-- FOR EACH ROW
-- EXECUTE FUNCTION marketplace.create_profile_for_new_user();