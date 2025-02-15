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
                    WHEN action ? 'variables' OR action ? 'input' THEN
                        action 
                        - 'input' - 'input_schema' - 'input_schema_locked' - 'input_locked'
                        - 'variables' - 'variables_schema' - 'variables_schema_locked' - 'variables_locked'
                        ||
                        CASE 
                            WHEN action ? 'input' THEN jsonb_build_object('plugin_config', action->'input')
                            ELSE '{}'::jsonb
                        END ||
                        CASE 
                            WHEN action ? 'input_schema' THEN jsonb_build_object('plugin_config_schema', action->'input_schema')
                            ELSE '{}'::jsonb
                        END ||
                        CASE 
                            WHEN action ? 'input_schema_locked' THEN jsonb_build_object('plugin_config_schema_locked', action->'input_schema_locked')
                            ELSE '{}'::jsonb
                        END ||
                        CASE 
                            WHEN action ? 'input_locked' THEN jsonb_build_object('plugin_config_locked', action->'input_locked')
                            ELSE '{}'::jsonb
                        END ||
                        CASE 
                            WHEN action ? 'variables' THEN jsonb_build_object('inputs', action->'variables')
                            ELSE '{}'::jsonb
                        END ||
                        CASE 
                            WHEN action ? 'variables_schema' THEN jsonb_build_object('inputs_schema', action->'variables_schema')
                            ELSE '{}'::jsonb
                        END ||
                        CASE 
                            WHEN action ? 'variables_schema_locked' THEN jsonb_build_object('inputs_schema_locked', action->'variables_schema_locked')
                            ELSE '{}'::jsonb
                        END ||
                        CASE 
                            WHEN action ? 'variables_locked' THEN jsonb_build_object('inputs_locked', action->'variables_locked')
                            ELSE '{}'::jsonb
                        END
                    ELSE action
                END
            )
            FROM jsonb_array_elements(flow_def->'actions') action
        )
    );
END;
$$ LANGUAGE plpgsql;

-- Update the flow_definition column in flow_versions without triggering timestamp updates
SET session_replication_role = replica;
UPDATE anything.flow_versions 
SET flow_definition = anything.transform_flow_definition(flow_definition)
WHERE flow_definition::text LIKE '%variables%'
   OR flow_definition::text LIKE '%variables_schema%'
   OR flow_definition::text LIKE '%variables_schema_locked%'
   OR flow_definition::text LIKE '%variables_locked%'
   OR flow_definition::text LIKE '%input%'
   OR flow_definition::text LIKE '%input_schema%'
   OR flow_definition::text LIKE '%input_schema_locked%'
   OR flow_definition::text LIKE '%input_locked%';
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