use std::fs;
use std::path::Path;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::env;

pub fn load_schema_templates() -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let mut templates = Vec::new();
    
    // Get current directory
    let current_dir = env::current_dir()?;
    let schemas_dir = current_dir.join("src/system_plugins/registry/schemas");

    // Check if schemas directory exists
    if !schemas_dir.exists() {
        return Err("Schemas directory not found".into());
    }

    // Iterate through files in schemas directory
    for entry in fs::read_dir(schemas_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Only process .json files
        if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
            let file = File::open(&path)?;
            let reader = BufReader::new(file);

            // Parse JSON file
            let json: Value = serde_json::from_reader(reader)?;
            
            // Add the JSON object to our templates vector
            templates.push(json);
        }
    }

    Ok(templates)
}
