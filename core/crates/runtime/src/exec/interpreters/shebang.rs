use crate::utils::string_utils::strip_leading;

use super::system::SystemShell;

pub trait ShebangInterpreter {
    fn get_interpreter_from_shebang(&self) -> Option<SystemShell>;
}

impl ShebangInterpreter for String {
    fn get_interpreter_from_shebang(&self) -> Option<SystemShell> {
        let s = strip_leading(self);
        let first_line = s.lines().next().unwrap_or("").trim_start();

        if !first_line.starts_with("#!") {
            return None;
        }

        let mut parts = first_line[2..].splitn(2, |c| c == ' ' || c == '\t');
        if let Some(value) = parts.next() {
            let mut args = Vec::new();
            if let Some(v) = parts.next().map(|arg| arg.to_owned()) {
                args.push(v)
            };

            Some(SystemShell {
                interpreter: value.to_string(),
                args,
            })
        } else {
            None
        }
    }
}

impl ShebangInterpreter for &str {
    fn get_interpreter_from_shebang(&self) -> Option<SystemShell> {
        self.to_string().get_interpreter_from_shebang()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_interpreter_can_be_compiled_from_string() {
        let command = r#"#!/usr/bin/env bash

        echo "Hello"
        "#
        .to_string();

        let res = command.get_interpreter_from_shebang().unwrap();
        assert_eq!(res.interpreter, "/usr/bin/env");
        assert_eq!(res.args, vec!["bash".to_string()]);
    }

    #[test]
    fn test_interpreter_can_be_compiled_from_string_with_newline() {
        let command = r#"
        #!/usr/bin/env bash

        echo "Hello"
        "#
        .to_string();

        let res = command.get_interpreter_from_shebang().unwrap();
        assert_eq!(res.interpreter, "/usr/bin/env");
        assert_eq!(res.args, vec!["bash".to_string()]);
    }
}
