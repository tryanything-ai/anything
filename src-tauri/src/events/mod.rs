use tokio::time::{sleep, Duration};
use tauri::{
    AppHandle, Runtime, Manager
};

use std::{collections::{HashMap, VecDeque}, fs};

use crate::sql::plugin::{select, DbInstances, DB_STRING, execute, Error};
use serde_json::Value as JsonValue;

use tauri::api::path::document_dir;

use std::io::{Result, Error as IOError, ErrorKind};
use uuid::Uuid;

#[derive(Clone, serde::Serialize)]
 struct Payload {
  message: String,
  name: String
}

pub async fn scheduler<R: Runtime>(app: &AppHandle<R>){
    loop {
        let app_handle = app.clone(); 

        tokio::spawn(async move {
            process(&app_handle).await;
        });

       sleep(Duration::from_secs(5)).await; 
    }
}

async fn process<R: Runtime>(app: &AppHandle<R>) {

    let res = fetch_event(app).await; 
    
    let sql_event_id: &str;

    match res {
        Ok(items) => {
            if let Some(item) = items.get(0) {
                if let Some(event_id) = item.get("event_id") {
                    
                    //TODO: let frontend know wtf is going on
                      // emit the `event-name` event to all webview windows on the frontend
                    //     app.emit_all("current_task", 
                    //     Payload { name: flow_name.to_string(),  message: "Tauri is awesome! it automated my work!".into() })
                    //     .unwrap();
                   
                    if let Some(worker_type) = item.get("worker_type") {
                        sql_event_id = event_id.as_str().unwrap();

                        if worker_type != "start" {
                            println!("Worker type is not Start. Doing Work."); 
                            //TODO: if node type is "START" create the other events
                            mark_as_done(app, sql_event_id.to_string()).await;
                            println!("event_id: {} marked as COMPLETE", event_id);
                        } else {
                            println!("Worker type is START. Determining what to build"); 
                            if let Some(flow_name_value) = item.get("flow_name") {
                                if let Some(flow_name_str) = flow_name_value.as_str() {
                                    // Assuming create_events_from_graph expects a &str
                                    create_events_from_graph(app,flow_name_str).await;
                                    //Mark the "start" event as done. 
                                    mark_as_done(app, sql_event_id.to_string()).await;                         
                                    println!("event_id: {} marked a START event COMPLETE", event_id);
                                } else {
                                    // Handle the case where the flow_name is not a string.
                                }
                            }
                        }
                        
                    } else {
                        println!("event_name not found in the item.");
                    }
                } else { 
                    println!("event_id not found in the item.");
                }
            } else {
                println!("No items in the response.");
            }
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}

async fn fetch_event<R: tauri::Runtime>(
    app: &AppHandle<R>,
) -> std::result::Result<Vec<HashMap<String, JsonValue>>, Error> {
    // Access the dbInstances from the app's state
    let db_instances = app.state::<DbInstances>(); 
    //make Query
    let db = DB_STRING.to_string();
    let query = "SELECT * FROM events WHERE event_status = $1 ORDER BY created_at ASC LIMIT 1".to_string(); 
    let values = vec![JsonValue::String("PENDING".to_string())];
    
    println!("Fetched Event"); 
    // Call the select function with the fetched dbInstances state
    select(db_instances, db, query, values).await
}

// async fn create_event<R: tauri::Runtime>(
//     app: &AppHandle<R>,
//     event_name: &str,
//     event_status: &str,
//     created_at: &str, // or whatever the type for created_at is in your context
// ) -> std::result::Result<(u64, i64), Error>{
//     // Access the dbInstances from the app's state
//     let db_instances = app.state::<DbInstances>();
    
//     // Construct the insert query
//     let db = DB_STRING.to_string();
//     let query = "INSERT INTO events (event_name, event_status, created_at) VALUES ($1, $2, $3)".to_string();
//     let values = vec![
//         JsonValue::String(event_name.to_string()),
//         JsonValue::String(event_status.to_string()),
//         JsonValue::String(created_at.to_string())
//     ];

//     println!("Creating Event"); 

//     // Call the insert function with the fetched dbInstances state
//     // Note: I'm assuming you have a function called insert similar to select. Adjust if different.
//     execute(db_instances, db, query, values).await
// }

async fn create_event<R: tauri::Runtime>(
    app: &AppHandle<R>,
    node: &HashMap<String, JsonValue>,
    flow_info: &HashMap<String, JsonValue>, // Assuming you have some flow-specific data
) -> std::result::Result<(), Error> {
    let db_instances = app.state::<DbInstances>(); 

    let db = DB_STRING.to_string();

    // Extract node details and other required information
    let node_id = node.get("id").and_then(|v| v.as_str()).unwrap_or_default();
    let node_type = node.get("type").and_then(|v| v.as_str()).unwrap_or_default();
    let worker_type = node.get("data")
                          .and_then(|data| data.get("worker_type"))
                          .and_then(|wt| wt.as_str())
                          .unwrap_or_default();
    // ... (Add other data extraction as needed) ...

    // Flow specific info (adjust as per your requirement)
    let flow_id = flow_info.get("flow_id").and_then(|v| v.as_str()).unwrap_or_default();
    let flow_name = flow_info.get("flow_name").and_then(|v| v.as_str()).unwrap_or_default();
    let flow_version = flow_info.get("flow_version").and_then(|v| v.as_str()).unwrap_or_default();
    // ... (Add other data extraction as needed) ...

    let query = "
        INSERT INTO events (event_id, session_id, node_id, node_type, flow_id, flow_name, flow_version, stage, worker_type, event_status, session_status, created_at, data) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
    ";

    let values = vec![
        JsonValue::String(Uuid::new_v4().to_string()), // event_id
        JsonValue::String(Uuid::new_v4().to_string()), // session_id
        JsonValue::String(node_id.to_string()),              // node_id
        JsonValue::String(node_type.to_string()),            // node_type
        JsonValue::String(flow_id.to_string()),              // flow_id
        JsonValue::String(flow_name.to_string()),            // flow_name
        JsonValue::String(flow_version.to_string()),         // flow_version
        // ... (Add other values as needed) ...
    ];

    match execute(db_instances, db, query.to_string(), values).await {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("Error adding event to db: {}", e);
            Err(e)          
        },
    }
}

async fn mark_as_done<R: tauri::Runtime>(
    app: &AppHandle<R>,
    event_id: String,
) {
    let db_instances = app.state::<DbInstances>(); 

    let db = DB_STRING.to_string();
    let query = "UPDATE events
    SET event_status = 'COMPLETE'
    WHERE event_id = $1".to_string(); 
    let values = vec![JsonValue::String(event_id)];

    match execute(db_instances, db, query, values).await {
        Ok((affected_rows, last_insert_id)) => {
            println!("Affected rows: {}", affected_rows);
            println!("Last insert ID: {}", last_insert_id);
        }
        Err(e) => {
            println!("Error executing the query: {:?}", e);
        }
    }
}

async fn create_events_from_graph<R: tauri::Runtime>(app: &AppHandle<R>, file_name: &str){

     let toml_document = read_from_documents(file_name).unwrap(); 

      // Convert TOML to serde_json::Value
      let parsed_toml: JsonValue = toml::from_str(&toml_document).expect("Failed to parse TOML");

      println!("{}", parsed_toml); 
      // Convert parsed TOML into JSON Value
      let json_data = serde_json::to_value(parsed_toml).expect("Failed to convert to JSON");

      let work_order = bfs_traversal(&json_data);
      //We now have all the events but including the start event. 

      println!("Found {} pieces of work to build out", work_order.len()); 

      //this loop skips the first item
      for work in work_order.iter().skip(1){
        println!("{}", work); 
        // create_event(app, work,)
        //SKip first one -> its the start node. don't make it again. 
        //make events via sql in the rest of the tasks. 
        //TODO: make event
        // create_event(app, event_name, event_status, created_at); 
        
       
       
       
        //TODO make new events for each thing. //TODO: determine how this works with "decisions"
        //TODO: would rather do this "JIT" so that decision nodes "results" can be taken into context. 
          println!("ID: {} is created as the next item in the work order", work.get("id").unwrap());
      }
}

fn bfs_traversal(json_data: &JsonValue) -> Vec<JsonValue> {
    // Resultant list of work
    let mut work_list = Vec::new();

    // Create a map of node ids to their outgoing edges
    let mut graph = std::collections::HashMap::new();
    if let Some(edges) = json_data.get("edges") {
        for edge in edges.as_array().unwrap() {
            let source = edge.get("source").unwrap().as_str().unwrap();
            let target = edge.get("target").unwrap().as_str().unwrap();

            graph.entry(source.to_string()).or_insert_with(Vec::new).push(target.to_string());
        }
    }

    // Use a BFS queue
    let mut queue = VecDeque::new();

    // Find and enqueue the node with "data.worker_type" = "start"
    if let Some(nodes) = json_data.get("nodes") {
        for node in nodes.as_array().unwrap() {
            if let Some(data) = node.get("data") {
                if data.get("worker_type").map_or(false, |w| w.as_str().unwrap_or("") == "start") {
                    queue.push_back(node.clone());
                    break;  // Since there should be only one start node based on the context
                }
            }
        }
    }

    // BFS traversal
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();

        // Add current node to the work list
        work_list.push(current.clone());

        // Enqueue neighbors
        if let Some(neighbors) = graph.get(current.get("id").unwrap().as_str().unwrap()) {
            if let Some(nodes) = json_data.get("nodes") {
                for neighbor_id in neighbors {
                    let neighbor = nodes.as_array().unwrap().iter().find(|&node| node.get("id").unwrap().as_str().unwrap() == neighbor_id).unwrap().clone();
                    queue.push_back(neighbor);
                }
            }
        }
    }

    work_list
}

fn read_from_documents(flow_name: &str) -> Result<String> {
    // Get the user's Documents directory
    let mut path = document_dir()
        .ok_or_else(|| IOError::new(ErrorKind::NotFound, "Documents directory not found"))?;
    
    // Append the required subdirectories and filename
    path.push("Anything");
    path.push("flows");
    path.push(flow_name); 
    path.push("flow.toml");

    // Check if the file exists
    if !path.exists() {
        return Err(IOError::new(ErrorKind::NotFound, format!("File not found at {:?}", path)));
    }

    // Read and return the file's contents
    fs::read_to_string(path)
}


// Thoughts on events based architefture
//https://discord.com/channels/616186924390023171/731495028677148753/1133165388981620837

// Inspiration 
// https://tokio.rs/tokio/tutorial/shared-state