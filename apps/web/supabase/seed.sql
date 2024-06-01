-- -- Create Data for Marketplace

-- insert users into the auth.users table
-- Inserting sample users into auth.users
INSERT INTO auth.users (
    id, email, created_at, updated_at
) VALUES
    ('0c8d9e2f-3d4e-4a6d-9c5b-7d2e0402a7c8', 'user1@example.com', now(), now()),
    ('5e6f1234-b5d7-4e6b-9d3a-6a2e7c1b2a9f', 'user2@example.com', now(), now()),
    ('1e4f12a7-3c55-4e6d-9b4d-2a1f0403a2a6', 'user3@example.com', now(), now()),
    ('3d8b144c-1e9d-4a8c-8234-4e5c9b3d5c2f', 'user4@example.com', now(), now()),
    ('2a7b3d8e-2f3c-4b5d-8e3a-4a7c3e6a7c8d', 'user5@example.com', now(), now());

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

-- -- Insert into "tags" table
-- Insert sample tags into the tags table
INSERT INTO marketplace.tags (id, tag_uuid, tag_label, tag_slug, tag_icon, updated_at, created_at, updated_by, created_by)
VALUES
    ('school', uuid_generate_v4(), 'School', 'school', null, now(), now(), null, null),
    ('work', uuid_generate_v4(), 'Work', 'work', null, now(), now(), null, null),
    ('dev', uuid_generate_v4(), 'Development', 'dev', null, now(), now(), null, null),
    ('content', uuid_generate_v4(), 'Content', 'content', null, now(), now(), null, null);



-- -- Insert into "profiles" table
-- INSERT INTO marketplace.profiles (
--     id, 
--     account_id, 
--     username, 
--     full_name, 
--     avatar_url, 
--     website, 
--     twitter, 
--     tiktok, 
--     instagram, 
--     youtube, 
--     linkedin, 
--     github, 
--     public, 
--     bio, 
--     updated_at, 
--     created_at, 
--     updated_by, 
--     created_by
-- ) 
-- VALUES 
-- (
--     uuid_generate_v4(), 
--     'account_id_1', 
--     'username_1', 
--     'full_name_1', 
--     'avatar_url_1', 
--     'website_1', 
--     'twitter_1', 
--     'tiktok_1', 
--     'instagram_1', 
--     'youtube_1', 
--     'linkedin_1', 
--     'github_1', 
--     false, 
--     'bio_1', 
--     CURRENT_TIMESTAMP, 
--     CURRENT_TIMESTAMP, 
--     'updated_by_1', 
--     'created_by_1'
-- ),
-- (
--     uuid_generate_v4(), 
--     'account_id_2', 
--     'username_2', 
--     'full_name_2', 
--     'avatar_url_2', 
--     'website_2', 
--     'twitter_2', 
--     'tiktok_2', 
--     'instagram_2', 
--     'youtube_2', 
--     'linkedin_2', 
--     'github_2', 
--     false, 
--     'bio_2', 
--     CURRENT_TIMESTAMP, 
--     CURRENT_TIMESTAMP, 
--     'updated_by_2', 
--     'created_by_2'
-- ),
-- (
--     uuid_generate_v4(), 
--     'account_id_3', 
--     'username_3', 
--     'full_name_3', 
--     'avatar_url_3', 
--     'website_3', 
--     'twitter_3', 
--     'tiktok_3', 
--     'instagram_3', 
--     'youtube_3', 
--     'linkedin_3', 
--     'github_3', 
--     false, 
--     'bio_3', 
--     CURRENT_TIMESTAMP, 
--     CURRENT_TIMESTAMP, 
--     'updated_by_3', 
--     'created_by_3'
-- ),
-- (
--     uuid_generate_v4(), 
--     'account_id_4', 
--     'username_4', 
--     'full_name_4', 
--     'avatar_url_4', 
--     'website_4', 
--     'twitter_4', 
--     'tiktok_4', 
--     'instagram_4', 
--     'youtube_4', 
--     'linkedin_4', 
--     'github_4', 
--     false, 
--     'bio_4', 
--     CURRENT_TIMESTAMP, 
--     CURRENT_TIMESTAMP, 
--     'updated_by_4', 
--     'created_by_4'
-- ),
-- (
--     uuid_generate_v4(), 
--     'account_id_5', 
--     'username_5', 
--     'full_name_5', 
--     'avatar_url_5', 
--     'website_5', 
--     'twitter_5', 
--     'tiktok_5', 
--     'instagram_5', 
--     'youtube_5', 
--     'linkedin_5', 
--     'github_5', 
--     false, 
--     'bio_5', 
--     CURRENT_TIMESTAMP, 
--     CURRENT_TIMESTAMP, 
--     'updated_by_5', 
--     'created_by_5'
-- );

