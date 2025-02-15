-- First, create a regular table for any potential errors
CREATE TABLE IF NOT EXISTS anything.flow_version_migration_errors (
    flow_version_id UUID,
    error_message TEXT,
    flow_definition JSONB
);

-- Alter the column type from JSON to JSONB if not already
ALTER TABLE anything.flow_versions 
ALTER COLUMN flow_definition TYPE JSONB USING flow_definition::JSONB;

-- Create the transformation function
CREATE OR REPLACE FUNCTION anything.transform_flow_definition(flow_def JSONB) 
RETURNS JSONB AS $$
DECLARE
    transformed_json TEXT;
BEGIN
    transformed_json = flow_def::text;
    transformed_json = REPLACE(transformed_json, '{{variables.', '{{inputs.');
    RETURN transformed_json::jsonb;
END;
$$ LANGUAGE plpgsql;

-- Update the flow_definition column in flow_versions
SET session_replication_role = replica; -- This is to prevent the migration from locking the table and prevents triggers from updating updated_at 
UPDATE anything.flow_versions 
SET flow_definition = anything.transform_flow_definition(flow_definition)
WHERE flow_definition::text LIKE '%{{variables.%';
SET session_replication_role = DEFAULT;

-- Check for errors
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM anything.flow_version_migration_errors) THEN
        RAISE NOTICE 'Migration completed with errors. Check flow_version_migration_errors table.';
    ELSE
        RAISE NOTICE 'Migration completed successfully.';
    END IF;
END $$;

-- Drop the function after we're done
DROP FUNCTION anything.transform_flow_definition;

-- Drop the error table if not needed
-- DROP TABLE anything.flow_version_migration_errors; 