CREATE TABLE events (
    id BIGINT,
    name TEXT NOT NULL,
    payload json NOT NULL,
    metadata json NOT NULL,
    tags json NOT NULL,
    timestamp timestamp with time zone DEFAULT (CURRENT_TIMESTAMP),
    PRIMARY KEY (id)
);