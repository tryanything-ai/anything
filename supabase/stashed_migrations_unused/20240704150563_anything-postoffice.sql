-- Create the postoffice table for logging API traffic
CREATE TABLE IF NOT EXISTS anything.postoffice (
    postoffice_id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    direction VARCHAR(10) NOT NULL CHECK (direction IN ('inbound', 'outbound')),
    method VARCHAR(10) NOT NULL,
    url TEXT NOT NULL,
    request_headers JSONB,
    request_body JSONB,
    response_status INTEGER,
    response_headers JSONB,
    response_body JSONB,
    duration_ms INTEGER,
    --Extra stats we may or may not use
    total_bytes BIGINT,
    time_to_first_byte_ms INTEGER,
    user_agent TEXT,
    origin TEXT,
    referer TEXT,
    content_type TEXT,
    content_length INTEGER,

    client_ip VARCHAR(45),
    -- Lots of optionall enrichment we may or may not do depending on how this evolves
    account_id UUID REFERENCES basejump.accounts(id),
    trigger_session_id UUID,
    flow_session_id UUID,
    flow_id UUID REFERENCES anything.flows(flow_id),
    flow_version_id UUID REFERENCES anything.flow_versions(flow_version_id),
    action_id TEXT,
    action_label TEXT,
    action_type TEXT,
    action_status TEXT,
    plugin_id TEXT,
    plugin_version TEXT,
    stage TEXT,

    -- Standard tracking columns
    updated_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE,
    updated_by UUID REFERENCES auth.users(id),
    created_by UUID REFERENCES auth.users(id)
);

-- Add timestamp trigger
CREATE TRIGGER set_postoffice_timestamp
    BEFORE INSERT OR UPDATE ON anything.postoffice
    FOR EACH ROW
EXECUTE PROCEDURE basejump.trigger_set_timestamps();

-- Add user tracking trigger
CREATE TRIGGER set_postoffice_user_tracking
    BEFORE INSERT OR UPDATE ON anything.postoffice
    FOR EACH ROW
EXECUTE PROCEDURE basejump.trigger_set_user_tracking();

-- Enable RLS
ALTER TABLE anything.postoffice ENABLE ROW LEVEL SECURITY;

-- Add select policy for authenticated users
CREATE POLICY "Authenticated users can view postoffice logs" ON anything.postoffice
    FOR SELECT
    TO authenticated
    USING (true);
