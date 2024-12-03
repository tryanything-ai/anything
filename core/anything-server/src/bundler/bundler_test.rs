// use super::*;  // Import everything from the parent module
// use serde_json::json;

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_http_action_bundling() {
//         // Create test variables context
//         let variables = json!({
//             "api_url": "https://api.example.com",
//             "auth_token": "secret123"
//         });

//         // Create test input definition matching HTTP action template
//         let input_definition = json!({
//             "method": "POST",
//             "url": "{{variables.api_url}}/endpoint",
//             "headers": {
//                 "Authorization": "Bearer {{variables.auth_token}}",
//                 "Content-Type": "application/json"
//             },
//             "body": {
//                 "foo": "bar"
//             }
//         });

//         // Bundle the inputs
//         let result = bundle_inputs(variables, Some(&input_definition)).unwrap();

//         // Verify rendered output
//         assert_eq!(result, json!({
//             "method": "POST",
//             "url": "https://api.example.com/endpoint",
//             "headers": {
//                 "Authorization": "Bearer secret123",
//                 "Content-Type": "application/json"
//             },
//             "body": {
//                 "foo": "bar"
//             }
//         }));
//     }

//     #[test]
//     fn test_empty_inputs() {
//         let variables = json!({});
//         let result = bundle_inputs(variables, None).unwrap();
//         assert_eq!(result, json!({}));
//     }

//     #[test]
//     fn test_numeric_output() {
//         let variables = json!({
//             "count": 42
//         });

//         let input_definition = json!({
//             "total_items": "{{variables.count}}"
//         });

//         let result = bundle_inputs(variables, Some(&input_definition)).unwrap();

//         // Verify the output is a number, not a string
//         assert_eq!(result, json!({
//             "total_items": 42
//         }));
//     }
// }
