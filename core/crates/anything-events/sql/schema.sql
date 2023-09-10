CREATE TABLE IF NOT EXISTS events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source_id SERIAL,
    event_name TEXT NOT NULL,
    payload json NOT NULL,
    metadata json NOT NULL,
    timestamp timestamp with time zone DEFAULT (CURRENT_TIMESTAMP)
);
-- CREATE TABLE IF NOT EXISTS event_tags (
--     event_id INTEGER NOT NULL,
--     tag_id INTEGER NOT NULL
-- );
-- CREATE TABLE IF NOT EXISTS tags (
--     id INTEGER PRIMARY KEY AUTOINCREMENT,
--     name TEXT NOT NULL
-- );
-- CREATE TABLE IF NOT EXISTS 
-- CREATE TABLE IF NOT EXISTS scheduler (
--     id UUID PRIMARY KEY,
--     last_seen_datetime TIMESTAMP WITH TIME ZONE NOT NULL,
--     queued_triggers INT,
--     -- waiting_for_trigger_id UUID REFERENCES trigger(id),
--     version VARCHAR
-- );
-- CREATE TABLE IF NOT EXISTS trigger (
--     id UUID PRIMARY KEY,
--     name VARCHAR NOT NULL,
--     job_id UUID NOT NULL REFERENCES job(id),
--     start_datetime TIMESTAMP WITH TIME ZONE NOT NULL,
--     end_datetime TIMESTAMP WITH TIME ZONE,
--     earliest_trigger_datetime TIMESTAMP WITH TIME ZONE,
--     latest_trigger_datetime TIMESTAMP WITH TIME ZONE,
--     period BIGINT,
--     cron VARCHAR,
--     trigger_offset BIGINT,
--     catchup VARCHAR NOT NULL,
--     UNIQUE(job_id, name) INCLUDE (id)
-- );
-- CREATE TABLE IF NOT EXISTS task (
--     id UUID PRIMARY KEY,
--     name VARCHAR NOT NULL,
--     job_id UUID NOT NULL REFERENCES job(id),
--     threshold INT,
--     retry_max_attempts INT,
--     retry_delay_secs BIGINT,
--     timeout_secs BIGINT,
--     image VARCHAR,
--     args VARCHAR [],
--     env VARCHAR [],
--     UNIQUE(job_id, name) INCLUDE (id)
-- );