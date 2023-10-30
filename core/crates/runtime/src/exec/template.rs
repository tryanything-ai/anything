use tera::Context;

use crate::errors::RuntimeResult;

use super::scope::Scope;

pub fn render_string(name: &str, template: &str, scope: &Scope) -> RuntimeResult<String> {
    let context: Context = scope.try_into()?;

    let _autoescape = true;

    let mut tera = tera::Tera::default();

    tera.add_raw_template(name, template)?;
    let res = tera.render(name, &context);
    tera.templates.remove(name);

    match res {
        Ok(s) => Ok(s),
        Err(e) => Err(e.into()),
    }
}

#[cfg(test)]
mod tests {

    use std::sync::{Arc, Mutex};

    use crate::ExecutionResult;

    use super::*;

    #[test]
    fn test_render_string_without_parent_scope() {
        let mut scope = Scope::default();
        scope.insert_binding("name", "bob", None).unwrap();

        let rendered_template =
            render_string("var_name", "something goes {{ name }}", &scope).unwrap();

        assert_eq!(rendered_template, "something goes bob");
    }

    #[test]
    fn test_render_string_with_parent_scope() {
        let mut parent_scope = Scope::default();
        parent_scope.insert_binding("dad", "fred", None).unwrap();
        let mut scope = Scope::default();
        scope.insert_binding("name", "bob", None).unwrap();
        scope.parent = Some(Arc::new(Mutex::new(parent_scope)));

        let rendered_template =
            render_string("execute_scope", "{{name}} dad is named {{ dad}}", &scope).unwrap();

        assert_eq!(rendered_template, "bob dad is named fred");
    }

    #[test]
    fn test_render_string_with_parent_scope_has_the_child_scope_overriding() {
        let mut parent_scope = Scope::default();
        parent_scope.insert_binding("name", "fred", None).unwrap();
        let mut scope = Scope::default();
        scope.insert_binding("name", "bob", None).unwrap();
        scope.parent = Some(Arc::new(Mutex::new(parent_scope)));

        let rendered_template =
            render_string("execute_scope", "I am named {{ name}}", &scope).unwrap();

        assert_eq!(rendered_template, "I am named bob");
    }

    #[test]
    fn test_render_string_without_parent_scope_includes_environment_variables() {
        std::env::set_var("AWS_ACCESS_KEY", "ABC123");
        let mut scope = Scope::default();
        scope.insert_binding("name", "bob", None).unwrap();
        scope.insert_environment_variable("AWS_ACCESS_KEY".to_string(), None);

        let rendered_template = render_string(
            "var_name",
            "something goes {{ name }} ({{ AWS_ACCESS_KEY }})",
            &scope,
        )
        .unwrap();

        assert_eq!(rendered_template, "something goes bob (ABC123)");
    }

    #[test]
    fn test_render_string_with_parent_scope_includes_environment_variables() {
        std::env::set_var("AWS_SECRET_KEY", "SomethingSecret");
        std::env::set_var("AWS_ACCESS_KEY", "ABC123");
        let mut parent_scope = Scope::default();
        parent_scope.insert_environment_variable("AWS_SECRET_KEY".to_string(), None);
        let mut scope = Scope::default();
        scope.parent = Some(Arc::new(Mutex::new(parent_scope)));

        scope.insert_binding("name", "bob", None).unwrap();
        scope.insert_environment_variable("AWS_ACCESS_KEY".to_string(), None);

        let rendered_template = render_string(
            "var_name",
            "something goes {{ name }} ({{ AWS_ACCESS_KEY }}: {{ AWS_SECRET_KEY }})",
            &scope,
        )
        .unwrap();

        assert_eq!(
            rendered_template,
            "something goes bob (ABC123: SomethingSecret)"
        );
    }

    #[test]
    fn test_render_string_only_includes_set_env_variables() {
        std::env::set_var("AWS_ACCESS_KEY", "ABC123");
        let mut parent_scope = Scope::default();
        parent_scope.insert_environment_variable("AWS_SECRET_KEY".to_string(), None);
        let mut scope = Scope::default();
        scope.parent = Some(Arc::new(Mutex::new(parent_scope)));

        scope.insert_binding("name", "bob", None).unwrap();
        scope.insert_environment_variable("AWS_ACCESS_KEY".to_string(), None);

        let rendered_template = render_string(
            "var_name",
            "something goes {{ name }} ({{ AWS_ACCESS_KEY }})",
            &scope,
        )
        .unwrap();

        assert_eq!(rendered_template, "something goes bob (ABC123)");
    }

    #[test]
    fn test_render_string_includes_previous_results() {
        let mut scope = Scope::default();
        scope.insert_binding("name", "bob", None).unwrap();

        let exec_result = ExecutionResult {
            stdout: "Hello world".to_string(),
            stderr: "".to_string(),
            status: 0,
        };

        let _ = scope.insert_result("test_stage".to_string(), exec_result);

        let rendered_template =
            render_string("var_name", "from previous {{ test_stage.stdout }}", &scope).unwrap();

        assert_eq!(rendered_template, "from previous Hello world");
    }

    #[test]
    fn test_render_string_includes_previous_scope_results() {
        let mut parent_scope = Scope::default();

        let exec_result = ExecutionResult {
            stdout: "Hello world".to_string(),
            stderr: "".to_string(),
            status: 0,
        };

        let _ = parent_scope.insert_result("test_stage".to_string(), exec_result);
        let mut scope = Scope::default();
        scope.parent = Some(Arc::new(Mutex::new(parent_scope)));

        scope.insert_binding("name", "bob", None).unwrap();

        let rendered_template = render_string(
            "var_name",
            "{{ name }} from previous {{ test_stage.stdout }}",
            &scope,
        )
        .unwrap();

        assert_eq!(rendered_template, "bob from previous Hello world");
    }
}
