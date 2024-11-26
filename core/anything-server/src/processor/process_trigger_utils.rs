use serde_json::Value;

use crate::task_types::Task;

pub fn process_trigger_task(
    task: &Task,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    println!("[PROCESS TRIGGER TASK] Processing trigger task");

    //Return the result we created in the trigger
    Ok(task.result.clone())
}
