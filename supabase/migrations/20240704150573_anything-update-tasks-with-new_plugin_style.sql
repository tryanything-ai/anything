-- First, add the columns as nullable
ALTER TABLE anything.tasks
    ADD COLUMN plugin_name TEXT,
    ADD COLUMN plugin_version TEXT;

-- Update existing rows using plugin_id as plugin_name with the new naming format
UPDATE anything.tasks
SET plugin_name = CONCAT('@anything/', plugin_id),
    plugin_version = '0.1.0';

-- Now make the columns NOT NULL
ALTER TABLE anything.tasks
    ALTER COLUMN plugin_name SET NOT NULL,
    ALTER COLUMN plugin_version SET NOT NULL;

-- Finally drop the old column
ALTER TABLE anything.tasks
    DROP COLUMN plugin_id;
