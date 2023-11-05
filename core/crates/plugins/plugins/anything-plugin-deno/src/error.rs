// pub type PluginResult<T> = Result<T, PluginError>;

// pub enum PluginError {
//     NotFoundInConfig,
//     RuntimeError,
//     StdError(std::io::Error),
// }

// impl Into<String> for PluginError {
//     fn into(self) -> String {
//         match self {
//             Self::NotFoundInConfig => "Not found in config".to_string(),
//             Self::RuntimeError => "Runtime Error".to_string(),
//             Self::StdError(e) => e.to_string(),
//         }
//     }
// }
