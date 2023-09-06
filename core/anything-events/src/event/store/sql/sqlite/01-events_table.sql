CREATE TABLE events (
    id BIGINT,
    event_name TEXT NOT NULL,
    payload json NOT NULL,
    metadata json NOT NULL,
    tags json NOT NULL,
    timestamp timestamp with time zone DEFAULT (CURRENT_TIMESTAMP),
    PRIMARY KEY (id)
);