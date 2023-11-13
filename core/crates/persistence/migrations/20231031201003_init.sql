CREATE TABLE IF NOT EXISTS triggers (
    trigger_id TEXT NOT NULL PRIMARY KEY,
    -- /file/created/<file-path> or /whatsapp/message/<message-id>
    event_name TEXT NOT NULL,
    payload json NOT NULL,
    metadata json,
    timestamp timestamp with time zone DEFAULT (CURRENT_TIMESTAMP)
);
CREATE TABLE IF NOT EXISTS events (
    id TEXT NOT NULL PRIMARY KEY,
    -- Not going to have both
    flow_id TEXT,
    trigger_id TEXT,
    name TEXT NOT NULL,
    context json NOT NULL,
    started_at timestamp with time zone DEFAULT (CURRENT_TIMESTAMP),
    ended_at timestamp with time zone DEFAULT (CURRENT_TIMESTAMP)
);
CREATE TABLE IF NOT EXISTS flows (
    flow_id TEXT PRIMARY KEY NOT NULL,
    flow_name TEXT NOT NULL,
    latest_version_id TEXT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT FALSE,
    updated_at timestamp with time zone DEFAULT (CURRENT_TIMESTAMP),
    UNIQUE (flow_name)
);
CREATE TABLE IF NOT EXISTS flow_versions (
    flow_id TEXT NOT NULL,
    flow_version_id TEXT NOT NULL,
    flow_version TEXT NOT NULL,
    description TEXT,
    checksum TEXT,
    updated_at timestamp with time zone DEFAULT (CURRENT_TIMESTAMP),
    published BOOLEAN NOT NULL DEFAULT FALSE,
    flow_definition json NOT NULL,
    UNIQUE (flow_id, flow_version)
);
-- CREATE TABLE IF NOT EXISTS nodes (
--     node_id TEXT PRIMARY KEY NOT NULL,
--     flow_id TEXT NOT NULL,
--     node_type TEXT NOT NULL,
--     node_name TEXT NOT NULL,
--     node_description TEXT NOT NULL,
--     node_config json NOT NULL,
--     node_definition json NOT NULL,
--     UNIQUE (flow_id, node_name) };
-- );