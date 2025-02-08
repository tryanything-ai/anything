use std::error::Error;

/// Struct to hold separated imports and body
struct CodeParts {
    imports: Vec<String>,
    body: Vec<String>,
}

/// Function to restructure code by separating imports and wrapping the body in an async IIFE
pub fn restructure_code(user_code: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    // if user_code.contains("import ") || user_code.contains("export ") {
    //     return Err("Import and export statements are not allowed".into());
    // }

    let mut code_parts = CodeParts {
        imports: Vec::new(),
        body: Vec::new(),
    };

    for line in user_code.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("import ") {
            code_parts.imports.push(line.to_string());
        } else if !trimmed.is_empty() {
            code_parts.body.push(line.to_string());
        }
    }

    let mut restructured_code = String::new();

    // Add all imports at the top
    for import in &code_parts.imports {
        restructured_code.push_str(import);
        restructured_code.push('\n');
    }

    // Wrap the body in an async IIFE
    restructured_code.push_str("(async () => {\n");
    for line in &code_parts.body {
        restructured_code.push_str(line);
        restructured_code.push('\n');
    }
    restructured_code.push_str(
        "})().then(result => { globalThis.result = result; }).catch(e => { globalThis.error = e.toString(); });\n",
    );

    Ok(restructured_code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restructure_code_simple() {
        let input_code = r#"
import React from "react";
import { useState } from "react";

const App = () => {
    const [count, setCount] = useState(0);
    return <div>{count}</div>;
};
return App;
"#;

        let expected_output = r#"import React from "react";
import { useState } from "react";
(async () => {
const App = () => {
    const [count, setCount] = useState(0);
    return <div>{count}</div>;
};
return App;
})().then(result => { globalThis.result = result; }).catch(e => { globalThis.error = e.toString(); });
"#;

        let result = restructure_code(input_code).unwrap();
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_restructure_code_simple_with_empty_lines() {
        let input_code = r#"
import fs from "fs";



function readFile() {
    // Read a file
}
return readFile;
"#;

        let expected_output = r#"import fs from "fs";
(async () => {
function readFile() {
    // Read a file
}
return readFile;
})().then(result => { globalThis.result = result; }).catch(e => { globalThis.error = e.toString(); });
"#;

        let result = restructure_code(input_code).unwrap();
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_restructure_code_simple_no_imports() {
        let input_code = r#"
const greet = () => {
    console.log("Hello, World!");
    return "Hello!";
};
return greet();
"#;

        let expected_output = r#"(async () => {
const greet = () => {
    console.log("Hello, World!");
    return "Hello!";
};
return greet();
})().then(result => { globalThis.result = result; }).catch(e => { globalThis.error = e.toString(); });
"#;

        let result = restructure_code(input_code).unwrap();
        assert_eq!(result, expected_output);
    }
}