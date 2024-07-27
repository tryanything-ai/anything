pub enum Stage {
    Production,
    Testing,
}

impl Stage {
    pub fn as_str(&self) -> &str {
        match self {
            Stage::Production => "production",
            Stage::Testing => "testing",
        }
    }
}