
-- 
-- 
--  Created as seperate file to allow for managing new marketplace profiles
--  and managing accounts with sql on creation
--
--


/**
  * When a user signs up, we need to create a personal account for them
  * and add them to the account_user table so they can act on it
 */
create or replace function basejump.run_new_user_setup()
    returns trigger
    language plpgsql
    security definer
    set search_path = public
as
$$
declare
    personal_account_id uuid;
    team_account_id uuid;
    generated_user_name text;
    generated_slug text;
    team_account json;
begin
    -- Generate username from email
   if new.email IS NOT NULL then
        generated_user_name := initcap(split_part(new.email, '@', 1));
        generated_slug := lower(split_part(new.email, '@', 1));
    end if;
    -- if new.email IS NOT NULL then
    --     generated_user_name := split_part(new.email, '@', 1);
    -- end if;

    -- Create personal account
    insert into basejump.accounts (name, primary_owner_user_id, personal_account, id)
    values (generated_user_name, NEW.id, true, NEW.id)
    returning id into personal_account_id;

    -- Add user to personal account
    insert into basejump.account_user (account_id, user_id, account_role)
    values (personal_account_id, NEW.id, 'owner');

    -- Create team account 
    team_account := public.create_account(
        slug => generated_slug || '-team',
        name => generated_user_name || '''s Team',
        primary_owner_user_id => NEW.id
    );

    team_account_id := (team_account->>'account_id')::uuid;

    -- Add user to the account_user table so they can act on it
     insert into basejump.account_user (account_id, user_id, account_role)
     values (team_account_id, NEW.id, 'owner'); 

    -- Create profile
    INSERT INTO marketplace.profiles (profile_id, account_id, username, full_name, avatar_url, website, twitter, tiktok, instagram, youtube, linkedin, github, public, bio, archived, created_by, updated_by)
    VALUES (NEW.id, personal_account_id, generated_user_name, NULL, 'https://fokcbrnvhnwnwwpiqkdc.supabase.co/storage/v1/object/marketplace/mocks/botttsNeutral-1698715092376.png', NULL, NULL, NULL, NULL, NULL, NULL, NULL, false, NULL, false, NEW.id, NEW.id);

    return NEW;
end;
$$;
-- create or replace function basejump.run_new_user_setup()
--     returns trigger
--     language plpgsql
--     security definer
--     set search_path = public
-- as
-- $$
-- declare
--     first_account_id    uuid;
--     generated_user_name text;
-- begin

--     -- first we setup the user profile
--     -- TODO: see if we can get the user's name from the auth.users table once we learn how oauth works
--     if new.email IS NOT NULL then
--         generated_user_name := split_part(new.email, '@', 1);
--     end if;
--     -- create the new users's personal account
--     insert into basejump.accounts (name, primary_owner_user_id, personal_account, id)
--     values (generated_user_name, NEW.id, true, NEW.id)
--     returning id into first_account_id;

--     -- add them to the account_user table so they can act on it
--     insert into basejump.account_user (account_id, user_id, account_role)
--     values (first_account_id, NEW.id, 'owner');

--     -- Create team account
--     team_account := public.create_account(
--         slug => generated_user_name || '-team',
--         name => generated_user_name || '''s Team'
--     );

--     team_account_id := (team_account->>'account_id')::uuid;

--      -- Create a new profile for the user in the marketplace.profiles table
--     INSERT INTO marketplace.profiles (id, account_id, username, full_name, avatar_url, website, twitter, tiktok, instagram, youtube, linkedin, github, public, bio, archived, created_by, updated_by)
--     VALUES (NEW.id, first_account_id, generated_user_name, NULL, 'https://fokcbrnvhnwnwwpiqkdc.supabase.co/storage/v1/object/marketplace/mocks/botttsNeutral-1698715092376.png', NULL, NULL, NULL, NULL, NULL, NULL, NULL, false, NULL, false, NEW.id, NEW.id);

--     return NEW;
-- end;
-- $$;

-- trigger the function every time a user is created
create trigger on_auth_user_created
    after insert
    on auth.users
    for each row
execute procedure basejump.run_new_user_setup();
