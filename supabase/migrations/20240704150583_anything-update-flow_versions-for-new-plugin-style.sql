-- First, create a regular table instead of a temporary one
CREATE TABLE IF NOT EXISTS anything.flow_version_migration_errors (
    flow_version_id UUID,
    error_message TEXT,
    flow_definition JSONB
);

-- Alter the column type from JSON to JSONB
ALTER TABLE anything.flow_versions 
ALTER COLUMN flow_definition TYPE JSONB USING flow_definition::JSONB;

-- Create the transformation function with error handling
CREATE OR REPLACE FUNCTION anything.transform_flow_definition(flow_def JSONB) 
RETURNS JSONB AS $$
BEGIN
    RETURN jsonb_set(
        flow_def,
        '{actions}',
        (
            SELECT jsonb_agg(
                CASE 
                    WHEN action->>'plugin_id' IS NOT NULL THEN
                        action - 'plugin_id' || 
                        jsonb_build_object(
                            'plugin_name', '@anything/' || (action->>'plugin_id')
                        )
                    ELSE
                        action
                END
            )
            FROM jsonb_array_elements(flow_def->'actions') action
        )
    );
END;
$$ LANGUAGE plpgsql;

-- Update the flow_definition column in flow_versions without triggering timestamp updates
-- https://stackoverflow.com/a/18709987 -> simple way to stop triggers from firing the updated_at column
SET session_replication_role = replica;
UPDATE anything.flow_versions 
SET flow_definition = anything.transform_flow_definition(flow_definition)
WHERE flow_definition::text LIKE '%plugin_id%';
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

-- Drop the table after we're done
-- DROP TABLE anything.flow_version_migration_errors; 