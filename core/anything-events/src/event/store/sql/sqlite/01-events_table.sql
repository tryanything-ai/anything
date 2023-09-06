CREATE TABLE events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_name TEXT NOT NULL,
    payload json NOT NULL,
    metadata json NOT NULL,
    tags json NOT NULL,
    timestamp timestamp with time zone DEFAULT (CURRENT_TIMESTAMP)
);