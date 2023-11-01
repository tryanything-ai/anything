pub(crate) fn is_newline(c: char) -> bool {
    c == '\r' || c == '\n'
}

pub(crate) fn remove_newlines(s: &str) -> String {
    s.chars().filter(|c| !is_newline(*c)).collect()
}

pub(crate) fn strip_leading(s: &str) -> String {
    let mut trimmed = if s.starts_with("\n") {
        s.trim_start_matches('\n').to_string()
    } else if s.starts_with("\r\n") {
        s.trim_start_matches("\r\n").to_string()
    } else {
        s.to_string()
    };

    if let Some(pos) = trimmed.find('\n') {
        let first_line_trimmed = trimmed[0..pos].trim_start().to_string();
        trimmed.replace_range(0..pos, &first_line_trimmed);
    } else {
        // If there's only one line, trim it
        trimmed = trimmed.trim_start().to_string();
    }

    trimmed
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_remove_newlines() {
        let s = "Hello\nWorld\r\n";
        assert_eq!(remove_newlines(s), "HelloWorld");
    }

    #[test]
    fn test_remove_newlines_from_read_file() {
        let res = std::fs::read_to_string(PathBuf::from("./tests/fixtures/simple.js"));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(
            remove_newlines(&res),
            "export default function () {    return \"Hello World\";}"
        );
    }

    #[test]
    fn test_remove_leading_newlines_from_string() {
        let command = r#"
        #!/usr/bin/env bash

        echo "Hello"
        "#;
        let res = strip_leading(command);
        assert_eq!(
            res,
            "#!/usr/bin/env bash\n\n        echo \"Hello\"\n        "
        );
    }
}
