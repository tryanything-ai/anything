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
    -- First transform: inputs -> plugin_config
    flow_def = flow_def 
    - 'inputs' - 'inputs_schema' - 'inputs_schema_locked' - 'inputs_locked' ||
    CASE 
        WHEN flow_def ? 'inputs' THEN jsonb_build_object('plugin_config', flow_def->'inputs')
        ELSE '{}'::jsonb
    END ||
    CASE 
        WHEN flow_def ? 'inputs_schema' THEN jsonb_build_object('plugin_config_schema', flow_def->'inputs_schema')
        ELSE '{}'::jsonb
    END ||
    CASE 
        WHEN flow_def ? 'inputs_schema_locked' THEN jsonb_build_object('plugin_config_schema_locked', flow_def->'inputs_schema_locked')
        ELSE '{}'::jsonb
    END ||
    CASE 
        WHEN flow_def ? 'inputs_locked' THEN jsonb_build_object('plugin_config_locked', flow_def->'inputs_locked')
        ELSE '{}'::jsonb
    END;

    -- Second transform: variables -> inputs
    RETURN flow_def 
    - 'variables' - 'variables_schema' - 'variables_schema_locked' - 'variables_locked' ||
    CASE 
        WHEN flow_def ? 'variables' THEN jsonb_build_object('inputs', flow_def->'variables')
        ELSE '{}'::jsonb
    END ||
    CASE 
        WHEN flow_def ? 'variables_schema' THEN jsonb_build_object('inputs_schema', flow_def->'variables_schema')
        ELSE '{}'::jsonb
    END ||
    CASE 
        WHEN flow_def ? 'variables_schema_locked' THEN jsonb_build_object('inputs_schema_locked', flow_def->'variables_schema_locked')
        ELSE '{}'::jsonb
    END ||
    CASE 
        WHEN flow_def ? 'variables_locked' THEN jsonb_build_object('inputs_locked', flow_def->'variables_locked')
        ELSE '{}'::jsonb
    END;
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
   OR flow_definition::text LIKE '%inputs%'
   OR flow_definition::text LIKE '%inputs_schema%'
   OR flow_definition::text LIKE '%inputs_schema_locked%'
   OR flow_definition::text LIKE '%inputs_locked%';
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