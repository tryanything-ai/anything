ALTER TABLE anything.tasks
    DROP COLUMN plugin_id,
    ADD COLUMN plugin_name TEXT NOT NULL,
    ADD COLUMN plugin_version TEXT NOT NULL;
