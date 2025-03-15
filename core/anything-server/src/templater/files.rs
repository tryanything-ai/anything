use super::TemplateError;
use super::Templater;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct FileRequirement {
    pub file_name: String, // The full variable path (e.g. "inputs.file_name")
    pub file_extension: String, // The extension part if specified
    pub format: String,        // Expected format (base64 or url)
}

pub fn get_template_file_requirements(
    template: &Value,
) -> Result<Vec<FileRequirement>, TemplateError> {
    let mut templater = Templater::new();
    templater.add_template("_analysis_template", template.clone());

    let variables = templater.get_template_variables("_analysis_template")?;
    let mut file_requirements = Vec::new();

    // Analyze each variable for file patterns
    for var in variables {
        // Look for variables with the pattern: inputs.*.file_extension.format
        let parts: Vec<&str> = var.split('.').collect();

        if parts.len() >= 2 && parts[0] == "inputs" {
            // Check if this looks like a file pattern
            if let Some((file_info, format)) = analyze_file_pattern(&parts) {
                // Skip "inputs." by starting from index 1
                file_requirements.push(FileRequirement {
                    file_name: parts[1..parts.len() - 1].join("."), // Everything except "inputs." and format
                    file_extension: file_info.to_string(),
                    format: format.to_string(),
                });
            }
        }
    }

    Ok(file_requirements)
}

fn analyze_file_pattern<'a>(parts: &[&'a str]) -> Option<(&'a str, &'a str)> {
    if parts.len() < 3 {
        return None;
    }

    // Check the last part for format specification
    match parts.last() {
        Some(&"file_base64") | Some(&"file_url") => {
            // The part before the format might contain file extension info
            let file_info = parts[parts.len() - 2];
            // Convert file_base64 to base64 and file_url to url for consistency
            let format = match *parts.last().unwrap() {
                "file_base64" => "base64",
                "file_url" => "url",
                _ => unreachable!(),
            };
            Some((file_info, format))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_file_requirements() {
        let template = json!({
            "document": {
                "image": "{{inputs.profile_pic.png.file_base64}}",
                "attachment": "{{inputs.document.pdf.file_url}}",
                "regular_field": "{{inputs.name}}",
                "nested": {
                    "file": "{{inputs.config.yaml.file_base64}}"
                }
            }
        });

        let requirements = get_template_file_requirements(&template).unwrap();

        // Sort requirements by path_variable for consistent testing
        let mut requirements = requirements;
        requirements.sort_by(|a, b| a.file_name.cmp(&b.file_name));

        assert_eq!(requirements.len(), 3);

        // Check profile pic requirements
        assert_eq!(requirements[0].file_name, "config.yaml");
        assert_eq!(requirements[0].file_extension, "yaml");
        assert_eq!(requirements[0].format, "base64");

        // Check document requirements
        assert_eq!(requirements[1].file_name, "document.pdf");
        assert_eq!(requirements[1].file_extension, "pdf");
        assert_eq!(requirements[1].format, "url");

        // Check config requirements
        assert_eq!(requirements[2].file_name, "profile_pic.png");
        assert_eq!(requirements[2].file_extension, "png");
        assert_eq!(requirements[2].format, "base64");
    }

    #[test]
    fn test_file_requirements_with_complex_template() {
        let template = json!({
            "files": {
                "images": [
                    {"content": "{{inputs.header_image.jpg.file_base64}}"},
                    {"content": "{{inputs.logo.png.file_url}}"}
                ],
                "documents": {
                    "main": "{{inputs.main_doc.docx.file_url}}",
                    "appendix": "{{inputs.appendix.pdf.file_base64}}"
                }
            },
            "metadata": {
                "timestamp": "{{inputs.timestamp}}",
                "user": "{{inputs.user_id}}"
            }
        });

        let requirements = get_template_file_requirements(&template).unwrap();

        // Sort requirements by path_variable for consistent testing
        let mut requirements = requirements;
        requirements.sort_by(|a, b| a.file_name.cmp(&b.file_name));

        assert_eq!(requirements.len(), 4);

        // Check appendix requirements
        assert_eq!(requirements[0].file_name, "appendix.pdf");
        assert_eq!(requirements[0].file_extension, "pdf");
        assert_eq!(requirements[0].format, "base64");

        // Check header image requirements
        assert_eq!(requirements[1].file_name, "header_image.jpg");
        assert_eq!(requirements[1].file_extension, "jpg");
        assert_eq!(requirements[1].format, "base64");

        // Check logo requirements
        assert_eq!(requirements[2].file_name, "logo.png");
        assert_eq!(requirements[2].file_extension, "png");
        assert_eq!(requirements[2].format, "url");

        // Check main doc requirements
        assert_eq!(requirements[3].file_name, "main_doc.docx");
        assert_eq!(requirements[3].file_extension, "docx");
        assert_eq!(requirements[3].format, "url");
    }
}
