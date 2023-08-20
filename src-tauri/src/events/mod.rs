use tokio::time::{sleep, Duration};
use tauri::{
    AppHandle, Runtime, Manager
};

use std::{collections::{HashMap, VecDeque}, fs};

use crate::sql::plugin::{select, DbInstances, DB_STRING, execute, Error};
use serde_json::Value as JsonValue;

use tauri::api::path::document_dir;

use std::io::{Result, Error as IOError, ErrorKind};

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
                                    create_events_from_graph(flow_name_str).await;
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

async fn create_events_from_graph(file_name: &str){

     let toml_document = read_from_documents(file_name).unwrap(); 

      // Convert TOML to serde_json::Value
      let parsed_toml: JsonValue = toml::from_str(&toml_document).expect("Failed to parse TOML");

      println!("{}", parsed_toml); 
      // Convert parsed TOML into JSON Value
      let json_data = serde_json::to_value(parsed_toml).expect("Failed to convert to JSON");

      let work_order = bfs_traversal(&json_data);

      println!("Found {} pieces of work to build out", work_order.len()); 
      for work in &work_order {
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

// Function to determine the next piece of work based on the graph
// fn next_piece_of_work(nodes: Vec<Node>, edges: Vec<Edge>) -> Option<String> {
//     let mut visited = HashSet::new();
//     let mut queue = VecDeque::new();

//     // Find the start node
//     let start_node = nodes.iter().find(|&node| node.data.start.unwrap());
//     if let Some(node) = start_node {
//         visited.insert(&node.id);
//         queue.push_back(&node.id);
//     } else {
//         return None; // No start node
//     }

//     // Convert edges into a adjacency list representation for easier traversal
//     let mut graph: HashMap<&String, Vec<&String>> = HashMap::new();
//     for edge in &edges {
//         graph.entry(&edge.source).or_insert_with(Vec::new).push(&edge.target);
//     }

//     // BFS traversal
//     while !queue.is_empty() {
//         let current = queue.pop_front().unwrap();
        
//         if let Some(neighbors) = graph.get(current) {
//             for neighbor in neighbors {
//                 if !visited.contains(neighbor) {
//                     visited.insert(*neighbor);
//                     queue.push_back(*neighbor);
//                 }
//             }
//         }
//     }

//     // Return the last visited node as the next piece of work
//     // Adjust this logic as per your requirements
//     queue.back().cloned().cloned()
// }


// Thoughts on events based architefture
//https://discord.com/channels/616186924390023171/731495028677148753/1133165388981620837

// Inspiration 
// https://tokio.rs/tokio/tutorial/shared-state