use std::{thread, time::Duration};

use tauri::{AppHandle, Manager};
use crate::sql::plugin::{select, DbInstances, Error, DB_STRING};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

pub async fn task_to_run_every_minute(app: AppHandle) {
    loop {

        println!("Hello, world from taks_to_run_every_minute!");
      
        let res = fetch_event(app.clone()).await; 
        println!("res: {:?}", res);
        // let mut events = select(DB_STRING).expect("Failed to select events");
        // Sleep for a minute
        thread::sleep(Duration::from_secs(1));
    }
}

async fn fetch_event<R: tauri::Runtime>(
    app: AppHandle<R>,
) -> std::result::Result<Vec<HashMap<String, JsonValue>>, Error> {
    // Access the dbInstances from the app's state
    // let app = AppHandle;
    let sql = "SELECT * FROM events WHERE status = $1 ORDER BY created_at ASC LIMIT 1".to_string(); 
    let db = DB_STRING.to_string();
    let values = vec![JsonValue::String("PENDING".to_string())];
    let db_instances = app.state::<DbInstances>(); 

    // Call the select function with the fetched dbInstances state
    select(db_instances, db, sql, values).await
}

// async fn mark_as_done() {
//     let db_instances = DbInstances::default();
//     let db = "your_db_string".to_string();
//     let query = "your_query_string".to_string();
//     let values = vec![];

//     match execute(db_instances, db, query, values).await {
//         Ok((affected_rows, last_insert_id)) => {
//             println!("Affected rows: {}", affected_rows);
//             println!("Last insert ID: {}", last_insert_id);
//         }
//         Err(e) => {
//             println!("Error executing the query: {:?}", e);
//         }
//     }
// }


// Thoughts on events based architefture
//https://discord.com/channels/616186924390023171/731495028677148753/1133165388981620837