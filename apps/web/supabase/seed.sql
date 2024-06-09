
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
    ('dddddddd-dddd-dddd-dddd-dddddddddddd', '7df12345-a5d3-4b13-9e3a-2f5c3e6a7b91', 'Flow 4', true, now(), now(), '3d8b144c-1e9d-4a8c-8234-4e5c9b3d5c2f', '3d8b144c-1e9d-4a8c-8234-4e5c9b3d5c2f'),
    ('eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee', 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'Flow 5', false, now(), now(), '2a7b3d8e-2f3c-4b5d-8e3a-4a7c3e6a7c8d', '2a7b3d8e-2f3c-4b5d-8e3a-4a7c3e6a7c8d');

-- Inserting sample flow versions into anything.flow_versions
INSERT INTO anything.flow_versions (
    flow_version_id, account_id, flow_id, flow_version, description, checksum, published, flow_definition, updated_at, created_at, updated_by, created_by
) VALUES
    ('11111111-1111-1111-1111-111111111111', 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa', 'v1.0', 'Initial version of Flow 1', 'checksum1', true, 
    '{
    "actions": [
    {
      "anything_action_version": "1.0.0",
      "type": "action",
      "plugin_id": "example_plugin_1",
      "plugin_version": "1.0.0",
      "label": "Example Action 1",
      "description": "This is the first example action",
      "icon": "<svg></svg>",
      "variables": {},
      "input": {
        "method": "GET",
        "url": "http://example.com",
        "headers": {},
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
            "type": "object"
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
          "x": 0,
          "y": 0
        }
      },
      "handles": [
        {
          "type": "target",
          "position": "left"
        },
        {
          "type": "source",
          "position": "right"
        }
      ]
    },
    {
      "anything_action_version": "1.0.0",
      "type": "action",
      "plugin_id": "example_plugin_2",
      "plugin_version": "1.0.0",
      "label": "Example Action 2",
      "description": "This is the second example action",
      "icon": "<svg></svg>",
      "variables": {},
      "input": {
        "method": "POST",
        "url": "http://example.com",
        "headers": {},
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
            "type": "object"
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
          "x": 200,
          "y": 0
        }
      },
      "handles": [
        {
          "type": "target",
          "position": "left"
        },
        {
          "type": "source",
          "position": "right"
        }
      ]
    },
    {
      "anything_action_version": "1.0.0",
      "type": "action",
      "plugin_id": "example_plugin_3",
      "plugin_version": "1.0.0",
      "label": "Example Action 3",
      "description": "This is the third example action",
      "icon": "<svg></svg>",
      "variables": {},
      "input": {
        "method": "PUT",
        "url": "http://example.com",
        "headers": {},
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
            "type": "object"
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
          "x": 400,
          "y": 0
        }
      },
      "handles": [
        {
          "type": "target",
          "position": "left"
        },
        {
          "type": "source",
          "position": "right"
        }
      ]
    }
  ],
  "edges": [
    {
      "id": "e1-2",
      "source": "1",
      "target": "2",
      "sourceHandle": "right",
      "targetHandle": "left"
    },
    {
      "id": "e2-3",
      "source": "2",
      "target": "3",
      "sourceHandle": "right",
      "targetHandle": "left"
    }
  ]
}', now(), now(), '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8', '0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8'),
    ('22222222-2222-2222-2222-222222222222', '7df12345-a5d3-4b13-9e3a-2f5c3e6a7b91', 'bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb', 'v1.0', 'Initial version of Flow 2', 'checksum2', true,  '{
    "actions": [
    {
      "anything_action_version": "1.0.0",
      "type": "action",
      "plugin_id": "example_plugin_1",
      "plugin_version": "1.0.0",
      "label": "Example Action 1",
      "description": "This is the first example action",
      "icon": "<svg></svg>",
      "variables": {},
      "input": {
        "method": "GET",
        "url": "http://example.com",
        "headers": {},
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
            "type": "object"
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
          "x": 0,
          "y": 0
        }
      },
      "handles": [
        {
          "type": "target",
          "position": "left"
        },
        {
          "type": "source",
          "position": "right"
        }
      ]
    },
    {
      "anything_action_version": "1.0.0",
      "type": "action",
      "plugin_id": "example_plugin_2",
      "plugin_version": "1.0.0",
      "label": "Example Action 2",
      "description": "This is the second example action",
      "icon": "<svg></svg>",
      "variables": {},
      "input": {
        "method": "POST",
        "url": "http://example.com",
        "headers": {},
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
            "type": "object"
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
          "x": 200,
          "y": 0
        }
      },
      "handles": [
        {
          "type": "target",
          "position": "left"
        },
        {
          "type": "source",
          "position": "right"
        }
      ]
    },
    {
      "anything_action_version": "1.0.0",
      "type": "action",
      "plugin_id": "example_plugin_3",
      "plugin_version": "1.0.0",
      "label": "Example Action 3",
      "description": "This is the third example action",
      "icon": "<svg></svg>",
      "variables": {},
      "input": {
        "method": "PUT",
        "url": "http://example.com",
        "headers": {},
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
            "type": "object"
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
          "x": 400,
          "y": 0
        }
      },
      "handles": [
        {
          "type": "target",
          "position": "left"
        },
        {
          "type": "source",
          "position": "right"
        }
      ]
    }
  ],
  "edges": [
    {
      "id": "e1-2",
      "source": "1",
      "target": "2",
      "sourceHandle": "right",
      "targetHandle": "left"
    },
    {
      "id": "e2-3",
      "source": "2",
      "target": "3",
      "sourceHandle": "right",
      "targetHandle": "left"
    }
  ]
}', now(), now(), '5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f', '5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f'),
    ('33333333-3333-3333-3333-333333333333', 'c9b8d2d5-3b12-4a6d-9eb2-1f6c7409b332', 'cccccccc-cccc-cccc-cccc-cccccccccccc', 'v1.0', 'Initial version of Flow 3', 'checksum3', true,  '{
    "actions": [
    {
      "anything_action_version": "1.0.0",
      "type": "action",
      "plugin_id": "example_plugin_1",
      "plugin_version": "1.0.0",
      "label": "Example Action 1",
      "description": "This is the first example action",
      "icon": "<svg></svg>",
      "variables": {},
      "input": {
        "method": "GET",
        "url": "http://example.com",
        "headers": {},
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
            "type": "object"
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
          "x": 0,
          "y": 0
        }
      },
      "handles": [
        {
          "type": "target",
          "position": "left"
        },
        {
          "type": "source",
          "position": "right"
        }
      ]
    },
    {
      "anything_action_version": "1.0.0",
      "type": "action",
      "plugin_id": "example_plugin_2",
      "plugin_version": "1.0.0",
      "label": "Example Action 2",
      "description": "This is the second example action",
      "icon": "<svg></svg>",
      "variables": {},
      "input": {
        "method": "POST",
        "url": "http://example.com",
        "headers": {},
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
            "type": "object"
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
          "x": 200,
          "y": 0
        }
      },
      "handles": [
        {
          "type": "target",
          "position": "left"
        },
        {
          "type": "source",
          "position": "right"
        }
      ]
    },
    {
      "anything_action_version": "1.0.0",
      "type": "action",
      "plugin_id": "example_plugin_3",
      "plugin_version": "1.0.0",
      "label": "Example Action 3",
      "description": "This is the third example action",
      "icon": "<svg></svg>",
      "variables": {},
      "input": {
        "method": "PUT",
        "url": "http://example.com",
        "headers": {},
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
            "type": "object"
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
          "x": 400,
          "y": 0
        }
      },
      "handles": [
        {
          "type": "target",
          "position": "left"
        },
        {
          "type": "source",
          "position": "right"
        }
      ]
    }
  ],
  "edges": [
    {
      "id": "e1-2",
      "source": "1",
      "target": "2",
      "sourceHandle": "right",
      "targetHandle": "left"
    },
    {
      "id": "e2-3",
      "source": "2",
      "target": "3",
      "sourceHandle": "right",
      "targetHandle": "left"
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