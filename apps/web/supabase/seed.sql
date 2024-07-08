
-- inspired by https://gist.github.com/khattaksd/4e8f4c89f4e928a2ecaad56d4a17ecd1
-- Create test users
INSERT INTO auth.users (
    instance_id,
    id,
    aud,
    role,
    email,
    encrypted_password,
    email_confirmed_at,
    recovery_sent_at,
    last_sign_in_at,
    raw_app_meta_data,
    raw_user_meta_data,
    created_at,
    updated_at,
    confirmation_token,
    email_change,
    email_change_token_new,
    recovery_token
) VALUES
    ('00000000-0000-0000-0000-000000000000', '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8', 'authenticated', 'authenticated', 'user1@example.com', crypt('password123', gen_salt('bf')), current_timestamp, current_timestamp, current_timestamp, '{"provider":"email","providers":["email"]}', '{}', current_timestamp, current_timestamp, '', '', '', ''),
    ('00000000-0000-0000-0000-000000000000', '5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f', 'authenticated', 'authenticated', 'user2@example.com', crypt('password123', gen_salt('bf')), current_timestamp, current_timestamp, current_timestamp, '{"provider":"email","providers":["email"]}', '{}', current_timestamp, current_timestamp, '', '', '', ''),
    ('00000000-0000-0000-0000-000000000000', '1e4f12a7-3c55-4e6d-9b4d-2a1f0403a2a6', 'authenticated', 'authenticated', 'user3@example.com', crypt('password123', gen_salt('bf')), current_timestamp, current_timestamp, current_timestamp, '{"provider":"email","providers":["email"]}', '{}', current_timestamp, current_timestamp, '', '', '', ''),
    ('00000000-0000-0000-0000-000000000000', '3d8b144c-1e9d-4a8c-8234-4e5c9b3d5c2f', 'authenticated', 'authenticated', 'user4@example.com', crypt('password123', gen_salt('bf')), current_timestamp, current_timestamp, current_timestamp, '{"provider":"email","providers":["email"]}', '{}', current_timestamp, current_timestamp, '', '', '', ''),
    ('00000000-0000-0000-0000-000000000000', '2a7b3d8e-2f3c-4b5d-8e3a-4a7c3e6a7c8d', 'authenticated', 'authenticated', 'user5@example.com', crypt('password123', gen_salt('bf')), current_timestamp, current_timestamp, current_timestamp, '{"provider":"email","providers":["email"]}', '{}', current_timestamp, current_timestamp, '', '', '', '');

-- inspired by https://gist.github.com/khattaksd/4e8f4c89f4e928a2ecaad56d4a17ecd1
-- test user email identities
INSERT INTO
    auth.identities (
        id,
        user_id,
        -- New column
        provider_id,
        identity_data,
        provider,
        last_sign_in_at,
        created_at,
        updated_at
    ) (
        select
            uuid_generate_v4 (),
            id,
            -- New column
            id,
            format('{"sub":"%s","email":"%s"}', id :: text, email) :: jsonb,
            'email',
            current_timestamp,
            current_timestamp,
            current_timestamp
        from
            auth.users
    );

-- Inserting sample accounts into basejump.accounts
INSERT INTO basejump.accounts (
    id, primary_owner_user_id, name, slug, personal_account, updated_at, created_at, created_by, updated_by
) VALUES
    ('c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8', 'Account 1', 'account-1', false, now(), now(), '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8', '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8'),
    ('7df12345-a5d3-4b13-9e3a-2f5c3e6a7b91', '5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f', 'Account 2', 'account-2', false, now(), now(), '5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f', '5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f');

-- Inserting sample users into basejump.account_user table
INSERT INTO basejump.account_user (user_id, account_id, account_role)
VALUES
    ('0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8', 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'owner'),
    ('5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f', '7df12345-a5d3-4b13-9e3a-2f5c3e6a7b91', 'owner'),
    ('1e4f12a7-3c55-4e6d-9b4d-2a1f0403a2a6', 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'member'),
    ('3d8b144c-1e9d-4a8c-8234-4e5c9b3d5c2f', '7df12345-a5d3-4b13-9e3a-2f5c3e6a7b91', 'member'),
    ('2a7b3d8e-2f3c-4b5d-8e3a-4a7c3e6a7c8d', 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'member');

-- Insert into marketplace profiles
-- Inserting sample profiles into marketplace.profiles
INSERT INTO marketplace.profiles (
    id, account_id, username, full_name, avatar_url, website, twitter, tiktok, instagram, youtube, linkedin, github, public, bio, updated_at, created_at, updated_by, created_by
) VALUES
    ('0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8', 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'user1', 'User One', 'https://fokcbrnvhnwnwwpiqkdc.supabase.co/storage/v1/object/marketplace/mocks/botttsNeutral-1698715092376.png', 'https://user1.example.com', '@user1', '@user1', '@user1', 'https://youtube.com/user1', 'https://linkedin.com/in/user1', 'https://github.com/user1', true, 'Bio of User One', now(), now(), '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8', '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8'),
    ('5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f', '7df12345-a5d3-4b13-9e3a-2f5c3e6a7b91', 'user2', 'User Two', 'https://fokcbrnvhnwnwwpiqkdc.supabase.co/storage/v1/object/marketplace/mocks/botttsNeutral-1698715092376.png', 'https://user2.example.com', '@user2', '@user2', '@user2', 'https://youtube.com/user2', 'https://linkedin.com/in/user2', 'https://github.com/user2', true, 'Bio of User Two', now(), now(), '5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f', '5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f'),
    ('1e4f12a7-3c55-4e6d-9b4d-2a1f0403a2a6', 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'user3', 'User Three', 'https://fokcbrnvhnwnwwpiqkdc.supabase.co/storage/v1/object/marketplace/mocks/botttsNeutral-1698715092376.png', 'https://user3.example.com', '@user3', '@user3', '@user3', 'https://youtube.com/user3', 'https://linkedin.com/in/user3', 'https://github.com/user3', true, 'Bio of User Three', now(), now(), '1e4f12a7-3c55-4e6d-9b4d-2a1f0403a2a6', '1e4f12a7-3c55-4e6d-9b4d-2a1f0403a2a6'),
    ('3d8b144c-1e9d-4a8c-8234-4e5c9b3d5c2f', '7df12345-a5d3-4b13-9e3a-2f5c3e6a7b91', 'user4', 'User Four', 'https://fokcbrnvhnwnwwpiqkdc.supabase.co/storage/v1/object/marketplace/mocks/botttsNeutral-1698715092376.png', 'https://user4.example.com', '@user4', '@user4', '@user4', 'https://youtube.com/user4', 'https://linkedin.com/in/user4', 'https://github.com/user4', true, 'Bio of User Four', now(), now(), '3d8b144c-1e9d-4a8c-8234-4e5c9b3d5c2f', '3d8b144c-1e9d-4a8c-8234-4e5c9b3d5c2f'),
    ('2a7b3d8e-2f3c-4b5d-8e3a-4a7c3e6a7c8d', 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'user5', 'User Five', 'https://fokcbrnvhnwnwwpiqkdc.supabase.co/storage/v1/object/marketplace/mocks/botttsNeutral-1698715092376.png', 'https://user5.example.com', '@user5', '@user5', '@user5', 'https://youtube.com/user5', 'https://linkedin.com/in/user5', 'https://github.com/user5', true, 'Bio of User Five', now(), now(), '2a7b3d8e-2f3c-4b5d-8e3a-4a7c3e6a7c8d', '2a7b3d8e-2f3c-4b5d-8e3a-4a7c3e6a7c8d');

-- Insert sample tags into the tags table
INSERT INTO marketplace.tags (id, tag_uuid, tag_label, tag_slug, tag_icon, updated_at, created_at, updated_by, created_by)
VALUES
    ('school', uuid_generate_v4(), 'School', 'school', null, now(), now(), null, null),
    ('work', uuid_generate_v4(), 'Work', 'work', null, now(), now(), null, null),
    ('dev', uuid_generate_v4(), 'Development', 'dev', null, now(), now(), null, null),
    ('content', uuid_generate_v4(), 'Content', 'content', null, now(), now(), null, null);

-- Inserting sample flow templates into marketplace.flow_templates
INSERT INTO marketplace.flow_templates (
    flow_template_id, account_id, flow_template_name, flow_template_description, public_template, publisher_id, anonymous_publish, slug, updated_at, created_at, updated_by, created_by
) VALUES
    ('11111111-1111-1111-1111-111111111111', 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'Template 1', 'Description for Template 1', true, '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8', false, 'template-1', now(), now(), '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8', '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8'),
    ('22222222-2222-2222-2222-222222222222', '7df12345-a5d3-4b13-9e3a-2f5c3e6a7b91', 'Template 2', 'Description for Template 2', false, '5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f', true, 'template-2', now(), now(), '5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f', '5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f'),
    ('33333333-3333-3333-3333-333333333333', 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'Template 3', 'Description for Template 3', true, '1e4f12a7-3c55-4e6d-9b4d-2a1f0403a2a6', false, 'template-3', now(), now(), '1e4f12a7-3c55-4e6d-9b4d-2a1f0403a2a6', '1e4f12a7-3c55-4e6d-9b4d-2a1f0403a2a6'),
    ('44444444-4444-4444-4444-444444444444', '7df12345-a5d3-4b13-9e3a-2f5c3e6a7b91', 'Template 4', 'Description for Template 4', false, '3d8b144c-1e9d-4a8c-8234-4e5c9b3d5c2f', true, 'template-4', now(), now(), '3d8b144c-1e9d-4a8c-8234-4e5c9b3d5c2f', '3d8b144c-1e9d-4a8c-8234-4e5c9b3d5c2f'),
    ('55555555-5555-5555-5555-555555555555', 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'Template 5', 'Description for Template 5', true, '2a7b3d8e-2f3c-4b5d-8e3a-4a7c3e6a7c8d', false, 'template-5', now(), now(), '2a7b3d8e-2f3c-4b5d-8e3a-4a7c3e6a7c8d', '2a7b3d8e-2f3c-4b5d-8e3a-4a7c3e6a7c8d');

-- Inserting sample flow template versions into marketplace.flow_template_versions
-- Inserting sample flow template versions into marketplace.flow_template_versions
INSERT INTO marketplace.flow_template_versions (
    flow_template_version_id, account_id, flow_template_version_name, flow_template_json, public_template, flow_template_version, publisher_id, flow_template_id, commit_message, anything_flow_version, recommended_version, updated_at, created_at, updated_by, created_by
) VALUES
    (uuid_generate_v4(), 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'Template 1 Version 1', '{"steps": []}', true, 'v1', '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8', '11111111-1111-1111-1111-111111111111', 'Initial version', 'v1.0', true, now(), now(), '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8', '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8'),
    (uuid_generate_v4(), 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'Template 1 Version 2', '{"steps": [{"action": "step1"}]}', true, 'v2', '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8', '11111111-1111-1111-1111-111111111111', 'Added step 1', 'v2.0', false, now(), now(), '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8', '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8'),
    (uuid_generate_v4(), '7df12345-a5d3-4b13-9e3a-2f5c3e6a7b91', 'Template 2 Version 1', '{"steps": []}', false, 'v1', '5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f', '22222222-2222-2222-2222-222222222222', 'Initial version', 'v1.0', true, now(), now(), '5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f', '5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f'),
    (uuid_generate_v4(), 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'Template 3 Version 1', '{"steps": []}', true, 'v1', '1e4f12a7-3c55-4e6d-9b4d-2a1f0403a2a6', '33333333-3333-3333-3333-333333333333', 'Initial version', 'v1.0', true, now(), now(), '1e4f12a7-3c55-4e6d-9b4d-2a1f0403a2a6', '1e4f12a7-3c55-4e6d-9b4d-2a1f0403a2a6'),
    (uuid_generate_v4(), '7df12345-a5d3-4b13-9e3a-2f5c3e6a7b91', 'Template 4 Version 1', '{"steps": []}', false, 'v1', '3d8b144c-1e9d-4a8c-8234-4e5c9b3d5c2f', '44444444-4444-4444-4444-444444444444', 'Initial version', 'v1.0', true, now(), now(), '3d8b144c-1e9d-4a8c-8234-4e5c9b3d5c2f', '3d8b144c-1e9d-4a8c-8234-4e5c9b3d5c2f'),
    (uuid_generate_v4(), 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'Template 5 Version 1', '{"steps": []}', true, 'v1', '2a7b3d8e-2f3c-4b5d-8e3a-4a7c3e6a7c8d', '55555555-5555-5555-5555-555555555555', 'Initial version', 'v1.0', true, now(), now(), '2a7b3d8e-2f3c-4b5d-8e3a-4a7c3e6a7c8d', '2a7b3d8e-2f3c-4b5d-8e3a-4a7c3e6a7c8d');

