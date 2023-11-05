use lazy_static::lazy_static;

lazy_static! {
    pub static ref POSSIBLE_SHELL_NAMES: Vec<&'static str> = vec!["bash", "sh", "zsh", "fish"];
}
