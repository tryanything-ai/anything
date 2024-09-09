-- Create the task_billing table
CREATE TABLE IF NOT EXISTS anything.tasks_billing
(
    task_id uuid NOT NULL PRIMARY KEY REFERENCES anything.tasks(task_id),
    account_id uuid NOT NULL REFERENCES basejump.accounts(id),
    execution_time interval,
    execution_time_ms bigint,
    task_status text,
    usage_reported_to_billing_provider boolean DEFAULT false,
    
    created_at timestamp with time zone,
    updated_at timestamp with time zone
);

-- Create a function to insert or update the task_billing record
CREATE OR REPLACE FUNCTION anything.update_task_billing()
RETURNS TRIGGER AS $$
BEGIN
    -- Insert or update the task_billing record
    INSERT INTO anything.tasks_billing (
        task_id, 
        account_id, 
        execution_time, 
        execution_time_ms,
        task_status,
        created_at, 
        updated_at
    )
    VALUES (
        NEW.task_id, 
        NEW.account_id, 
        CASE 
            WHEN NEW.ended_at IS NOT NULL AND NEW.started_at IS NOT NULL 
            THEN NEW.ended_at - NEW.started_at
            ELSE NULL
        END,
        CASE 
            WHEN NEW.ended_at IS NOT NULL AND NEW.started_at IS NOT NULL 
            THEN EXTRACT(EPOCH FROM (NEW.ended_at - NEW.started_at)) * 1000
            ELSE NULL
        END,
        NEW.task_status,
        NOW(), 
        NOW()
    )
    ON CONFLICT (task_id) 
    DO UPDATE SET
        account_id = NEW.account_id,
        execution_time = CASE 
            WHEN NEW.ended_at IS NOT NULL AND NEW.started_at IS NOT NULL 
            THEN NEW.ended_at - NEW.started_at
            ELSE NULL
        END,
        execution_time_ms = CASE 
            WHEN NEW.ended_at IS NOT NULL AND NEW.started_at IS NOT NULL 
            THEN EXTRACT(EPOCH FROM (NEW.ended_at - NEW.started_at)) * 1000
            ELSE NULL
        END,
        task_status = NEW.task_status,
        updated_at = NOW();

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create a trigger to call the function
CREATE TRIGGER update_task_billing_trigger
AFTER INSERT OR UPDATE ON anything.tasks
FOR EACH ROW
EXECUTE FUNCTION anything.update_task_billing();

-- protect the timestamps by setting created_at and updated_at to be read-only and managed by a trigger
CREATE TRIGGER set_tasks_billing_timestamp
    BEFORE INSERT OR UPDATE ON anything.tasks_billing
    FOR EACH ROW
EXECUTE PROCEDURE basejump.trigger_set_timestamps();


-- enable RLS on the table
ALTER TABLE anything.tasks_billing ENABLE ROW LEVEL SECURITY;