-- Inserting sample flow template tags into marketplace.flow_template_tags
INSERT INTO marketplace.flow_template_tags (
    tag_id, account_id, flow_template_id, updated_at, created_at, updated_by, created_by
) VALUES
    ('school', 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', '11111111-1111-1111-1111-111111111111', now(), now(), '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8', '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8'),
    ('work', '7df12345-a5d3-4b13-9e3a-2f5c3e6a7b91', '22222222-2222-2222-2222-222222222222', now(), now(), '5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f', '5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f'),
    ('dev', 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', '33333333-3333-3333-3333-333333333333', now(), now(), '1e4f12a7-3c55-4e6d-9b4d-2a1f0403a2a6', '1e4f12a7-3c55-4e6d-9b4d-2a1f0403a2a6'),
    ('content', '7df12345-a5d3-4b13-9e3a-2f5c3e6a7b91', '44444444-4444-4444-4444-444444444444', now(), now(), '3d8b144c-1e9d-4a8c-8234-4e5c9b3d5c2f', '3d8b144c-1e9d-4a8c-8234-4e5c9b3d5c2f'),
    ('school', 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', '55555555-5555-5555-5555-555555555555', now(), now(), '2a7b3d8e-2f3c-4b5d-8e3a-4a7c3e6a7c8d', '2a7b3d8e-2f3c-4b5d-8e3a-4a7c3e6a7c8d');

---------------------------------------
--- APPLICATION WORKFLOW MANAGEMENT ---
---------------------------------------
-- Inserting sample flows into anything.flows
INSERT INTO anything.flows (
    flow_id, account_id, flow_name, active, updated_at, created_at, updated_by, created_by
) VALUES
    ('aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa', 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'Flow 1', true, now(), now(), '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8', '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8'),
    ('bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb', '7df12345-a5d3-4b13-9e3a-2f5c3e6a7b91', 'Flow 2', false, now(), now(), '5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f', '5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f'),
    ('cccccccc-cccc-cccc-cccc-cccccccccccc', 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'Flow 3', true, now(), now(), '1e4f12a7-3c55-4e6d-9b4d-2a1f0403a2a6', '1e4f12a7-3c55-4e6d-9b4d-2a1f0403a2a6'),
    ('dddddddd-dddd-dddd-dddd-dddddddddddd', '7df12345-a5d3-4b13-9e3a-2f5c3e6a7b91', 'Flow 4', true, now(), now(), '3d8b144c-1e9d-4a8c-8234-4e5c9b3d5c2f', '3d8b144c-1e9d-4a8c-8234-4e5c9b3d5c2f'); 
    -- ('eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee', 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'Flow 5', false, now(), now(), '2a7b3d8e-2f3c-4b5d-8e3a-4a7c3e6a7c8d', '2a7b3d8e-2f3c-4b5d-8e3a-4a7c3e6a7c8d');

-- Inserting sample flow versions into anything.flow_versions
INSERT INTO anything.flow_versions (
    flow_version_id, account_id, flow_id, flow_version, description, checksum, published, flow_definition, updated_at, created_at, updated_by, created_by
) VALUES
    ('11111111-1111-1111-1111-111111111111', 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa', 'v1.0', 'Initial version of Flow 1', 'checksum1', true, 
    '{
      "edges": [
   {
      "id": "reactflow__edge-example_actionb-example_action_2a",
      "source": "example_action",
      "sourceHandle": "b",
      "target": "example_action_2",
      "targetHandle": "a",
      "type": "anything"
    },
    {
      "id": "reactflow__edge-example_action_2b-other_example_actiona",
      "source": "example_action_2",
      "sourceHandle": "b",
      "target": "other_example_action",
      "targetHandle": "a",
      "type": "anything"
    },
    {
      "id": "reactflow__edge-anything_inputb-example_actiona",
      "source": "anything_input",
      "sourceHandle": "b",
      "target": "example_action",
      "targetHandle": "a",
      "type": "anything"
    },
    {
      "id": "reactflow__edge-other_example_actionb-anything_outputa",
      "source": "other_example_action",
      "sourceHandle": "b",
      "target": "anything_output",
      "targetHandle": "a",
      "type": "anything"
    }
    ],
    "actions": [
      {
      "anything_action_version": "1.0.0",
      "type": "action",
      "plugin_id": "anything_input",
      "node_id": "anything_input", 
      "plugin_version": "1.0.0",
      "label": "Inputs",
      "description": "Inputs for the flow",
      "icon": "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"24\" height=\"24\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\" class=\"lucide lucide-braces\"><path d=\"M8 3H7a2 2 0 0 0-2 2v5a2 2 0 0 1-2 2 2 2 0 0 1 2 2v5c0 1.1.9 2 2 2h1\"/><path d=\"M16 21h1a2 2 0 0 0 2-2v-5c0-1.1.9-2 2-2a2 2 0 0 1-2-2V5a2 2 0 0 0-2-2h-1\"/></svg>",
      "variables": {},
      "input": {},
      "input_schema": {},
      "presentation": {
        "position": {
          "x": 300,
          "y": 100
        }
      },
      "handles": [
        {
          "id": "b",
          "type": "source",
          "position": "bottom"
        }
      ]
    },
    {
      "anything_action_version": "1.0.0",
      "type": "action",
      "plugin_id": "example_plugin",
      "node_id": "example_action", 
      "plugin_version": "1.0.0",
      "label": "Example Action 1",
      "description": "This is the first example action",
      "icon": "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 320 320\"><path d=\"m297.06 130.97c7.26-21.79 4.76-45.66-6.85-65.48-17.46-30.4-52.56-46.04-86.84-38.68-15.25-17.18-37.16-26.95-60.13-26.81-35.04-.08-66.13 22.48-76.91 55.82-22.51 4.61-41.94 18.7-53.31 38.67-17.59 30.32-13.58 68.54 9.92 94.54-7.26 21.79-4.76 45.66 6.85 65.48 17.46 30.4 52.56 46.04 86.84 38.68 15.24 17.18 37.16 26.95 60.13 26.8 35.06.09 66.16-22.49 76.94-55.86 22.51-4.61 41.94-18.7 53.31-38.67 17.57-30.32 13.55-68.51-9.94-94.51zm-120.28 168.11c-14.03.02-27.62-4.89-38.39-13.88.49-.26 1.34-.73 1.89-1.07l63.72-36.8c3.26-1.85 5.26-5.32 5.24-9.07v-89.83l26.93 15.55c.29.14.48.42.52.74v74.39c-.04 33.08-26.83 59.9-59.91 59.97zm-128.84-55.03c-7.03-12.14-9.56-26.37-7.15-40.18.47.28 1.3.79 1.89 1.13l63.72 36.8c3.23 1.89 7.23 1.89 10.47 0l77.79-44.92v31.1c.02.32-.13.63-.38.83l-64.41 37.19c-28.69 16.52-65.33 6.7-81.92-21.95zm-16.77-139.09c7-12.16 18.05-21.46 31.21-26.29 0 .55-.03 1.52-.03 2.2v73.61c-.02 3.74 1.98 7.21 5.23 9.06l77.79 44.91-26.93 15.55c-.27.18-.61.21-.91.08l-64.42-37.22c-28.63-16.58-38.45-53.21-21.95-81.89zm221.26 51.49-77.79-44.92 26.93-15.54c.27-.18.61-.21.91-.08l64.42 37.19c28.68 16.57 38.51 53.26 21.94 81.94-7.01 12.14-18.05 21.44-31.2 26.28v-75.81c.03-3.74-1.96-7.2-5.2-9.06zm26.8-40.34c-.47-.29-1.3-.79-1.89-1.13l-63.72-36.8c-3.23-1.89-7.23-1.89-10.47 0l-77.79 44.92v-31.1c-.02-.32.13-.63.38-.83l64.41-37.16c28.69-16.55 65.37-6.7 81.91 22 6.99 12.12 9.52 26.31 7.15 40.1zm-168.51 55.43-26.94-15.55c-.29-.14-.48-.42-.52-.74v-74.39c.02-33.12 26.89-59.96 60.01-59.94 14.01 0 27.57 4.92 38.34 13.88-.49.26-1.33.73-1.89 1.07l-63.72 36.8c-3.26 1.85-5.26 5.31-5.24 9.06l-.04 89.79zm14.63-31.54 34.65-20.01 34.65 20v40.01l-34.65 20-34.65-20z\"/></svg>",
      "variables": {},
      "input": {
        "method": "GET",
        "url": "http://example.com",
        "headers": "",
        "body": ""
      },
      "input_schema": {
        "type": "object",
        "properties": {
          "method": {
             "title": "Method",
              "description": "HTTP Method for request",
              "type": "string",
              "oneOf": [
                {
                  "value": "GET",
                  "title": "GET"
                },
                {
                  "value": "POST",
                  "title": "POST"
                },
                {
                  "value": "PUT",
                  "title": "PUT"
                },
                {
                  "value": "DELETE",
                  "title": "DELETE"
                }
              ],
              "x-jsf-presentation": {
                "inputType": "select"
              }
          },
          "url": {
             "title": "URL",
            "description": "URL for request",
            "type": "string"
          },
          "headers": {
            "title": "Headers",
            "description": "Headers for request",
            "type": "string"
          },
          "body": {
            "title": "Body",
            "description": "Body for request",
            "type": "string"
          }
        },
        "x-jsf-order": ["url", "method", "headers", "body"],
        "required": ["method", "url"],
        "additionalProperties": false
      },
      "presentation": {
        "position": {
          "x": 300,
          "y": 300
        }
      },
      "handles": [
        {
          "id": "a", 
          "type": "target",
          "position": "top"
        },
        {
          "id": "b",
          "type": "source",
          "position": "bottom"
        }
      ]
    },
    {
      "anything_action_version": "1.0.0",
      "type": "action",
      "plugin_id": "example_plugin",
      "node_id": "example_action_2",
      "plugin_version": "1.0.0",
      "label": "Example Action 2",
      "description": "This is the second example action",
      "icon": "<svg xmlns:xodm=\"http://www.corel.com/coreldraw/odm/2003\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" version=\"1.1\" id=\"Layer_1\" x=\"0px\" y=\"0px\" viewBox=\"0 0 2500 2503\" style=\"enable-background:new 0 0 2500 2503;\" xml:space=\"preserve\">\n<style type=\"text/css\">\n\t.st0{fill:none;}\n\t.st1{fill:#F0CDC2;stroke:#1616B4;stroke-width:3.1298;stroke-linejoin:round;stroke-miterlimit:22.9245;}\n\t.st2{fill:#C9B3F5;stroke:#1616B4;stroke-width:3.1298;stroke-linejoin:round;stroke-miterlimit:22.9245;}\n\t.st3{fill:#88AAF1;stroke:#1616B4;stroke-width:3.1298;stroke-linejoin:round;stroke-miterlimit:22.9245;}\n\t.st4{fill:#B8FAF6;stroke:#1616B4;stroke-width:3.1298;stroke-linejoin:round;stroke-miterlimit:22.9245;}\n</style>\n<g id=\"Layer_x0020_1\">\n\t<rect x=\"0.1\" y=\"1.6\" class=\"st0\" width=\"2499.9\" height=\"2499.9\"></rect>\n\t<g id=\"_2082587881456\">\n\t\t<polygon class=\"st1\" points=\"1248.7,2501.4 1248.7,1874.2 472.8,1420.5   \"></polygon>\n\t\t<polygon class=\"st2\" points=\"1251.3,2501.4 1251.3,1874.2 2027.1,1420.5   \"></polygon>\n\t\t<polygon class=\"st3\" points=\"1248.7,1718.4 1248.7,917.9 464,1269.3   \"></polygon>\n\t\t<polygon class=\"st2\" points=\"1251.3,1718.4 1251.3,917.9 2036,1269.3   \"></polygon>\n\t\t<polygon class=\"st1\" points=\"464,1269.3 1248.7,1.6 1248.7,917.9   \"></polygon>\n\t\t<polygon class=\"st4\" points=\"2036,1269.3 1251.3,1.6 1251.3,917.9   \"></polygon>\n\t</g>\n</g>\n</svg>",
      "variables": {},
      "input": {
        "method": "POST",
        "url": "http://example.com",
        "headers": "",
        "body": ""
      },
      "input_schema": {
        "type": "object",
        "properties": {
          "method": {
            "type": "string",
            "enum": ["GET", "POST", "PUT", "DELETE"]
          },
          "url": {
            "type": "string"
          },
          "headers": {
            "type": "string"
          },
          "body": {
            "type": "string"
          }
        },
        "required": ["method", "url"],
        "additionalProperties": false
      },
      "presentation": {
        "position": {
          "x": 300,
          "y": 500
        }
      },
      "handles": [
        {
          "id": "a", 
          "type": "target",
          "position": "top"
        },
        {
          "id": "b",
          "type": "source",
          "position": "bottom"
        }
      ]
    },
    {
      "anything_action_version": "1.0.0",
      "type": "action",
      "plugin_id": "other_example_plugin",
      "node_id": "other_example_action",
      "plugin_version": "1.0.0",
      "label": "Example Action 3",
      "description": "This is the third example action",
      "icon": "<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" viewBox=\"0.00 0.00 1024.00 1024.00\">\n<path fill=\"none\" d=\" \n  M 0.00 0.00 \n  L 1024.00 0.00\n  L 1024.00 1024.00\n  L 0.00 1024.00\n  L 0.00 0.00\n  Z\n  M 544.53 291.36\n  L 514.04 291.29\n  A 0.89 0.89 0.0 0 1 513.28 290.86\n  C 503.27 274.77 492.95 258.85 483.09 242.71\n  Q 451.44 190.92 418.99 139.61\n  C 414.92 133.17 410.92 125.28 402.26 125.83\n  C 394.88 126.31 389.53 132.50 386.81 139.04\n  C 363.54 194.95 374.90 259.13 396.96 313.20\n  Q 397.16 313.68 396.78 314.03\n  C 369.88 338.98 358.72 371.19 357.81 407.33\n  Q 357.79 407.84 357.37 408.12\n  Q 332.96 424.39 308.38 440.40\n  C 289.89 452.44 270.95 463.40 264.29 486.21\n  Q 256.87 511.57 276.81 531.52\n  Q 277.17 531.88 277.17 532.39\n  Q 277.14 550.31 277.08 568.22\n  C 277.03 582.05 277.80 591.67 284.08 602.43\n  C 299.76 629.28 330.05 636.97 359.08 631.14\n  Q 371.03 628.75 381.44 621.32\n  Q 381.84 621.03 382.27 621.29\n  Q 394.70 628.94 408.97 632.17\n  A 0.52 0.52 0.0 0 1 409.37 632.75\n  Q 395.08 736.01 380.36 838.75\n  Q 380.07 840.76 382.28 842.75\n  Q 417.48 874.48 460.23 889.16\n  C 500.49 902.98 544.08 903.97 585.14 893.12\n  C 592.22 891.25 599.98 888.52 607.38 886.04\n  Q 616.55 882.97 623.65 879.59\n  Q 664.48 860.16 697.91 830.12\n  A 2.38 2.38 0.0 0 0 698.71 828.34\n  Q 698.70 664.99 698.57 502.00\n  Q 698.57 501.95 698.64 493.49\n  C 699.00 452.17 688.48 411.86 665.26 377.44\n  Q 658.46 367.36 651.02 358.22\n  Q 650.55 357.64 650.79 356.93\n  C 668.11 306.20 667.74 253.83 645.17 204.88\n  Q 636.43 185.92 620.58 166.44\n  Q 599.71 140.81 571.70 128.31\n  C 563.74 124.76 554.58 122.40 546.05 124.93\n  C 537.36 127.51 530.84 134.43 529.35 143.45\n  Q 528.66 147.62 530.15 156.30\n  C 537.79 200.66 544.76 245.44 545.18 290.70\n  A 0.65 0.65 0.0 0 1 544.53 291.36\n  Z\"\n/>\n<path fill=\"#012630\" d=\"\n  M 544.53 291.36\n  A 0.65 0.65 0.0 0 0 545.18 290.70\n  C 544.76 245.44 537.79 200.66 530.15 156.30\n  Q 528.66 147.62 529.35 143.45\n  C 530.84 134.43 537.36 127.51 546.05 124.93\n  C 554.58 122.40 563.74 124.76 571.70 128.31\n  Q 599.71 140.81 620.58 166.44\n  Q 636.43 185.92 645.17 204.88\n  C 667.74 253.83 668.11 306.20 650.79 356.93\n  Q 650.55 357.64 651.02 358.22\n  Q 658.46 367.36 665.26 377.44\n  C 688.48 411.86 699.00 452.17 698.64 493.49\n  Q 698.57 501.95 698.57 502.00\n  Q 698.70 664.99 698.71 828.34\n  A 2.38 2.38 0.0 0 1 697.91 830.12\n  Q 664.48 860.16 623.65 879.59\n  Q 616.55 882.97 607.38 886.04\n  C 599.98 888.52 592.22 891.25 585.14 893.12\n  C 544.08 903.97 500.49 902.98 460.23 889.16\n  Q 417.48 874.48 382.28 842.75\n  Q 380.07 840.76 380.36 838.75\n  Q 395.08 736.01 409.37 632.75\n  A 0.52 0.52 0.0 0 0 408.97 632.17\n  Q 394.70 628.94 382.27 621.29\n  Q 381.84 621.03 381.44 621.32\n  Q 371.03 628.75 359.08 631.14\n  C 330.05 636.97 299.76 629.28 284.08 602.43\n  C 277.80 591.67 277.03 582.05 277.08 568.22\n  Q 277.14 550.31 277.17 532.39\n  Q 277.17 531.88 276.81 531.52\n  Q 256.87 511.57 264.29 486.21\n  C 270.95 463.40 289.89 452.44 308.38 440.40\n  Q 332.96 424.39 357.37 408.12\n  Q 357.79 407.84 357.81 407.33\n  C 358.72 371.19 369.88 338.98 396.78 314.03\n  Q 397.16 313.68 396.96 313.20\n  C 374.90 259.13 363.54 194.95 386.81 139.04\n  C 389.53 132.50 394.88 126.31 402.26 125.83\n  C 410.92 125.28 414.92 133.17 418.99 139.61\n  Q 451.44 190.92 483.09 242.71\n  C 492.95 258.85 503.27 274.77 513.28 290.86\n  A 0.89 0.89 0.0 0 0 514.04 291.29\n  L 544.53 291.36\n  Z\n  M 498.48 329.35\n  Q 482.61 329.24 466.73 329.25\n  Q 453.25 329.27 446.83 330.55\n  C 416.92 336.54 397.62 364.01 396.03 393.65\n  C 395.63 401.03 395.72 415.33 395.89 426.10\n  Q 354.30 454.50 311.89 481.71\n  Q 307.52 484.51 304.77 487.61\n  Q 300.73 492.14 297.40 497.37\n  Q 297.12 497.80 297.47 498.18\n  L 308.75 510.38\n  A 1.46 1.45 51.1 0 0 310.64 510.59\n  Q 321.93 502.83 334.60 497.11\n  Q 340.26 494.56 343.68 494.42\n  C 352.51 494.04 359.46 500.96 361.62 509.29\n  C 366.46 527.94 344.44 533.75 331.46 537.89\n  Q 324.88 539.98 313.18 546.11\n  Q 312.73 546.35 312.73 546.85\n  Q 312.74 560.30 312.89 573.76\n  Q 312.92 577.24 314.71 580.77\n  C 320.99 593.20 330.24 594.55 342.90 594.87\n  C 351.06 595.08 359.46 594.41 366.07 589.54\n  Q 368.41 587.82 374.72 581.23\n  Q 381.78 573.87 388.02 565.76\n  A 3.31 3.31 0.0 0 0 388.70 563.75\n  C 388.78 553.29 386.62 540.67 395.76 533.02\n  C 405.27 525.08 418.74 530.17 422.93 541.00\n  Q 424.35 544.66 424.35 552.70\n  Q 424.35 567.22 424.46 581.75\n  C 424.49 585.01 416.22 591.86 413.76 594.11\n  A 0.37 0.37 0.0 0 0 414.01 594.75\n  Q 451.31 595.19 488.75 594.99\n  Q 502.73 594.92 508.69 593.91\n  C 534.80 589.50 556.34 569.48 562.04 543.31\n  C 563.42 536.96 562.98 529.73 563.02 522.90\n  Q 563.06 515.66 564.00 512.70\n  C 569.29 496.02 591.82 495.65 598.02 512.02\n  Q 599.51 515.95 599.52 525.21\n  C 599.54 540.38 599.09 552.99 593.04 567.55\n  C 591.24 571.88 589.65 576.39 587.36 580.30\n  C 580.58 591.89 572.21 601.59 561.86 610.13\n  Q 558.81 612.65 556.38 615.26\n  Q 546.81 620.87 542.11 622.83\n  C 530.69 627.61 514.87 632.91 501.76 632.77\n  Q 473.94 632.47 446.27 632.53\n  Q 445.73 632.53 445.66 633.06\n  L 429.75 748.41\n  L 419.34 823.76\n  A 2.21 2.20 -66.6 0 0 420.13 825.77\n  Q 457.43 856.25 506.30 862.08\n  Q 559.10 868.76 606.29 847.04\n  Q 635.26 833.71 662.51 811.24\n  Q 662.86 810.95 662.86 810.50\n  Q 663.37 654.88 662.77 499.26\n  Q 662.72 486.04 662.14 479.27\n  Q 659.21 444.88 643.88 413.71\n  Q 641.69 409.25 638.37 405.30\n  Q 637.73 404.55 637.87 405.53\n  Q 639.71 418.11 631.02 425.14\n  C 621.91 432.50 609.17 429.32 604.37 418.80\n  Q 603.03 415.86 603.01 410.27\n  Q 602.97 390.62 602.90 370.98\n  Q 602.89 370.49 603.15 370.06\n  Q 634.73 316.60 623.34 256.18\n  Q 613.32 203.04 569.79 169.06\n  Q 569.22 168.61 569.26 169.34\n  Q 569.52 174.19 570.65 180.00\n  C 577.59 215.96 581.55 253.35 581.71 290.50\n  Q 581.84 318.25 581.78 346.00\n  C 581.74 365.46 554.87 370.52 547.23 352.49\n  Q 545.97 349.51 545.94 344.36\n  Q 545.89 337.11 545.87 329.84\n  Q 545.86 329.27 545.29 329.27\n  L 498.48 329.35\n  Z\n  M 409.16 196.24\n  C 408.18 229.52 415.80 263.93 429.39 294.50\n  A 1.41 1.40 69.8 0 0 431.07 295.28\n  Q 436.93 293.55 442.96 292.59\n  C 451.31 291.26 460.90 291.60 469.69 291.80\n  A 0.68 0.67 -16.6 0 0 470.27 290.74\n  Q 459.08 274.61 448.95 257.81\n  Q 430.55 227.33 410.64 195.83\n  Q 409.23 193.60 409.16 196.24\n  Z\"\n/>\n<path fill=\"#fefdf7\" d=\"\n  M 506.30 862.08\n  Q 506.44 862.11 506.61 862.08\n  Q 506.96 862.02 506.96 861.66\n  L 506.64 667.39\n  Q 532.01 642.28 556.38 615.26\n  Q 558.81 612.65 561.86 610.13\n  C 572.21 601.59 580.58 591.89 587.36 580.30\n  C 589.65 576.39 591.24 571.88 593.04 567.55\n  C 599.09 552.99 599.54 540.38 599.52 525.21\n  Q 599.51 515.95 598.02 512.02\n  C 591.82 495.65 569.29 496.02 564.00 512.70\n  Q 563.06 515.66 563.02 522.90\n  C 562.98 529.73 563.42 536.96 562.04 543.31\n  C 556.34 569.48 534.80 589.50 508.69 593.91\n  Q 502.73 594.92 488.75 594.99\n  Q 451.31 595.19 414.01 594.75\n  A 0.37 0.37 0.0 0 1 413.76 594.11\n  C 416.22 591.86 424.49 585.01 424.46 581.75\n  Q 424.35 567.22 424.35 552.70\n  Q 424.35 544.66 422.93 541.00\n  C 418.74 530.17 405.27 525.08 395.76 533.02\n  C 386.62 540.67 388.78 553.29 388.70 563.75\n  A 3.31 3.31 0.0 0 1 388.02 565.76\n  Q 381.78 573.87 374.72 581.23\n  Q 368.41 587.82 366.07 589.54\n  C 359.46 594.41 351.06 595.08 342.90 594.87\n  C 330.24 594.55 320.99 593.20 314.71 580.77\n  Q 312.92 577.24 312.89 573.76\n  Q 312.74 560.30 312.73 546.85\n  Q 312.73 546.35 313.18 546.11\n  Q 324.88 539.98 331.46 537.89\n  C 344.44 533.75 366.46 527.94 361.62 509.29\n  C 359.46 500.96 352.51 494.04 343.68 494.42\n  Q 340.26 494.56 334.60 497.11\n  Q 321.93 502.83 310.64 510.59\n  A 1.46 1.45 51.1 0 1 308.75 510.38\n  L 297.47 498.18\n  Q 297.12 497.80 297.40 497.37\n  Q 300.73 492.14 304.77 487.61\n  Q 307.52 484.51 311.89 481.71\n  Q 354.30 454.50 395.89 426.10\n  C 396.51 425.87 397.47 425.60 398.01 425.20\n  Q 413.92 413.63 429.41 401.92\n  A 1.44 1.43 -27.7 0 0 429.91 400.33\n  Q 425.74 387.75 433.29 378.42\n  C 436.89 373.98 444.19 369.27 448.55 365.30\n  Q 465.37 349.98 482.25 334.74\n  Q 484.26 332.93 485.58 330.73\n  A 2.33 2.32 -75.2 0 1 487.52 329.60\n  L 498.48 329.35\n  L 545.29 329.27\n  Q 545.86 329.27 545.87 329.84\n  Q 545.89 337.11 545.94 344.36\n  Q 545.97 349.51 547.23 352.49\n  C 554.87 370.52 581.74 365.46 581.78 346.00\n  Q 581.84 318.25 581.71 290.50\n  C 581.55 253.35 577.59 215.96 570.65 180.00\n  Q 569.52 174.19 569.26 169.34\n  Q 569.22 168.61 569.79 169.06\n  Q 613.32 203.04 623.34 256.18\n  Q 634.73 316.60 603.15 370.06\n  Q 602.89 370.49 602.90 370.98\n  Q 602.97 390.62 603.01 410.27\n  Q 603.03 415.86 604.37 418.80\n  C 609.17 429.32 621.91 432.50 631.02 425.14\n  Q 639.71 418.11 637.87 405.53\n  Q 637.73 404.55 638.37 405.30\n  Q 641.69 409.25 643.88 413.71\n  Q 659.21 444.88 662.14 479.27\n  Q 662.72 486.04 662.77 499.26\n  Q 663.37 654.88 662.86 810.50\n  Q 662.86 810.95 662.51 811.24\n  Q 635.26 833.71 606.29 847.04\n  Q 559.10 868.76 506.30 862.08\n  Z\n  M 487.09 448.99\n  C 488.00 450.86 488.88 453.02 489.81 454.47\n  C 501.58 472.76 525.60 473.20 538.05 454.99\n  Q 544.25 445.93 542.41 436.29\n  C 539.29 420.03 525.95 409.27 509.51 410.21\n  C 490.01 411.33 470.69 431.75 461.77 447.67\n  A 0.51 0.51 0.0 0 0 462.22 448.43\n  L 486.22 448.44\n  A 0.96 0.96 0.0 0 1 487.09 448.99\n  Z\"\n/>\n<path fill=\"#456e77\" d=\"\n  M 429.39 294.50\n  C 415.80 263.93 408.18 229.52 409.16 196.24\n  Q 409.23 193.60 410.64 195.83\n  Q 430.55 227.33 448.95 257.81\n  Q 459.08 274.61 470.27 290.74\n  A 0.68 0.67 -16.6 0 1 469.69 291.80\n  C 460.90 291.60 451.31 291.26 442.96 292.59\n  Q 436.93 293.55 431.07 295.28\n  A 1.41 1.40 69.8 0 1 429.39 294.50\n  Z\"\n/>\n<path fill=\"#9fd4d7\" d=\"\n  M 498.48 329.35\n  L 487.52 329.60\n  A 2.33 2.32 -75.2 0 0 485.58 330.73\n  Q 484.26 332.93 482.25 334.74\n  Q 465.37 349.98 448.55 365.30\n  C 444.19 369.27 436.89 373.98 433.29 378.42\n  Q 425.74 387.75 429.91 400.33\n  A 1.44 1.43 -27.7 0 1 429.41 401.92\n  Q 413.92 413.63 398.01 425.20\n  C 397.47 425.60 396.51 425.87 395.89 426.10\n  C 395.72 415.33 395.63 401.03 396.03 393.65\n  C 397.62 364.01 416.92 336.54 446.83 330.55\n  Q 453.25 329.27 466.73 329.25\n  Q 482.61 329.24 498.48 329.35\n  Z\"\n/>\n<path fill=\"#012630\" d=\"\n  M 486.22 448.44\n  L 462.22 448.43\n  A 0.51 0.51 0.0 0 1 461.77 447.67\n  C 470.69 431.75 490.01 411.33 509.51 410.21\n  C 525.95 409.27 539.29 420.03 542.41 436.29\n  Q 544.25 445.93 538.05 454.99\n  C 525.60 473.20 501.58 472.76 489.81 454.47\n  C 488.88 453.02 488.00 450.86 487.09 448.99\n  A 0.96 0.96 0.0 0 0 486.22 448.44\n  Z\"\n/>\n<path fill=\"#456e77\" d=\"\n  M 556.38 615.26\n  Q 532.01 642.28 506.64 667.39\n  L 429.75 748.41\n  L 445.66 633.06\n  Q 445.73 632.53 446.27 632.53\n  Q 473.94 632.47 501.76 632.77\n  C 514.87 632.91 530.69 627.61 542.11 622.83\n  Q 546.81 620.87 556.38 615.26\n  Z\"\n/>\n<path fill=\"#9fd4d7\" d=\"\n  M 506.64 667.39\n  L 506.96 861.66\n  Q 506.96 862.02 506.61 862.08\n  Q 506.44 862.11 506.30 862.08\n  Q 457.43 856.25 420.13 825.77\n  A 2.21 2.20 -66.6 0 1 419.34 823.76\n  L 429.75 748.41\n  L 506.64 667.39\n  Z\"\n/> \n</svg>",
      "variables": {},
      "input": {
        "method": "PUT",
        "url": "http://example.com",
        "headers": "",
        "body": ""
      },
      "input_schema": {
        "type": "object",
        "properties": {
          "method": {
            "type": "string",
            "enum": ["GET", "POST", "PUT", "DELETE"]
          },
          "url": {
            "type": "string"
          },
          "headers": {
            "type": "string"
          },
          "body": {
            "type": "string"
          }
        },
        "required": ["method", "url"],
        "additionalProperties": false
      },
      "presentation": {
        "position": {
          "x": 300,
          "y": 700
        }
      },
      "handles": [
        {
          "id": "a", 
          "type": "target",
          "position": "top"
        },
        {
          "id": "b",
          "type": "source",
          "position": "bottom"
        }
      ]
    },
     {
      "anything_action_version": "1.0.0",
      "type": "action",
      "plugin_id": "anything_output",
      "node_id": "anything_output", 
      "plugin_version": "1.0.0",
      "label": "Outputs",
      "description": "Outputs for the flow",
      "icon": "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"24\" height=\"24\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\" class=\"lucide lucide-braces\"><path d=\"M8 3H7a2 2 0 0 0-2 2v5a2 2 0 0 1-2 2 2 2 0 0 1 2 2v5c0 1.1.9 2 2 2h1\"/><path d=\"M16 21h1a2 2 0 0 0 2-2v-5c0-1.1.9-2 2-2a2 2 0 0 1-2-2V5a2 2 0 0 0-2-2h-1\"/></svg>",
      "variables": {},
      "input": {},
      "input_schema": {},
      "presentation": {
        "position": {
          "x": 300,
          "y": 900
        }
      },
      "handles": [
        {
          "id": "a", 
          "type": "target",
          "position": "top"
        }
      ]
    }
  ]
}', now(), now(), '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8', '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8'),
    ('22222222-2222-2222-2222-222222222222', '7df12345-a5d3-4b13-9e3a-2f5c3e6a7b91', 'bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb', 'v1.0', 'Initial version of Flow 2', 'checksum2', true,  '{
    "actions": [
    {
      "anything_action_version": "1.0.0",
      "type": "action",
      "plugin_id": "example_plugin",
      "node_id": "example_action",
      "plugin_version": "1.0.0",
      "label": "Example Action 1",
      "description": "This is the first example action",
      "icon": "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 320 320\"><path d=\"m297.06 130.97c7.26-21.79 4.76-45.66-6.85-65.48-17.46-30.4-52.56-46.04-86.84-38.68-15.25-17.18-37.16-26.95-60.13-26.81-35.04-.08-66.13 22.48-76.91 55.82-22.51 4.61-41.94 18.7-53.31 38.67-17.59 30.32-13.58 68.54 9.92 94.54-7.26 21.79-4.76 45.66 6.85 65.48 17.46 30.4 52.56 46.04 86.84 38.68 15.24 17.18 37.16 26.95 60.13 26.8 35.06.09 66.16-22.49 76.94-55.86 22.51-4.61 41.94-18.7 53.31-38.67 17.57-30.32 13.55-68.51-9.94-94.51zm-120.28 168.11c-14.03.02-27.62-4.89-38.39-13.88.49-.26 1.34-.73 1.89-1.07l63.72-36.8c3.26-1.85 5.26-5.32 5.24-9.07v-89.83l26.93 15.55c.29.14.48.42.52.74v74.39c-.04 33.08-26.83 59.9-59.91 59.97zm-128.84-55.03c-7.03-12.14-9.56-26.37-7.15-40.18.47.28 1.3.79 1.89 1.13l63.72 36.8c3.23 1.89 7.23 1.89 10.47 0l77.79-44.92v31.1c.02.32-.13.63-.38.83l-64.41 37.19c-28.69 16.52-65.33 6.7-81.92-21.95zm-16.77-139.09c7-12.16 18.05-21.46 31.21-26.29 0 .55-.03 1.52-.03 2.2v73.61c-.02 3.74 1.98 7.21 5.23 9.06l77.79 44.91-26.93 15.55c-.27.18-.61.21-.91.08l-64.42-37.22c-28.63-16.58-38.45-53.21-21.95-81.89zm221.26 51.49-77.79-44.92 26.93-15.54c.27-.18.61-.21.91-.08l64.42 37.19c28.68 16.57 38.51 53.26 21.94 81.94-7.01 12.14-18.05 21.44-31.2 26.28v-75.81c.03-3.74-1.96-7.2-5.2-9.06zm26.8-40.34c-.47-.29-1.3-.79-1.89-1.13l-63.72-36.8c-3.23-1.89-7.23-1.89-10.47 0l-77.79 44.92v-31.1c-.02-.32.13-.63.38-.83l64.41-37.16c28.69-16.55 65.37-6.7 81.91 22 6.99 12.12 9.52 26.31 7.15 40.1zm-168.51 55.43-26.94-15.55c-.29-.14-.48-.42-.52-.74v-74.39c.02-33.12 26.89-59.96 60.01-59.94 14.01 0 27.57 4.92 38.34 13.88-.49.26-1.33.73-1.89 1.07l-63.72 36.8c-3.26 1.85-5.26 5.31-5.24 9.06l-.04 89.79zm14.63-31.54 34.65-20.01 34.65 20v40.01l-34.65 20-34.65-20z\"/></svg>",
      "variables": {},
      "input": {
        "method": "GET",
        "url": "http://example.com",
        "headers": "",
        "body": ""
      },
      "input_schema": {
        "type": "object",
        "properties": {
          "method": {
            "type": "string",
            "enum": ["GET", "POST", "PUT", "DELETE"]
          },
          "url": {
            "type": "string"
          },
          "headers": {
            "type": "string"
          },
          "body": {
            "type": "string"
          }
        },
        "required": ["method", "url"],
        "additionalProperties": false
      },
      "presentation": {
        "position": {
          "x": 300,
          "y": 100
        }
      },
      "handles": [
        {
          "id": "a", 
          "type": "target",
          "position": "top"
        },
        {
          "id": "b",
          "type": "source",
          "position": "bottom"
        }
      ]
    },
    {
      "anything_action_version": "1.0.0",
      "type": "action",
      "plugin_id": "example_plugin",
      "node_id": "example_action_2",
      "plugin_version": "1.0.0",
      "label": "Example Action 2",
      "description": "This is the second example action",
      "icon": "<svg xmlns:xodm=\"http://www.corel.com/coreldraw/odm/2003\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" version=\"1.1\" id=\"Layer_1\" x=\"0px\" y=\"0px\" viewBox=\"0 0 2500 2503\" style=\"enable-background:new 0 0 2500 2503;\" xml:space=\"preserve\">\n<style type=\"text/css\">\n\t.st0{fill:none;}\n\t.st1{fill:#F0CDC2;stroke:#1616B4;stroke-width:3.1298;stroke-linejoin:round;stroke-miterlimit:22.9245;}\n\t.st2{fill:#C9B3F5;stroke:#1616B4;stroke-width:3.1298;stroke-linejoin:round;stroke-miterlimit:22.9245;}\n\t.st3{fill:#88AAF1;stroke:#1616B4;stroke-width:3.1298;stroke-linejoin:round;stroke-miterlimit:22.9245;}\n\t.st4{fill:#B8FAF6;stroke:#1616B4;stroke-width:3.1298;stroke-linejoin:round;stroke-miterlimit:22.9245;}\n</style>\n<g id=\"Layer_x0020_1\">\n\t<rect x=\"0.1\" y=\"1.6\" class=\"st0\" width=\"2499.9\" height=\"2499.9\"></rect>\n\t<g id=\"_2082587881456\">\n\t\t<polygon class=\"st1\" points=\"1248.7,2501.4 1248.7,1874.2 472.8,1420.5   \"></polygon>\n\t\t<polygon class=\"st2\" points=\"1251.3,2501.4 1251.3,1874.2 2027.1,1420.5   \"></polygon>\n\t\t<polygon class=\"st3\" points=\"1248.7,1718.4 1248.7,917.9 464,1269.3   \"></polygon>\n\t\t<polygon class=\"st2\" points=\"1251.3,1718.4 1251.3,917.9 2036,1269.3   \"></polygon>\n\t\t<polygon class=\"st1\" points=\"464,1269.3 1248.7,1.6 1248.7,917.9   \"></polygon>\n\t\t<polygon class=\"st4\" points=\"2036,1269.3 1251.3,1.6 1251.3,917.9   \"></polygon>\n\t</g>\n</g>\n</svg>",
      "variables": {},
      "input": {
        "method": "POST",
        "url": "http://example.com",
        "headers": "",
        "body": ""
      },
      "input_schema": {
        "type": "object",
        "properties": {
          "method": {
            "type": "string",
            "enum": ["GET", "POST", "PUT", "DELETE"]
          },
          "url": {
            "type": "string"
          },
          "headers": {
            "type": "string"
          },
          "body": {
            "type": "string"
          }
        },
        "required": ["method", "url"],
        "additionalProperties": false
      },
      "presentation": {
        "position": {
          "x": 300,
          "y": 300
        }
      },
      "handles": [
        {
          "id": "a", 
          "type": "target",
          "position": "top"
        },
        {
          "id": "b",
          "type": "source",
          "position": "bottom"
        }
      ]
    },
    {
      "anything_action_version": "1.0.0",
      "type": "action",
      "plugin_id": "other_example_plugin",
      "node_id": "other_example_action",
      "plugin_version": "1.0.0",
      "label": "Example Action 3",
      "description": "This is the third example action",
      "icon": "<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" viewBox=\"0.00 0.00 1024.00 1024.00\">\n<path fill=\"none\" d=\" \n  M 0.00 0.00 \n  L 1024.00 0.00\n  L 1024.00 1024.00\n  L 0.00 1024.00\n  L 0.00 0.00\n  Z\n  M 544.53 291.36\n  L 514.04 291.29\n  A 0.89 0.89 0.0 0 1 513.28 290.86\n  C 503.27 274.77 492.95 258.85 483.09 242.71\n  Q 451.44 190.92 418.99 139.61\n  C 414.92 133.17 410.92 125.28 402.26 125.83\n  C 394.88 126.31 389.53 132.50 386.81 139.04\n  C 363.54 194.95 374.90 259.13 396.96 313.20\n  Q 397.16 313.68 396.78 314.03\n  C 369.88 338.98 358.72 371.19 357.81 407.33\n  Q 357.79 407.84 357.37 408.12\n  Q 332.96 424.39 308.38 440.40\n  C 289.89 452.44 270.95 463.40 264.29 486.21\n  Q 256.87 511.57 276.81 531.52\n  Q 277.17 531.88 277.17 532.39\n  Q 277.14 550.31 277.08 568.22\n  C 277.03 582.05 277.80 591.67 284.08 602.43\n  C 299.76 629.28 330.05 636.97 359.08 631.14\n  Q 371.03 628.75 381.44 621.32\n  Q 381.84 621.03 382.27 621.29\n  Q 394.70 628.94 408.97 632.17\n  A 0.52 0.52 0.0 0 1 409.37 632.75\n  Q 395.08 736.01 380.36 838.75\n  Q 380.07 840.76 382.28 842.75\n  Q 417.48 874.48 460.23 889.16\n  C 500.49 902.98 544.08 903.97 585.14 893.12\n  C 592.22 891.25 599.98 888.52 607.38 886.04\n  Q 616.55 882.97 623.65 879.59\n  Q 664.48 860.16 697.91 830.12\n  A 2.38 2.38 0.0 0 0 698.71 828.34\n  Q 698.70 664.99 698.57 502.00\n  Q 698.57 501.95 698.64 493.49\n  C 699.00 452.17 688.48 411.86 665.26 377.44\n  Q 658.46 367.36 651.02 358.22\n  Q 650.55 357.64 650.79 356.93\n  C 668.11 306.20 667.74 253.83 645.17 204.88\n  Q 636.43 185.92 620.58 166.44\n  Q 599.71 140.81 571.70 128.31\n  C 563.74 124.76 554.58 122.40 546.05 124.93\n  C 537.36 127.51 530.84 134.43 529.35 143.45\n  Q 528.66 147.62 530.15 156.30\n  C 537.79 200.66 544.76 245.44 545.18 290.70\n  A 0.65 0.65 0.0 0 1 544.53 291.36\n  Z\"\n/>\n<path fill=\"#012630\" d=\"\n  M 544.53 291.36\n  A 0.65 0.65 0.0 0 0 545.18 290.70\n  C 544.76 245.44 537.79 200.66 530.15 156.30\n  Q 528.66 147.62 529.35 143.45\n  C 530.84 134.43 537.36 127.51 546.05 124.93\n  C 554.58 122.40 563.74 124.76 571.70 128.31\n  Q 599.71 140.81 620.58 166.44\n  Q 636.43 185.92 645.17 204.88\n  C 667.74 253.83 668.11 306.20 650.79 356.93\n  Q 650.55 357.64 651.02 358.22\n  Q 658.46 367.36 665.26 377.44\n  C 688.48 411.86 699.00 452.17 698.64 493.49\n  Q 698.57 501.95 698.57 502.00\n  Q 698.70 664.99 698.71 828.34\n  A 2.38 2.38 0.0 0 1 697.91 830.12\n  Q 664.48 860.16 623.65 879.59\n  Q 616.55 882.97 607.38 886.04\n  C 599.98 888.52 592.22 891.25 585.14 893.12\n  C 544.08 903.97 500.49 902.98 460.23 889.16\n  Q 417.48 874.48 382.28 842.75\n  Q 380.07 840.76 380.36 838.75\n  Q 395.08 736.01 409.37 632.75\n  A 0.52 0.52 0.0 0 0 408.97 632.17\n  Q 394.70 628.94 382.27 621.29\n  Q 381.84 621.03 381.44 621.32\n  Q 371.03 628.75 359.08 631.14\n  C 330.05 636.97 299.76 629.28 284.08 602.43\n  C 277.80 591.67 277.03 582.05 277.08 568.22\n  Q 277.14 550.31 277.17 532.39\n  Q 277.17 531.88 276.81 531.52\n  Q 256.87 511.57 264.29 486.21\n  C 270.95 463.40 289.89 452.44 308.38 440.40\n  Q 332.96 424.39 357.37 408.12\n  Q 357.79 407.84 357.81 407.33\n  C 358.72 371.19 369.88 338.98 396.78 314.03\n  Q 397.16 313.68 396.96 313.20\n  C 374.90 259.13 363.54 194.95 386.81 139.04\n  C 389.53 132.50 394.88 126.31 402.26 125.83\n  C 410.92 125.28 414.92 133.17 418.99 139.61\n  Q 451.44 190.92 483.09 242.71\n  C 492.95 258.85 503.27 274.77 513.28 290.86\n  A 0.89 0.89 0.0 0 0 514.04 291.29\n  L 544.53 291.36\n  Z\n  M 498.48 329.35\n  Q 482.61 329.24 466.73 329.25\n  Q 453.25 329.27 446.83 330.55\n  C 416.92 336.54 397.62 364.01 396.03 393.65\n  C 395.63 401.03 395.72 415.33 395.89 426.10\n  Q 354.30 454.50 311.89 481.71\n  Q 307.52 484.51 304.77 487.61\n  Q 300.73 492.14 297.40 497.37\n  Q 297.12 497.80 297.47 498.18\n  L 308.75 510.38\n  A 1.46 1.45 51.1 0 0 310.64 510.59\n  Q 321.93 502.83 334.60 497.11\n  Q 340.26 494.56 343.68 494.42\n  C 352.51 494.04 359.46 500.96 361.62 509.29\n  C 366.46 527.94 344.44 533.75 331.46 537.89\n  Q 324.88 539.98 313.18 546.11\n  Q 312.73 546.35 312.73 546.85\n  Q 312.74 560.30 312.89 573.76\n  Q 312.92 577.24 314.71 580.77\n  C 320.99 593.20 330.24 594.55 342.90 594.87\n  C 351.06 595.08 359.46 594.41 366.07 589.54\n  Q 368.41 587.82 374.72 581.23\n  Q 381.78 573.87 388.02 565.76\n  A 3.31 3.31 0.0 0 0 388.70 563.75\n  C 388.78 553.29 386.62 540.67 395.76 533.02\n  C 405.27 525.08 418.74 530.17 422.93 541.00\n  Q 424.35 544.66 424.35 552.70\n  Q 424.35 567.22 424.46 581.75\n  C 424.49 585.01 416.22 591.86 413.76 594.11\n  A 0.37 0.37 0.0 0 0 414.01 594.75\n  Q 451.31 595.19 488.75 594.99\n  Q 502.73 594.92 508.69 593.91\n  C 534.80 589.50 556.34 569.48 562.04 543.31\n  C 563.42 536.96 562.98 529.73 563.02 522.90\n  Q 563.06 515.66 564.00 512.70\n  C 569.29 496.02 591.82 495.65 598.02 512.02\n  Q 599.51 515.95 599.52 525.21\n  C 599.54 540.38 599.09 552.99 593.04 567.55\n  C 591.24 571.88 589.65 576.39 587.36 580.30\n  C 580.58 591.89 572.21 601.59 561.86 610.13\n  Q 558.81 612.65 556.38 615.26\n  Q 546.81 620.87 542.11 622.83\n  C 530.69 627.61 514.87 632.91 501.76 632.77\n  Q 473.94 632.47 446.27 632.53\n  Q 445.73 632.53 445.66 633.06\n  L 429.75 748.41\n  L 419.34 823.76\n  A 2.21 2.20 -66.6 0 0 420.13 825.77\n  Q 457.43 856.25 506.30 862.08\n  Q 559.10 868.76 606.29 847.04\n  Q 635.26 833.71 662.51 811.24\n  Q 662.86 810.95 662.86 810.50\n  Q 663.37 654.88 662.77 499.26\n  Q 662.72 486.04 662.14 479.27\n  Q 659.21 444.88 643.88 413.71\n  Q 641.69 409.25 638.37 405.30\n  Q 637.73 404.55 637.87 405.53\n  Q 639.71 418.11 631.02 425.14\n  C 621.91 432.50 609.17 429.32 604.37 418.80\n  Q 603.03 415.86 603.01 410.27\n  Q 602.97 390.62 602.90 370.98\n  Q 602.89 370.49 603.15 370.06\n  Q 634.73 316.60 623.34 256.18\n  Q 613.32 203.04 569.79 169.06\n  Q 569.22 168.61 569.26 169.34\n  Q 569.52 174.19 570.65 180.00\n  C 577.59 215.96 581.55 253.35 581.71 290.50\n  Q 581.84 318.25 581.78 346.00\n  C 581.74 365.46 554.87 370.52 547.23 352.49\n  Q 545.97 349.51 545.94 344.36\n  Q 545.89 337.11 545.87 329.84\n  Q 545.86 329.27 545.29 329.27\n  L 498.48 329.35\n  Z\n  M 409.16 196.24\n  C 408.18 229.52 415.80 263.93 429.39 294.50\n  A 1.41 1.40 69.8 0 0 431.07 295.28\n  Q 436.93 293.55 442.96 292.59\n  C 451.31 291.26 460.90 291.60 469.69 291.80\n  A 0.68 0.67 -16.6 0 0 470.27 290.74\n  Q 459.08 274.61 448.95 257.81\n  Q 430.55 227.33 410.64 195.83\n  Q 409.23 193.60 409.16 196.24\n  Z\"\n/>\n<path fill=\"#fefdf7\" d=\"\n  M 506.30 862.08\n  Q 506.44 862.11 506.61 862.08\n  Q 506.96 862.02 506.96 861.66\n  L 506.64 667.39\n  Q 532.01 642.28 556.38 615.26\n  Q 558.81 612.65 561.86 610.13\n  C 572.21 601.59 580.58 591.89 587.36 580.30\n  C 589.65 576.39 591.24 571.88 593.04 567.55\n  C 599.09 552.99 599.54 540.38 599.52 525.21\n  Q 599.51 515.95 598.02 512.02\n  C 591.82 495.65 569.29 496.02 564.00 512.70\n  Q 563.06 515.66 563.02 522.90\n  C 562.98 529.73 563.42 536.96 562.04 543.31\n  C 556.34 569.48 534.80 589.50 508.69 593.91\n  Q 502.73 594.92 488.75 594.99\n  Q 451.31 595.19 414.01 594.75\n  A 0.37 0.37 0.0 0 1 413.76 594.11\n  C 416.22 591.86 424.49 585.01 424.46 581.75\n  Q 424.35 567.22 424.35 552.70\n  Q 424.35 544.66 422.93 541.00\n  C 418.74 530.17 405.27 525.08 395.76 533.02\n  C 386.62 540.67 388.78 553.29 388.70 563.75\n  A 3.31 3.31 0.0 0 1 388.02 565.76\n  Q 381.78 573.87 374.72 581.23\n  Q 368.41 587.82 366.07 589.54\n  C 359.46 594.41 351.06 595.08 342.90 594.87\n  C 330.24 594.55 320.99 593.20 314.71 580.77\n  Q 312.92 577.24 312.89 573.76\n  Q 312.74 560.30 312.73 546.85\n  Q 312.73 546.35 313.18 546.11\n  Q 324.88 539.98 331.46 537.89\n  C 344.44 533.75 366.46 527.94 361.62 509.29\n  C 359.46 500.96 352.51 494.04 343.68 494.42\n  Q 340.26 494.56 334.60 497.11\n  Q 321.93 502.83 310.64 510.59\n  A 1.46 1.45 51.1 0 1 308.75 510.38\n  L 297.47 498.18\n  Q 297.12 497.80 297.40 497.37\n  Q 300.73 492.14 304.77 487.61\n  Q 307.52 484.51 311.89 481.71\n  Q 354.30 454.50 395.89 426.10\n  C 396.51 425.87 397.47 425.60 398.01 425.20\n  Q 413.92 413.63 429.41 401.92\n  A 1.44 1.43 -27.7 0 0 429.91 400.33\n  Q 425.74 387.75 433.29 378.42\n  C 436.89 373.98 444.19 369.27 448.55 365.30\n  Q 465.37 349.98 482.25 334.74\n  Q 484.26 332.93 485.58 330.73\n  A 2.33 2.32 -75.2 0 1 487.52 329.60\n  L 498.48 329.35\n  L 545.29 329.27\n  Q 545.86 329.27 545.87 329.84\n  Q 545.89 337.11 545.94 344.36\n  Q 545.97 349.51 547.23 352.49\n  C 554.87 370.52 581.74 365.46 581.78 346.00\n  Q 581.84 318.25 581.71 290.50\n  C 581.55 253.35 577.59 215.96 570.65 180.00\n  Q 569.52 174.19 569.26 169.34\n  Q 569.22 168.61 569.79 169.06\n  Q 613.32 203.04 623.34 256.18\n  Q 634.73 316.60 603.15 370.06\n  Q 602.89 370.49 602.90 370.98\n  Q 602.97 390.62 603.01 410.27\n  Q 603.03 415.86 604.37 418.80\n  C 609.17 429.32 621.91 432.50 631.02 425.14\n  Q 639.71 418.11 637.87 405.53\n  Q 637.73 404.55 638.37 405.30\n  Q 641.69 409.25 643.88 413.71\n  Q 659.21 444.88 662.14 479.27\n  Q 662.72 486.04 662.77 499.26\n  Q 663.37 654.88 662.86 810.50\n  Q 662.86 810.95 662.51 811.24\n  Q 635.26 833.71 606.29 847.04\n  Q 559.10 868.76 506.30 862.08\n  Z\n  M 487.09 448.99\n  C 488.00 450.86 488.88 453.02 489.81 454.47\n  C 501.58 472.76 525.60 473.20 538.05 454.99\n  Q 544.25 445.93 542.41 436.29\n  C 539.29 420.03 525.95 409.27 509.51 410.21\n  C 490.01 411.33 470.69 431.75 461.77 447.67\n  A 0.51 0.51 0.0 0 0 462.22 448.43\n  L 486.22 448.44\n  A 0.96 0.96 0.0 0 1 487.09 448.99\n  Z\"\n/>\n<path fill=\"#456e77\" d=\"\n  M 429.39 294.50\n  C 415.80 263.93 408.18 229.52 409.16 196.24\n  Q 409.23 193.60 410.64 195.83\n  Q 430.55 227.33 448.95 257.81\n  Q 459.08 274.61 470.27 290.74\n  A 0.68 0.67 -16.6 0 1 469.69 291.80\n  C 460.90 291.60 451.31 291.26 442.96 292.59\n  Q 436.93 293.55 431.07 295.28\n  A 1.41 1.40 69.8 0 1 429.39 294.50\n  Z\"\n/>\n<path fill=\"#9fd4d7\" d=\"\n  M 498.48 329.35\n  L 487.52 329.60\n  A 2.33 2.32 -75.2 0 0 485.58 330.73\n  Q 484.26 332.93 482.25 334.74\n  Q 465.37 349.98 448.55 365.30\n  C 444.19 369.27 436.89 373.98 433.29 378.42\n  Q 425.74 387.75 429.91 400.33\n  A 1.44 1.43 -27.7 0 1 429.41 401.92\n  Q 413.92 413.63 398.01 425.20\n  C 397.47 425.60 396.51 425.87 395.89 426.10\n  C 395.72 415.33 395.63 401.03 396.03 393.65\n  C 397.62 364.01 416.92 336.54 446.83 330.55\n  Q 453.25 329.27 466.73 329.25\n  Q 482.61 329.24 498.48 329.35\n  Z\"\n/>\n<path fill=\"#012630\" d=\"\n  M 486.22 448.44\n  L 462.22 448.43\n  A 0.51 0.51 0.0 0 1 461.77 447.67\n  C 470.69 431.75 490.01 411.33 509.51 410.21\n  C 525.95 409.27 539.29 420.03 542.41 436.29\n  Q 544.25 445.93 538.05 454.99\n  C 525.60 473.20 501.58 472.76 489.81 454.47\n  C 488.88 453.02 488.00 450.86 487.09 448.99\n  A 0.96 0.96 0.0 0 0 486.22 448.44\n  Z\"\n/>\n<path fill=\"#456e77\" d=\"\n  M 556.38 615.26\n  Q 532.01 642.28 506.64 667.39\n  L 429.75 748.41\n  L 445.66 633.06\n  Q 445.73 632.53 446.27 632.53\n  Q 473.94 632.47 501.76 632.77\n  C 514.87 632.91 530.69 627.61 542.11 622.83\n  Q 546.81 620.87 556.38 615.26\n  Z\"\n/>\n<path fill=\"#9fd4d7\" d=\"\n  M 506.64 667.39\n  L 506.96 861.66\n  Q 506.96 862.02 506.61 862.08\n  Q 506.44 862.11 506.30 862.08\n  Q 457.43 856.25 420.13 825.77\n  A 2.21 2.20 -66.6 0 1 419.34 823.76\n  L 429.75 748.41\n  L 506.64 667.39\n  Z\"\n/> \n</svg>",
      "variables": {},
      "input": {
        "method": "PUT",
        "url": "http://example.com",
        "headers": "",
        "body": ""
      },
      "input_schema": {
        "type": "object",
        "properties": {
          "method": {
            "type": "string",
            "enum": ["GET", "POST", "PUT", "DELETE"]
          },
          "url": {
            "type": "string"
          },
          "headers": {
            "type": "string"
          },
          "body": {
            "type": "string"
          }
        },
        "required": ["method", "url"],
        "additionalProperties": false
      },
      "presentation": {
        "position": {
          "x": 300,
          "y": 500
        }
      },
      "handles": [
        {
          "id": "a", 
          "type": "target",
          "position": "top"
        },
        {
          "id": "b",
          "type": "source",
          "position": "bottom"
        }
      ]
    }
  ]
}', now(), now(), '5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f', '5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f'),
    ('33333333-3333-3333-3333-333333333333', 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'cccccccc-cccc-cccc-cccc-cccccccccccc', 'v1.0', 'Initial version of Flow 3', 'checksum3', true,  '{
    "actions": [
    {
      "anything_action_version": "1.0.0",
      "type": "action",
      "plugin_id": "example_plugin",
      "node_id": "example_action",
      "plugin_version": "1.0.0",
      "label": "Example Action 1",
      "description": "This is the first example action",
      "icon": "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 320 320\"><path d=\"m297.06 130.97c7.26-21.79 4.76-45.66-6.85-65.48-17.46-30.4-52.56-46.04-86.84-38.68-15.25-17.18-37.16-26.95-60.13-26.81-35.04-.08-66.13 22.48-76.91 55.82-22.51 4.61-41.94 18.7-53.31 38.67-17.59 30.32-13.58 68.54 9.92 94.54-7.26 21.79-4.76 45.66 6.85 65.48 17.46 30.4 52.56 46.04 86.84 38.68 15.24 17.18 37.16 26.95 60.13 26.8 35.06.09 66.16-22.49 76.94-55.86 22.51-4.61 41.94-18.7 53.31-38.67 17.57-30.32 13.55-68.51-9.94-94.51zm-120.28 168.11c-14.03.02-27.62-4.89-38.39-13.88.49-.26 1.34-.73 1.89-1.07l63.72-36.8c3.26-1.85 5.26-5.32 5.24-9.07v-89.83l26.93 15.55c.29.14.48.42.52.74v74.39c-.04 33.08-26.83 59.9-59.91 59.97zm-128.84-55.03c-7.03-12.14-9.56-26.37-7.15-40.18.47.28 1.3.79 1.89 1.13l63.72 36.8c3.23 1.89 7.23 1.89 10.47 0l77.79-44.92v31.1c.02.32-.13.63-.38.83l-64.41 37.19c-28.69 16.52-65.33 6.7-81.92-21.95zm-16.77-139.09c7-12.16 18.05-21.46 31.21-26.29 0 .55-.03 1.52-.03 2.2v73.61c-.02 3.74 1.98 7.21 5.23 9.06l77.79 44.91-26.93 15.55c-.27.18-.61.21-.91.08l-64.42-37.22c-28.63-16.58-38.45-53.21-21.95-81.89zm221.26 51.49-77.79-44.92 26.93-15.54c.27-.18.61-.21.91-.08l64.42 37.19c28.68 16.57 38.51 53.26 21.94 81.94-7.01 12.14-18.05 21.44-31.2 26.28v-75.81c.03-3.74-1.96-7.2-5.2-9.06zm26.8-40.34c-.47-.29-1.3-.79-1.89-1.13l-63.72-36.8c-3.23-1.89-7.23-1.89-10.47 0l-77.79 44.92v-31.1c-.02-.32.13-.63.38-.83l64.41-37.16c28.69-16.55 65.37-6.7 81.91 22 6.99 12.12 9.52 26.31 7.15 40.1zm-168.51 55.43-26.94-15.55c-.29-.14-.48-.42-.52-.74v-74.39c.02-33.12 26.89-59.96 60.01-59.94 14.01 0 27.57 4.92 38.34 13.88-.49.26-1.33.73-1.89 1.07l-63.72 36.8c-3.26 1.85-5.26 5.31-5.24 9.06l-.04 89.79zm14.63-31.54 34.65-20.01 34.65 20v40.01l-34.65 20-34.65-20z\"/></svg>",
      "variables": {},
      "input": {
        "method": "GET",
        "url": "http://example.com",
        "headers": "",
        "body": ""
      },
      "input_schema": {
        "type": "object",
        "properties": {
          "method": {
            "type": "string",
            "enum": ["GET", "POST", "PUT", "DELETE"]
          },
          "url": {
            "type": "string"
          },
          "headers": {
            "type": "string"
          },
          "body": {
            "type": "string"
          }
        },
        "required": ["method", "url"],
        "additionalProperties": false
      },
      "presentation": {
        "position": {
          "x": 300,
          "y": 300
        }
      },
      "handles": [
        {
          "id": "a", 
          "type": "target",
          "position": "top"
        },
        {
          "id": "b",
          "type": "source",
          "position": "bottom"
        }
      ]
    },
    {
      "anything_action_version": "1.0.0",
      "type": "action",
      "plugin_id": "example_plugin",
      "node_id": "example_action_2",
      "plugin_version": "1.0.0",
      "label": "Example Action 2",
      "description": "This is the second example action",
      "icon": "<svg xmlns:xodm=\"http://www.corel.com/coreldraw/odm/2003\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" version=\"1.1\" id=\"Layer_1\" x=\"0px\" y=\"0px\" viewBox=\"0 0 2500 2503\" style=\"enable-background:new 0 0 2500 2503;\" xml:space=\"preserve\">\n<style type=\"text/css\">\n\t.st0{fill:none;}\n\t.st1{fill:#F0CDC2;stroke:#1616B4;stroke-width:3.1298;stroke-linejoin:round;stroke-miterlimit:22.9245;}\n\t.st2{fill:#C9B3F5;stroke:#1616B4;stroke-width:3.1298;stroke-linejoin:round;stroke-miterlimit:22.9245;}\n\t.st3{fill:#88AAF1;stroke:#1616B4;stroke-width:3.1298;stroke-linejoin:round;stroke-miterlimit:22.9245;}\n\t.st4{fill:#B8FAF6;stroke:#1616B4;stroke-width:3.1298;stroke-linejoin:round;stroke-miterlimit:22.9245;}\n</style>\n<g id=\"Layer_x0020_1\">\n\t<rect x=\"0.1\" y=\"1.6\" class=\"st0\" width=\"2499.9\" height=\"2499.9\"></rect>\n\t<g id=\"_2082587881456\">\n\t\t<polygon class=\"st1\" points=\"1248.7,2501.4 1248.7,1874.2 472.8,1420.5   \"></polygon>\n\t\t<polygon class=\"st2\" points=\"1251.3,2501.4 1251.3,1874.2 2027.1,1420.5   \"></polygon>\n\t\t<polygon class=\"st3\" points=\"1248.7,1718.4 1248.7,917.9 464,1269.3   \"></polygon>\n\t\t<polygon class=\"st2\" points=\"1251.3,1718.4 1251.3,917.9 2036,1269.3   \"></polygon>\n\t\t<polygon class=\"st1\" points=\"464,1269.3 1248.7,1.6 1248.7,917.9   \"></polygon>\n\t\t<polygon class=\"st4\" points=\"2036,1269.3 1251.3,1.6 1251.3,917.9   \"></polygon>\n\t</g>\n</g>\n</svg>",
      "variables": {},
      "input": {
        "method": "POST",
        "url": "http://example.com",
        "headers": "",
        "body": ""
      },
      "input_schema": {
        "type": "object",
        "properties": {
          "method": {
            "type": "string",
            "enum": ["GET", "POST", "PUT", "DELETE"]
          },
          "url": {
            "type": "string"
          },
          "headers": {
            "type": "string"
          },
          "body": {
            "type": "string"
          }
        },
        "required": ["method", "url"],
        "additionalProperties": false
      },
      "presentation": {
        "position": {
          "x": 300,
          "y": 500
        }
      },
      "handles": [
        {
          "id": "a", 
          "type": "target",
          "position": "top"
        },
        {
          "id": "b",
          "type": "source",
          "position": "bottom"
        }
      ]
    },
    {
      "anything_action_version": "1.0.0",
      "type": "action",
      "plugin_id": "other_example_plugin",
      "node_id": "other_example_action",
      "plugin_version": "1.0.0",
      "label": "Example Action 3",
      "description": "This is the third example action",
      "icon": "<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" viewBox=\"0.00 0.00 1024.00 1024.00\">\n<path fill=\"none\" d=\" \n  M 0.00 0.00 \n  L 1024.00 0.00\n  L 1024.00 1024.00\n  L 0.00 1024.00\n  L 0.00 0.00\n  Z\n  M 544.53 291.36\n  L 514.04 291.29\n  A 0.89 0.89 0.0 0 1 513.28 290.86\n  C 503.27 274.77 492.95 258.85 483.09 242.71\n  Q 451.44 190.92 418.99 139.61\n  C 414.92 133.17 410.92 125.28 402.26 125.83\n  C 394.88 126.31 389.53 132.50 386.81 139.04\n  C 363.54 194.95 374.90 259.13 396.96 313.20\n  Q 397.16 313.68 396.78 314.03\n  C 369.88 338.98 358.72 371.19 357.81 407.33\n  Q 357.79 407.84 357.37 408.12\n  Q 332.96 424.39 308.38 440.40\n  C 289.89 452.44 270.95 463.40 264.29 486.21\n  Q 256.87 511.57 276.81 531.52\n  Q 277.17 531.88 277.17 532.39\n  Q 277.14 550.31 277.08 568.22\n  C 277.03 582.05 277.80 591.67 284.08 602.43\n  C 299.76 629.28 330.05 636.97 359.08 631.14\n  Q 371.03 628.75 381.44 621.32\n  Q 381.84 621.03 382.27 621.29\n  Q 394.70 628.94 408.97 632.17\n  A 0.52 0.52 0.0 0 1 409.37 632.75\n  Q 395.08 736.01 380.36 838.75\n  Q 380.07 840.76 382.28 842.75\n  Q 417.48 874.48 460.23 889.16\n  C 500.49 902.98 544.08 903.97 585.14 893.12\n  C 592.22 891.25 599.98 888.52 607.38 886.04\n  Q 616.55 882.97 623.65 879.59\n  Q 664.48 860.16 697.91 830.12\n  A 2.38 2.38 0.0 0 0 698.71 828.34\n  Q 698.70 664.99 698.57 502.00\n  Q 698.57 501.95 698.64 493.49\n  C 699.00 452.17 688.48 411.86 665.26 377.44\n  Q 658.46 367.36 651.02 358.22\n  Q 650.55 357.64 650.79 356.93\n  C 668.11 306.20 667.74 253.83 645.17 204.88\n  Q 636.43 185.92 620.58 166.44\n  Q 599.71 140.81 571.70 128.31\n  C 563.74 124.76 554.58 122.40 546.05 124.93\n  C 537.36 127.51 530.84 134.43 529.35 143.45\n  Q 528.66 147.62 530.15 156.30\n  C 537.79 200.66 544.76 245.44 545.18 290.70\n  A 0.65 0.65 0.0 0 1 544.53 291.36\n  Z\"\n/>\n<path fill=\"#012630\" d=\"\n  M 544.53 291.36\n  A 0.65 0.65 0.0 0 0 545.18 290.70\n  C 544.76 245.44 537.79 200.66 530.15 156.30\n  Q 528.66 147.62 529.35 143.45\n  C 530.84 134.43 537.36 127.51 546.05 124.93\n  C 554.58 122.40 563.74 124.76 571.70 128.31\n  Q 599.71 140.81 620.58 166.44\n  Q 636.43 185.92 645.17 204.88\n  C 667.74 253.83 668.11 306.20 650.79 356.93\n  Q 650.55 357.64 651.02 358.22\n  Q 658.46 367.36 665.26 377.44\n  C 688.48 411.86 699.00 452.17 698.64 493.49\n  Q 698.57 501.95 698.57 502.00\n  Q 698.70 664.99 698.71 828.34\n  A 2.38 2.38 0.0 0 1 697.91 830.12\n  Q 664.48 860.16 623.65 879.59\n  Q 616.55 882.97 607.38 886.04\n  C 599.98 888.52 592.22 891.25 585.14 893.12\n  C 544.08 903.97 500.49 902.98 460.23 889.16\n  Q 417.48 874.48 382.28 842.75\n  Q 380.07 840.76 380.36 838.75\n  Q 395.08 736.01 409.37 632.75\n  A 0.52 0.52 0.0 0 0 408.97 632.17\n  Q 394.70 628.94 382.27 621.29\n  Q 381.84 621.03 381.44 621.32\n  Q 371.03 628.75 359.08 631.14\n  C 330.05 636.97 299.76 629.28 284.08 602.43\n  C 277.80 591.67 277.03 582.05 277.08 568.22\n  Q 277.14 550.31 277.17 532.39\n  Q 277.17 531.88 276.81 531.52\n  Q 256.87 511.57 264.29 486.21\n  C 270.95 463.40 289.89 452.44 308.38 440.40\n  Q 332.96 424.39 357.37 408.12\n  Q 357.79 407.84 357.81 407.33\n  C 358.72 371.19 369.88 338.98 396.78 314.03\n  Q 397.16 313.68 396.96 313.20\n  C 374.90 259.13 363.54 194.95 386.81 139.04\n  C 389.53 132.50 394.88 126.31 402.26 125.83\n  C 410.92 125.28 414.92 133.17 418.99 139.61\n  Q 451.44 190.92 483.09 242.71\n  C 492.95 258.85 503.27 274.77 513.28 290.86\n  A 0.89 0.89 0.0 0 0 514.04 291.29\n  L 544.53 291.36\n  Z\n  M 498.48 329.35\n  Q 482.61 329.24 466.73 329.25\n  Q 453.25 329.27 446.83 330.55\n  C 416.92 336.54 397.62 364.01 396.03 393.65\n  C 395.63 401.03 395.72 415.33 395.89 426.10\n  Q 354.30 454.50 311.89 481.71\n  Q 307.52 484.51 304.77 487.61\n  Q 300.73 492.14 297.40 497.37\n  Q 297.12 497.80 297.47 498.18\n  L 308.75 510.38\n  A 1.46 1.45 51.1 0 0 310.64 510.59\n  Q 321.93 502.83 334.60 497.11\n  Q 340.26 494.56 343.68 494.42\n  C 352.51 494.04 359.46 500.96 361.62 509.29\n  C 366.46 527.94 344.44 533.75 331.46 537.89\n  Q 324.88 539.98 313.18 546.11\n  Q 312.73 546.35 312.73 546.85\n  Q 312.74 560.30 312.89 573.76\n  Q 312.92 577.24 314.71 580.77\n  C 320.99 593.20 330.24 594.55 342.90 594.87\n  C 351.06 595.08 359.46 594.41 366.07 589.54\n  Q 368.41 587.82 374.72 581.23\n  Q 381.78 573.87 388.02 565.76\n  A 3.31 3.31 0.0 0 0 388.70 563.75\n  C 388.78 553.29 386.62 540.67 395.76 533.02\n  C 405.27 525.08 418.74 530.17 422.93 541.00\n  Q 424.35 544.66 424.35 552.70\n  Q 424.35 567.22 424.46 581.75\n  C 424.49 585.01 416.22 591.86 413.76 594.11\n  A 0.37 0.37 0.0 0 0 414.01 594.75\n  Q 451.31 595.19 488.75 594.99\n  Q 502.73 594.92 508.69 593.91\n  C 534.80 589.50 556.34 569.48 562.04 543.31\n  C 563.42 536.96 562.98 529.73 563.02 522.90\n  Q 563.06 515.66 564.00 512.70\n  C 569.29 496.02 591.82 495.65 598.02 512.02\n  Q 599.51 515.95 599.52 525.21\n  C 599.54 540.38 599.09 552.99 593.04 567.55\n  C 591.24 571.88 589.65 576.39 587.36 580.30\n  C 580.58 591.89 572.21 601.59 561.86 610.13\n  Q 558.81 612.65 556.38 615.26\n  Q 546.81 620.87 542.11 622.83\n  C 530.69 627.61 514.87 632.91 501.76 632.77\n  Q 473.94 632.47 446.27 632.53\n  Q 445.73 632.53 445.66 633.06\n  L 429.75 748.41\n  L 419.34 823.76\n  A 2.21 2.20 -66.6 0 0 420.13 825.77\n  Q 457.43 856.25 506.30 862.08\n  Q 559.10 868.76 606.29 847.04\n  Q 635.26 833.71 662.51 811.24\n  Q 662.86 810.95 662.86 810.50\n  Q 663.37 654.88 662.77 499.26\n  Q 662.72 486.04 662.14 479.27\n  Q 659.21 444.88 643.88 413.71\n  Q 641.69 409.25 638.37 405.30\n  Q 637.73 404.55 637.87 405.53\n  Q 639.71 418.11 631.02 425.14\n  C 621.91 432.50 609.17 429.32 604.37 418.80\n  Q 603.03 415.86 603.01 410.27\n  Q 602.97 390.62 602.90 370.98\n  Q 602.89 370.49 603.15 370.06\n  Q 634.73 316.60 623.34 256.18\n  Q 613.32 203.04 569.79 169.06\n  Q 569.22 168.61 569.26 169.34\n  Q 569.52 174.19 570.65 180.00\n  C 577.59 215.96 581.55 253.35 581.71 290.50\n  Q 581.84 318.25 581.78 346.00\n  C 581.74 365.46 554.87 370.52 547.23 352.49\n  Q 545.97 349.51 545.94 344.36\n  Q 545.89 337.11 545.87 329.84\n  Q 545.86 329.27 545.29 329.27\n  L 498.48 329.35\n  Z\n  M 409.16 196.24\n  C 408.18 229.52 415.80 263.93 429.39 294.50\n  A 1.41 1.40 69.8 0 0 431.07 295.28\n  Q 436.93 293.55 442.96 292.59\n  C 451.31 291.26 460.90 291.60 469.69 291.80\n  A 0.68 0.67 -16.6 0 0 470.27 290.74\n  Q 459.08 274.61 448.95 257.81\n  Q 430.55 227.33 410.64 195.83\n  Q 409.23 193.60 409.16 196.24\n  Z\"\n/>\n<path fill=\"#fefdf7\" d=\"\n  M 506.30 862.08\n  Q 506.44 862.11 506.61 862.08\n  Q 506.96 862.02 506.96 861.66\n  L 506.64 667.39\n  Q 532.01 642.28 556.38 615.26\n  Q 558.81 612.65 561.86 610.13\n  C 572.21 601.59 580.58 591.89 587.36 580.30\n  C 589.65 576.39 591.24 571.88 593.04 567.55\n  C 599.09 552.99 599.54 540.38 599.52 525.21\n  Q 599.51 515.95 598.02 512.02\n  C 591.82 495.65 569.29 496.02 564.00 512.70\n  Q 563.06 515.66 563.02 522.90\n  C 562.98 529.73 563.42 536.96 562.04 543.31\n  C 556.34 569.48 534.80 589.50 508.69 593.91\n  Q 502.73 594.92 488.75 594.99\n  Q 451.31 595.19 414.01 594.75\n  A 0.37 0.37 0.0 0 1 413.76 594.11\n  C 416.22 591.86 424.49 585.01 424.46 581.75\n  Q 424.35 567.22 424.35 552.70\n  Q 424.35 544.66 422.93 541.00\n  C 418.74 530.17 405.27 525.08 395.76 533.02\n  C 386.62 540.67 388.78 553.29 388.70 563.75\n  A 3.31 3.31 0.0 0 1 388.02 565.76\n  Q 381.78 573.87 374.72 581.23\n  Q 368.41 587.82 366.07 589.54\n  C 359.46 594.41 351.06 595.08 342.90 594.87\n  C 330.24 594.55 320.99 593.20 314.71 580.77\n  Q 312.92 577.24 312.89 573.76\n  Q 312.74 560.30 312.73 546.85\n  Q 312.73 546.35 313.18 546.11\n  Q 324.88 539.98 331.46 537.89\n  C 344.44 533.75 366.46 527.94 361.62 509.29\n  C 359.46 500.96 352.51 494.04 343.68 494.42\n  Q 340.26 494.56 334.60 497.11\n  Q 321.93 502.83 310.64 510.59\n  A 1.46 1.45 51.1 0 1 308.75 510.38\n  L 297.47 498.18\n  Q 297.12 497.80 297.40 497.37\n  Q 300.73 492.14 304.77 487.61\n  Q 307.52 484.51 311.89 481.71\n  Q 354.30 454.50 395.89 426.10\n  C 396.51 425.87 397.47 425.60 398.01 425.20\n  Q 413.92 413.63 429.41 401.92\n  A 1.44 1.43 -27.7 0 0 429.91 400.33\n  Q 425.74 387.75 433.29 378.42\n  C 436.89 373.98 444.19 369.27 448.55 365.30\n  Q 465.37 349.98 482.25 334.74\n  Q 484.26 332.93 485.58 330.73\n  A 2.33 2.32 -75.2 0 1 487.52 329.60\n  L 498.48 329.35\n  L 545.29 329.27\n  Q 545.86 329.27 545.87 329.84\n  Q 545.89 337.11 545.94 344.36\n  Q 545.97 349.51 547.23 352.49\n  C 554.87 370.52 581.74 365.46 581.78 346.00\n  Q 581.84 318.25 581.71 290.50\n  C 581.55 253.35 577.59 215.96 570.65 180.00\n  Q 569.52 174.19 569.26 169.34\n  Q 569.22 168.61 569.79 169.06\n  Q 613.32 203.04 623.34 256.18\n  Q 634.73 316.60 603.15 370.06\n  Q 602.89 370.49 602.90 370.98\n  Q 602.97 390.62 603.01 410.27\n  Q 603.03 415.86 604.37 418.80\n  C 609.17 429.32 621.91 432.50 631.02 425.14\n  Q 639.71 418.11 637.87 405.53\n  Q 637.73 404.55 638.37 405.30\n  Q 641.69 409.25 643.88 413.71\n  Q 659.21 444.88 662.14 479.27\n  Q 662.72 486.04 662.77 499.26\n  Q 663.37 654.88 662.86 810.50\n  Q 662.86 810.95 662.51 811.24\n  Q 635.26 833.71 606.29 847.04\n  Q 559.10 868.76 506.30 862.08\n  Z\n  M 487.09 448.99\n  C 488.00 450.86 488.88 453.02 489.81 454.47\n  C 501.58 472.76 525.60 473.20 538.05 454.99\n  Q 544.25 445.93 542.41 436.29\n  C 539.29 420.03 525.95 409.27 509.51 410.21\n  C 490.01 411.33 470.69 431.75 461.77 447.67\n  A 0.51 0.51 0.0 0 0 462.22 448.43\n  L 486.22 448.44\n  A 0.96 0.96 0.0 0 1 487.09 448.99\n  Z\"\n/>\n<path fill=\"#456e77\" d=\"\n  M 429.39 294.50\n  C 415.80 263.93 408.18 229.52 409.16 196.24\n  Q 409.23 193.60 410.64 195.83\n  Q 430.55 227.33 448.95 257.81\n  Q 459.08 274.61 470.27 290.74\n  A 0.68 0.67 -16.6 0 1 469.69 291.80\n  C 460.90 291.60 451.31 291.26 442.96 292.59\n  Q 436.93 293.55 431.07 295.28\n  A 1.41 1.40 69.8 0 1 429.39 294.50\n  Z\"\n/>\n<path fill=\"#9fd4d7\" d=\"\n  M 498.48 329.35\n  L 487.52 329.60\n  A 2.33 2.32 -75.2 0 0 485.58 330.73\n  Q 484.26 332.93 482.25 334.74\n  Q 465.37 349.98 448.55 365.30\n  C 444.19 369.27 436.89 373.98 433.29 378.42\n  Q 425.74 387.75 429.91 400.33\n  A 1.44 1.43 -27.7 0 1 429.41 401.92\n  Q 413.92 413.63 398.01 425.20\n  C 397.47 425.60 396.51 425.87 395.89 426.10\n  C 395.72 415.33 395.63 401.03 396.03 393.65\n  C 397.62 364.01 416.92 336.54 446.83 330.55\n  Q 453.25 329.27 466.73 329.25\n  Q 482.61 329.24 498.48 329.35\n  Z\"\n/>\n<path fill=\"#012630\" d=\"\n  M 486.22 448.44\n  L 462.22 448.43\n  A 0.51 0.51 0.0 0 1 461.77 447.67\n  C 470.69 431.75 490.01 411.33 509.51 410.21\n  C 525.95 409.27 539.29 420.03 542.41 436.29\n  Q 544.25 445.93 538.05 454.99\n  C 525.60 473.20 501.58 472.76 489.81 454.47\n  C 488.88 453.02 488.00 450.86 487.09 448.99\n  A 0.96 0.96 0.0 0 0 486.22 448.44\n  Z\"\n/>\n<path fill=\"#456e77\" d=\"\n  M 556.38 615.26\n  Q 532.01 642.28 506.64 667.39\n  L 429.75 748.41\n  L 445.66 633.06\n  Q 445.73 632.53 446.27 632.53\n  Q 473.94 632.47 501.76 632.77\n  C 514.87 632.91 530.69 627.61 542.11 622.83\n  Q 546.81 620.87 556.38 615.26\n  Z\"\n/>\n<path fill=\"#9fd4d7\" d=\"\n  M 506.64 667.39\n  L 506.96 861.66\n  Q 506.96 862.02 506.61 862.08\n  Q 506.44 862.11 506.30 862.08\n  Q 457.43 856.25 420.13 825.77\n  A 2.21 2.20 -66.6 0 1 419.34 823.76\n  L 429.75 748.41\n  L 506.64 667.39\n  Z\"\n/> \n</svg>",
      "variables": {},
      "input": {
        "method": "PUT",
        "url": "http://example.com",
        "headers": "",
        "body": ""
      },
      "input_schema": {
        "type": "object",
        "properties": {
          "method": {
            "type": "string",
            "enum": ["GET", "POST", "PUT", "DELETE"]
          },
          "url": {
            "type": "string"
          },
          "headers": {
            "type": "string"
          },
          "body": {
            "type": "string"
          }
        },
        "required": ["method", "url"],
        "additionalProperties": false
      },
      "presentation": {
        "position": {
          "x": 300,
          "y": 700
        }
      },
      "handles": [
        {
          "id": "a", 
          "type": "target",
          "position": "top"
        },
        {
          "id": "b",
          "type": "source",
          "position": "bottom"
        }
      ]
    }
  ]
}', now(), now(), '2a7b3d8e-2f3c-4b5d-8e3a-4a7c3e6a7c8d', '2a7b3d8e-2f3c-4b5d-8e3a-4a7c3e6a7c8d');

-- Inserting sample events into anything.events
INSERT INTO anything.tasks (
    task_id, account_id, task_status, flow_id, flow_version_id, flow_version_name, trigger_id, trigger_session_id, trigger_session_status, flow_session_id, flow_session_status, node_id, is_trigger, plugin_id, stage, config, context, started_at, ended_at, debug_result, result, updated_at, created_at, updated_by, created_by
) VALUES
    (uuid_generate_v4(), 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'completed', 'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa', '11111111-1111-1111-1111-111111111111', 'v1.0', 'trigger1', 'session1', 'completed', 'flow_session_1', 'completed', 'node1', false, 'extension1', 'DEV', '{"key": "value"}', '{"key": "value"}', now(), now(), '{"key": "value"}', '{"key": "value"}', now(), now(), '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8', '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8'),
    (uuid_generate_v4(), '7df12345-a5d3-4b13-9e3a-2f5c3e6a7b91', 'completed', 'bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb', '22222222-2222-2222-2222-222222222222', 'v1.0', 'trigger2', 'session2', 'completed', 'flow_session_2', 'completed', 'node2', false, 'extension2', 'DEV', '{"key": "value"}', '{"key": "value"}', now(), now(), '{"key": "value"}', '{"key": "value"}', now(), now(), '5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f', '5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f'),
    (uuid_generate_v4(), 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'completed', 'cccccccc-cccc-cccc-cccc-cccccccccccc', '33333333-3333-3333-3333-333333333333', 'v1.0', 'trigger3', 'session3', 'completed', 'flow_session_3', 'completed', 'node3', false, 'extension3', 'DEV', '{"key": "value"}', '{"key": "value"}', now(), now(), '{"key": "value"}', '{"key": "value"}', now(), now(), '1e4f12a7-3c55-4e6d-9b4d-2a1f0403a2a6', '1e4f12a7-3c55-4e6d-9b4d-2a1f0403a2a6'); 
    -- (uuid_generate_v4(), '7df12345-a5d3-4b13-9e3a-2f5c3e6a7b91', 'completed', 'dddddddd-dddd-dddd-dddd-dddddddddddd', '44444444-4444-4444-4444-444444444444', 'v1.0', 'trigger4', 'session4', 'completed', 'flow_session_4', 'completed', 'node4', false, 'extension4', 'DEV', '{"key": "value"}', '{"key": "value"}', now(), now(), '{"key": "value"}', '{"key": "value"}', now(), now(), '3d8b144c-1e9d-4a8c-8234-4e5c9b3d5c2f', '3d8b144c-1e9d-4a8c-8234-4e5c9b3d5c2f'),
    -- (uuid_generate_v4(), 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'completed', 'eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee', '55555555-5555-5555-5555-555555555555', 'v1.0', 'trigger5', 'session5', 'completed', 'flow_session_5', 'completed', 'node5', false, 'extension5', 'DEV', '{"key": "value"}', '{"key": "value"}', now(), now(), '{"key": "value"}', '{"key": "value"}', now(), now(), '2a7b3d8e-2f3c-4b5d-8e3a-4a7c3e6a7c8d', '2a7b3d8e-2f3c-4b5d-8e3a-4a7c3e6a7c8d');

-- Inserting sample action-templates into anyting.action_templates
INSERT INTO anything.action_templates (
    action_template_id, account_id,  action_template_definition,  updated_at, created_at, updated_by, created_by
) VALUES
    (uuid_generate_v4(), 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', '{
      "anything_action_version": "1.0.0",
      "type": "action",
      "plugin_id": "example_plugin",
      "node_id": "example_action", 
      "plugin_version": "1.0.0",
      "label": "Example Action 1",
      "description": "This is the first example action",
      "icon": "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 320 320\"><path d=\"m297.06 130.97c7.26-21.79 4.76-45.66-6.85-65.48-17.46-30.4-52.56-46.04-86.84-38.68-15.25-17.18-37.16-26.95-60.13-26.81-35.04-.08-66.13 22.48-76.91 55.82-22.51 4.61-41.94 18.7-53.31 38.67-17.59 30.32-13.58 68.54 9.92 94.54-7.26 21.79-4.76 45.66 6.85 65.48 17.46 30.4 52.56 46.04 86.84 38.68 15.24 17.18 37.16 26.95 60.13 26.8 35.06.09 66.16-22.49 76.94-55.86 22.51-4.61 41.94-18.7 53.31-38.67 17.57-30.32 13.55-68.51-9.94-94.51zm-120.28 168.11c-14.03.02-27.62-4.89-38.39-13.88.49-.26 1.34-.73 1.89-1.07l63.72-36.8c3.26-1.85 5.26-5.32 5.24-9.07v-89.83l26.93 15.55c.29.14.48.42.52.74v74.39c-.04 33.08-26.83 59.9-59.91 59.97zm-128.84-55.03c-7.03-12.14-9.56-26.37-7.15-40.18.47.28 1.3.79 1.89 1.13l63.72 36.8c3.23 1.89 7.23 1.89 10.47 0l77.79-44.92v31.1c.02.32-.13.63-.38.83l-64.41 37.19c-28.69 16.52-65.33 6.7-81.92-21.95zm-16.77-139.09c7-12.16 18.05-21.46 31.21-26.29 0 .55-.03 1.52-.03 2.2v73.61c-.02 3.74 1.98 7.21 5.23 9.06l77.79 44.91-26.93 15.55c-.27.18-.61.21-.91.08l-64.42-37.22c-28.63-16.58-38.45-53.21-21.95-81.89zm221.26 51.49-77.79-44.92 26.93-15.54c.27-.18.61-.21.91-.08l64.42 37.19c28.68 16.57 38.51 53.26 21.94 81.94-7.01 12.14-18.05 21.44-31.2 26.28v-75.81c.03-3.74-1.96-7.2-5.2-9.06zm26.8-40.34c-.47-.29-1.3-.79-1.89-1.13l-63.72-36.8c-3.23-1.89-7.23-1.89-10.47 0l-77.79 44.92v-31.1c-.02-.32.13-.63.38-.83l64.41-37.16c28.69-16.55 65.37-6.7 81.91 22 6.99 12.12 9.52 26.31 7.15 40.1zm-168.51 55.43-26.94-15.55c-.29-.14-.48-.42-.52-.74v-74.39c.02-33.12 26.89-59.96 60.01-59.94 14.01 0 27.57 4.92 38.34 13.88-.49.26-1.33.73-1.89 1.07l-63.72 36.8c-3.26 1.85-5.26 5.31-5.24 9.06l-.04 89.79zm14.63-31.54 34.65-20.01 34.65 20v40.01l-34.65 20-34.65-20z\"/></svg>",
      "variables": {},
      "input": {
        "method": "GET",
        "url": "http://example.com",
        "headers": "",
        "body": ""
      },
      "input_schema": {
        "type": "object",
        "properties": {
          "method": {
             "title": "Method",
              "description": "HTTP Method for request",
              "type": "string",
              "oneOf": [
                {
                  "value": "GET",
                  "title": "GET"
                },
                {
                  "value": "POST",
                  "title": "POST"
                },
                {
                  "value": "PUT",
                  "title": "PUT"
                },
                {
                  "value": "DELETE",
                  "title": "DELETE"
                }
              ],
              "x-jsf-presentation": {
                "inputType": "select"
              }
          },
          "url": {
             "title": "URL",
            "description": "URL for request",
            "type": "string"
          },
          "headers": {
            "title": "Headers",
            "description": "Headers for request",
            "type": "string"
          },
          "body": {
            "title": "Body",
            "description": "Body for request",
            "type": "string"
          }
        },
        "x-jsf-order": ["url", "method", "headers", "body"],
        "required": ["method", "url"],
        "additionalProperties": false
      },
      "presentation": {
        "position": {
          "x": 300,
          "y": 100
        }
      },
      "handles": [
        {
          "id": "a", 
          "type": "target",
          "position": "top"
        },
        {
          "id": "b",
          "type": "source",
          "position": "bottom"
        }
      ]
    }', now(), now(), '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8', '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8'),
    (uuid_generate_v4(), 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', '{
      "anything_action_version": "1.0.0",
      "type": "action",
      "plugin_id": "example_plugin",
      "node_id": "example_action", 
      "plugin_version": "1.0.0",
      "label": "Example Action 1",
      "description": "This is the first example action",
      "icon": "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 320 320\"><path d=\"m297.06 130.97c7.26-21.79 4.76-45.66-6.85-65.48-17.46-30.4-52.56-46.04-86.84-38.68-15.25-17.18-37.16-26.95-60.13-26.81-35.04-.08-66.13 22.48-76.91 55.82-22.51 4.61-41.94 18.7-53.31 38.67-17.59 30.32-13.58 68.54 9.92 94.54-7.26 21.79-4.76 45.66 6.85 65.48 17.46 30.4 52.56 46.04 86.84 38.68 15.24 17.18 37.16 26.95 60.13 26.8 35.06.09 66.16-22.49 76.94-55.86 22.51-4.61 41.94-18.7 53.31-38.67 17.57-30.32 13.55-68.51-9.94-94.51zm-120.28 168.11c-14.03.02-27.62-4.89-38.39-13.88.49-.26 1.34-.73 1.89-1.07l63.72-36.8c3.26-1.85 5.26-5.32 5.24-9.07v-89.83l26.93 15.55c.29.14.48.42.52.74v74.39c-.04 33.08-26.83 59.9-59.91 59.97zm-128.84-55.03c-7.03-12.14-9.56-26.37-7.15-40.18.47.28 1.3.79 1.89 1.13l63.72 36.8c3.23 1.89 7.23 1.89 10.47 0l77.79-44.92v31.1c.02.32-.13.63-.38.83l-64.41 37.19c-28.69 16.52-65.33 6.7-81.92-21.95zm-16.77-139.09c7-12.16 18.05-21.46 31.21-26.29 0 .55-.03 1.52-.03 2.2v73.61c-.02 3.74 1.98 7.21 5.23 9.06l77.79 44.91-26.93 15.55c-.27.18-.61.21-.91.08l-64.42-37.22c-28.63-16.58-38.45-53.21-21.95-81.89zm221.26 51.49-77.79-44.92 26.93-15.54c.27-.18.61-.21.91-.08l64.42 37.19c28.68 16.57 38.51 53.26 21.94 81.94-7.01 12.14-18.05 21.44-31.2 26.28v-75.81c.03-3.74-1.96-7.2-5.2-9.06zm26.8-40.34c-.47-.29-1.3-.79-1.89-1.13l-63.72-36.8c-3.23-1.89-7.23-1.89-10.47 0l-77.79 44.92v-31.1c-.02-.32.13-.63.38-.83l64.41-37.16c28.69-16.55 65.37-6.7 81.91 22 6.99 12.12 9.52 26.31 7.15 40.1zm-168.51 55.43-26.94-15.55c-.29-.14-.48-.42-.52-.74v-74.39c.02-33.12 26.89-59.96 60.01-59.94 14.01 0 27.57 4.92 38.34 13.88-.49.26-1.33.73-1.89 1.07l-63.72 36.8c-3.26 1.85-5.26 5.31-5.24 9.06l-.04 89.79zm14.63-31.54 34.65-20.01 34.65 20v40.01l-34.65 20-34.65-20z\"/></svg>",
      "variables": {},
      "input": {
        "method": "GET",
        "url": "http://example.com",
        "headers": "",
        "body": ""
      },
      "input_schema": {
        "type": "object",
        "properties": {
          "method": {
             "title": "Method",
              "description": "HTTP Method for request",
              "type": "string",
              "oneOf": [
                {
                  "value": "GET",
                  "title": "GET"
                },
                {
                  "value": "POST",
                  "title": "POST"
                },
                {
                  "value": "PUT",
                  "title": "PUT"
                },
                {
                  "value": "DELETE",
                  "title": "DELETE"
                }
              ],
              "x-jsf-presentation": {
                "inputType": "select"
              }
          },
          "url": {
             "title": "URL",
            "description": "URL for request",
            "type": "string"
          },
          "headers": {
            "title": "Headers",
            "description": "Headers for request",
            "type": "string"
          },
          "body": {
            "title": "Body",
            "description": "Body for request",
            "type": "string"
          }
        },
        "x-jsf-order": ["url", "method", "headers", "body"],
        "required": ["method", "url"],
        "additionalProperties": false
      },
      "presentation": {
        "position": {
          "x": 300,
          "y": 100
        }
      },
      "handles": [
        {
          "id": "a", 
          "type": "target",
          "position": "top"
        },
        {
          "id": "b",
          "type": "source",
          "position": "bottom"
        }
      ]
    }', now(), now(), '5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f', '5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f'),
    (uuid_generate_v4(), 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', '{
      "anything_action_version": "1.0.0",
      "type": "action",
      "plugin_id": "example_plugin",
      "node_id": "example_action", 
      "plugin_version": "1.0.0",
      "label": "Example Action 1",
      "description": "This is the first example action",
      "icon": "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 320 320\"><path d=\"m297.06 130.97c7.26-21.79 4.76-45.66-6.85-65.48-17.46-30.4-52.56-46.04-86.84-38.68-15.25-17.18-37.16-26.95-60.13-26.81-35.04-.08-66.13 22.48-76.91 55.82-22.51 4.61-41.94 18.7-53.31 38.67-17.59 30.32-13.58 68.54 9.92 94.54-7.26 21.79-4.76 45.66 6.85 65.48 17.46 30.4 52.56 46.04 86.84 38.68 15.24 17.18 37.16 26.95 60.13 26.8 35.06.09 66.16-22.49 76.94-55.86 22.51-4.61 41.94-18.7 53.31-38.67 17.57-30.32 13.55-68.51-9.94-94.51zm-120.28 168.11c-14.03.02-27.62-4.89-38.39-13.88.49-.26 1.34-.73 1.89-1.07l63.72-36.8c3.26-1.85 5.26-5.32 5.24-9.07v-89.83l26.93 15.55c.29.14.48.42.52.74v74.39c-.04 33.08-26.83 59.9-59.91 59.97zm-128.84-55.03c-7.03-12.14-9.56-26.37-7.15-40.18.47.28 1.3.79 1.89 1.13l63.72 36.8c3.23 1.89 7.23 1.89 10.47 0l77.79-44.92v31.1c.02.32-.13.63-.38.83l-64.41 37.19c-28.69 16.52-65.33 6.7-81.92-21.95zm-16.77-139.09c7-12.16 18.05-21.46 31.21-26.29 0 .55-.03 1.52-.03 2.2v73.61c-.02 3.74 1.98 7.21 5.23 9.06l77.79 44.91-26.93 15.55c-.27.18-.61.21-.91.08l-64.42-37.22c-28.63-16.58-38.45-53.21-21.95-81.89zm221.26 51.49-77.79-44.92 26.93-15.54c.27-.18.61-.21.91-.08l64.42 37.19c28.68 16.57 38.51 53.26 21.94 81.94-7.01 12.14-18.05 21.44-31.2 26.28v-75.81c.03-3.74-1.96-7.2-5.2-9.06zm26.8-40.34c-.47-.29-1.3-.79-1.89-1.13l-63.72-36.8c-3.23-1.89-7.23-1.89-10.47 0l-77.79 44.92v-31.1c-.02-.32.13-.63.38-.83l64.41-37.16c28.69-16.55 65.37-6.7 81.91 22 6.99 12.12 9.52 26.31 7.15 40.1zm-168.51 55.43-26.94-15.55c-.29-.14-.48-.42-.52-.74v-74.39c.02-33.12 26.89-59.96 60.01-59.94 14.01 0 27.57 4.92 38.34 13.88-.49.26-1.33.73-1.89 1.07l-63.72 36.8c-3.26 1.85-5.26 5.31-5.24 9.06l-.04 89.79zm14.63-31.54 34.65-20.01 34.65 20v40.01l-34.65 20-34.65-20z\"/></svg>",
      "variables": {},
      "input": {
        "method": "GET",
        "url": "http://example.com",
        "headers": "",
        "body": ""
      },
      "input_schema": {
        "type": "object",
        "properties": {
          "method": {
             "title": "Method",
              "description": "HTTP Method for request",
              "type": "string",
              "oneOf": [
                {
                  "value": "GET",
                  "title": "GET"
                },
                {
                  "value": "POST",
                  "title": "POST"
                },
                {
                  "value": "PUT",
                  "title": "PUT"
                },
                {
                  "value": "DELETE",
                  "title": "DELETE"
                }
              ],
              "x-jsf-presentation": {
                "inputType": "select"
              }
          },
          "url": {
             "title": "URL",
            "description": "URL for request",
            "type": "string"
          },
          "headers": {
            "title": "Headers",
            "description": "Headers for request",
            "type": "string"
          },
          "body": {
            "title": "Body",
            "description": "Body for request",
            "type": "string"
          }
        },
        "x-jsf-order": ["url", "method", "headers", "body"],
        "required": ["method", "url"],
        "additionalProperties": false
      },
      "presentation": {
        "position": {
          "x": 300,
          "y": 100
        }
      },
      "handles": [
        {
          "id": "a", 
          "type": "target",
          "position": "top"
        },
        {
          "id": "b",
          "type": "source",
          "position": "bottom"
        }
      ]
    }', now(), now(), '1e4f12a7-3c55-4e6d-9b4d-2a1f0403a2a6', '1e4f12a7-3c55-4e6d-9b4d-2a1f0403a2a6'); 


   -- Inserting sample secrets into marketplace.secrets
INSERT INTO vault.secrets (
    id, name, description, secret
) VALUES
    ('123e4567-e89b-12d3-a456-426614174000', 'API_KEY_1', 'silly description', 'SUPER_SECRET_KEY_1'),
    ('123e4567-e89b-12d3-a456-426614174001', 'API_KEY_2', 'silly description', 'SUPER_SECRET_KEY_2'),
    ('123e4567-e89b-12d3-a456-426614174002', 'API_KEY_3', 'silly description', 'SUPER_SECRET_KEY_3'),
    ('123e4567-e89b-12d3-a456-426614174003', 'API_KEY_4', 'silly description', 'SUPER_SECRET_KEY_4'),
    ('123e4567-e89b-12d3-a456-426614174004', 'API_KEY_5', 'silly description', 'SUPER_SECRET_KEY_5');

   -- Inserting sample secrets into anything.secrets
INSERT INTO anything.secrets (
    secret_id, account_id, secret_name, vault_secret_id, created_at, updated_at, created_by, updated_by
) VALUES
    ('123e4567-e89b-12d3-a456-426614174000', 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'API_KEY_1', '123e4567-e89b-12d3-a456-426614174000', now(), now(), '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8', '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8'),
    ('123e4567-e89b-12d3-a456-426614174001', '7df12345-a5d3-4b13-9e3a-2f5c3e6a7b91', 'API_KEY_2', '123e4567-e89b-12d3-a456-426614174001', now(), now(), '5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f', '5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f'),
    ('123e4567-e89b-12d3-a456-426614174002', 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'API_KEY_3', '123e4567-e89b-12d3-a456-426614174002', now(), now(), '1e4f12a7-3c55-4e6d-9b4d-2a1f0403a2a6', '1e4f12a7-3c55-4e6d-9b4d-2a1f0403a2a6'),
    ('123e4567-e89b-12d3-a456-426614174003', '7df12345-a5d3-4b13-9e3a-2f5c3e6a7b91', 'API_KEY_4', '123e4567-e89b-12d3-a456-426614174003', now(), now(), '3d8b144c-1e9d-4a8c-8234-4e5c9b3d5c2f', '3d8b144c-1e9d-4a8c-8234-4e5c9b3d5c2f'),
    ('123e4567-e89b-12d3-a456-426614174004', 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'API_KEY_5', '123e4567-e89b-12d3-a456-426614174004', now(), now(), '2a7b3d8e-2f3c-4b5d-8e3a-4a7c3e6a7c8d', '2a7b3d8e-2f3c-4b5d-8e3a-4a7c3e6a7c8d');