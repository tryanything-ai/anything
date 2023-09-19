-- Add migration script here
CREATE TABLE IF NOT EXISTS events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_id TEXT NOT NULL,
    source_id TEXT NOT NULL,
    event_name TEXT NOT NULL,
    payload json NOT NULL,
    metadata json,
    timestamp timestamp with time zone DEFAULT (CURRENT_TIMESTAMP)
);