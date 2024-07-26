
-- 
-- 
--  Created as seperate file to allow for managing new marketplace profiles
--  and managing accounts with sql on creation
--
--

-- trigger the function whenever a new account is created
CREATE TRIGGER basejump_add_current_user_to_new_account
    AFTER INSERT
    ON basejump.accounts
    FOR EACH ROW
EXECUTE FUNCTION basejump.add_current_user_to_new_account();

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
    first_account_id    uuid;
    generated_user_name text;
begin

    -- first we setup the user profile
    -- TODO: see if we can get the user's name from the auth.users table once we learn how oauth works
    if new.email IS NOT NULL then
        generated_user_name := split_part(new.email, '@', 1);
    end if;
    -- create the new users's personal account
    insert into basejump.accounts (name, primary_owner_user_id, personal_account, id)
    values (generated_user_name, NEW.id, true, NEW.id)
    returning id into first_account_id;

    -- add them to the account_user table so they can act on it
    insert into basejump.account_user (account_id, user_id, account_role)
    values (first_account_id, NEW.id, 'owner');

     -- Create a new profile for the user in the marketplace.profiles table
    INSERT INTO marketplace.profiles (id, account_id, username, full_name, avatar_url, website, twitter, tiktok, instagram, youtube, linkedin, github, public, bio, archived, created_by, updated_by)
    VALUES (NEW.id, first_account_id, generated_user_name, NULL, 'https://fokcbrnvhnwnwwpiqkdc.supabase.co/storage/v1/object/marketplace/mocks/botttsNeutral-1698715092376.png', NULL, NULL, NULL, NULL, NULL, NULL, NULL, false, NULL, false, NEW.id, NEW.id);

    return NEW;
end;
$$;

-- trigger the function every time a user is created
create trigger on_auth_user_created
    after insert
    on auth.users
    for each row
execute procedure basejump.run_new_user_setup();
