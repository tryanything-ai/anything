use anything_persistence::{CreateEvent, FlowVersion};
use serde_json::Value as JsonValue;
use std::collections::VecDeque;
use uuid::Uuid;

pub fn create_execution_plan(flow_version: FlowVersion) -> Vec<CreateEvent> {
    // Your code here
    println!("create_execution_plan");

    let flow_definition = flow_version.flow_definition.clone();
    let json_data = serde_json::from_value(flow_definition).unwrap();

    // println!("json_data: {:?}", json_data);
    //traverse graph
    let result = bfs_traversal(&json_data);
    // println!("work list from bfs: {:?}", result);

    let trigger_session_id = Uuid::new_v4().to_string();
    let flow_session_id = Uuid::new_v4().to_string();
    let mut events = Vec::new();

    //grab trigger
    let trigger = if let Some(trigger) = json_data.get("trigger") {
        trigger.clone()
    } else {
        panic!("Trigger not found in json_data");
    };

    for result in &result {
        let is_trigger = events.len() == 0;
        //TODO: maybe make pass through ID from client to make pinging easier in debug
        let event = CreateEvent {
            event_id: Uuid::new_v4().to_string(),
            event_status: "WAITING".to_string(),
            trigger_id: Some(
                trigger
                    .get("node_name")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string(),
            ),
            is_trigger,
            engine_id: if is_trigger {
                "trigger".to_string()
            } else {
                result.get("engine").unwrap().as_str().unwrap().to_string()
            },
            stage: "DEV".to_string(),
            node_id: result
                .get("node_name")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            context: None,
            started_at: None,
            ended_at: None,
            flow_id: Some(flow_version.flow_id.clone()),
            flow_version_id: Some(flow_version.flow_version_id.clone()),
            flow_version_name: Some(flow_version.flow_version.clone()),
            flow_session_status: "WAITING".to_string(),
            trigger_session_id: Some(trigger_session_id.clone()),
            trigger_session_status: "WAITING".to_string(),
            flow_session_id: Some(flow_session_id.clone()),
            created_at: Some(chrono::Utc::now()),
            debug_result: None,
            result: None,
        };
        events.push(event);
    }
    // println!("events going to manager: {:?}", events);
    return events;
}

fn bfs_traversal(json_data: &JsonValue) -> Vec<JsonValue> {
    println!("bfs_traversal");
    // Resultant list of work
    let mut work_list = Vec::new();

    // Create a map of node ids to their outgoing edges
    let mut graph = std::collections::HashMap::new();
    if let Some(edges) = json_data.get("edges") {
        for edge in edges.as_array().unwrap() {
            let source = edge.get("source").unwrap().as_str().unwrap();
            let target = edge.get("target").unwrap().as_str().unwrap();

            graph
                .entry(source.to_string())
                .or_insert_with(Vec::new)
                .push(target.to_string());
        }
    }

    // Use a BFS queue
    let mut queue = VecDeque::new();

    //put the trigger as the first node in the queue
    let trigger = if let Some(trigger) = json_data.get("trigger") {
        trigger.clone()
    } else {
        panic!("Trigger not found in json_data");
    };

    queue.push_back(trigger.clone());

    // BFS traversal
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        // println!("current: {:?}", current);

        // Add current node to the work list
        work_list.push(current.clone());
        // println!("work_list: {:?}", work_list);

        // Enqueue neighbors
        if let Some(neighbors) = graph.get(current.get("node_name").unwrap().as_str().unwrap()) {
            if let Some(actions) = json_data.get("actions") {
                for neighbor_id in neighbors {
                    let neighbor = actions
                        .as_array()
                        .unwrap()
                        .iter()
                        .find(|&action| {
                            action.get("node_name").unwrap().as_str().unwrap() == neighbor_id
                        })
                        .unwrap()
                        .clone();
                    queue.push_back(neighbor);
                }
            }
        }
    }

    work_list
}
