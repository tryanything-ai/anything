use anything_persistence::FlowVersion;
use serde_json::Value as JsonValue;
use std::collections::VecDeque;

pub fn create_execution_plan(_flow_version: FlowVersion) {
    // Your code here
    // todo!("Implement create_execution_plan")
    println!("create_execution_plan");

    let flow_definition = _flow_version.flow_definition.clone();
    let json_data = serde_json::from_value(flow_definition).unwrap();

    // println!("json_data: {:?}", json_data);

    let result = bfs_traversal(&json_data);
    println!("result from bfs: {:?}", result);
    // return result;

    //traverse graph
    //create events for each node
    //write events to sqlite
    //Hint to executer that we have new stuff to do
    //  ( executer decides on its one what to do, scheduleing priority etc)
    //return true
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

    // Print the graph
    println!("Graph: {:?}", graph);

    // Use a BFS queue
    let mut queue = VecDeque::new();

    // Find and enqueue the node with "data.worker_type" = "start"
    // if let Some(nodes) = json_data.get("nodes") {
    //     for node in nodes.as_array().unwrap() {
    //         if let Some(data) = node.get("data") {
    //             if data
    //                 .get("worker_type")
    //                 .map_or(false, |w| w.as_str().unwrap_or("") == "start")
    //             {
    //                 queue.push_back(node.clone());
    //                 break; // Since there should be only one start node based on the context
    //             }
    //         }
    //     }
    // }

    //put the trigger as the first node in the queue
    let trigger = if let Some(trigger) = json_data.get("trigger") {
        trigger.clone()
    } else {
        panic!("Trigger not found in json_data");
    };

    // let trigger = json_data.get("trigger").unwrap();
    queue.push_back(trigger.clone());

    // BFS traversal
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        println!("current: {:?}", current);

        // Add current node to the work list
        work_list.push(current.clone());
        println!("work_list: {:?}", work_list);

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
