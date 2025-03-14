/**
  * Creates a new team account and links the creating user to it
  * The anything.accounts_billing record will be created automatically via trigger
  * Returns the created account details including billing info
 */
create or replace function public.create_team(
    name text,
    slug text
)
    returns json
    language plpgsql
    security definer
    set search_path = public
as
$$
declare
    account_result json;
    team_account_id uuid;
    billing_info record;
    result json;
begin
    -- Use create_account to handle the account creation
    -- create_account will use auth.uid() by default if we don't pass primary_owner_user_id
    account_result := public.create_account(
        slug => slug,
        name => name
    );

    -- Extract the account_id from the result
    team_account_id := (account_result->>'account_id')::uuid;

    -- Get the billing info that was created by the trigger
    select * from anything.accounts_billing 
    where account_id = team_account_id 
    into billing_info;

    -- Prepare the return value with billing details
    select json_build_object(
        'account_id', team_account_id,
        'name', name,
        'slug', slug,
        'billing', json_build_object(
            'free_trial_started_at', billing_info.free_trial_started_at,
            'free_trial_ends_at', billing_info.free_trial_ends_at,
            'customer_status', billing_info.customer_status,
            'free_trial_task_limit', billing_info.free_trial_task_limit
        )
    ) into result;

    return result;
end;
$$;

-- Update permissions for the simplified signature
grant execute on function public.create_team(text, text) to authenticated;