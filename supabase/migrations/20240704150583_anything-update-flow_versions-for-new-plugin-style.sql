-- First, create a temporary table to log errors
CREATE TEMPORARY TABLE flow_version_migration_errors (
    flow_version_id UUID,
    error_message TEXT,
    flow_definition JSONB
);

-- Create the transformation function with error handling
CREATE OR REPLACE FUNCTION anything.transform_flow_definition(
    flow_version_id UUID,
    flow_def JSONB
) RETURNS JSONB AS $$
BEGIN
    -- Check for null
    IF flow_def IS NULL THEN
        INSERT INTO flow_version_migration_errors (flow_version_id, error_message, flow_definition)
        VALUES (flow_version_id, 'Flow definition is NULL', NULL);
        RETURN flow_def;
    END IF;

    -- Check for actions array
    IF flow_def->'actions' IS NULL OR jsonb_typeof(flow_def->'actions') != 'array' THEN
        INSERT INTO flow_version_migration_errors (flow_version_id, error_message, flow_definition)
        VALUES (flow_version_id, 'No actions array found or invalid type', flow_def);
        RETURN flow_def;
    END IF;

    -- Transform each action in the actions array
    RETURN jsonb_set(
        flow_def,
        '{actions}',
        (
            SELECT jsonb_agg(
                CASE 
                    WHEN action->>'plugin_id' IS NULL THEN
                        -- Log error for actions missing plugin_id
                        (SELECT action FROM (
                            INSERT INTO flow_version_migration_errors (flow_version_id, error_message, flow_definition)
                            VALUES (flow_version_id, 'Action missing plugin_id', action)
                        ) _ RETURNING action)
                    ELSE
                        -- Perform the transformation: only update plugin_name, preserve existing plugin_version
                        action - 'plugin_id' || 
                        jsonb_build_object(
                            'plugin_name', '@anything/' || (action->>'plugin_id')
                        )
                END
            )
            FROM jsonb_array_elements(flow_def->'actions') action
        )
    );
EXCEPTION WHEN OTHERS THEN
    -- Log any other errors
    INSERT INTO flow_version_migration_errors (flow_version_id, error_message, flow_definition)
    VALUES (flow_version_id, SQLERRM, flow_def);
    RETURN flow_def;
END;
$$ LANGUAGE plpgsql;

-- Update the flow_definition column in flow_versions
UPDATE anything.flow_versions
SET flow_definition = anything.transform_flow_definition(id, flow_definition)
WHERE flow_definition::text LIKE '%plugin_id%';

-- Check for errors
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM flow_version_migration_errors) THEN
        RAISE NOTICE 'Migration completed with errors. Check flow_version_migration_errors table.';
    ELSE
        RAISE NOTICE 'Migration completed successfully.';
    END IF;
END $$;

-- Drop the temporary function
DROP FUNCTION anything.transform_flow_definition;

-- Keep the error table for review
-- DROP TABLE flow_version_migration_errors; 