use serde::Deserialize;

use crate::EngineOption;

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub enum PluginOption {
    #[default]
    None,
    String(String),
    List(Vec<PluginOption>),
    Map(indexmap::IndexMap<String, PluginOption>),
}

impl Into<EngineOption> for PluginOption {
    fn into(self) -> EngineOption {
        match self {
            Self::None => EngineOption::None,
            Self::String(s) => EngineOption::String(s),
            Self::List(v) => EngineOption::List(
                v.into_iter()
                    .map(|o| o.into())
                    .collect::<Vec<EngineOption>>(),
            ),
            Self::Map(m) => EngineOption::Map(m.into_iter().fold(
                indexmap::indexmap! {},
                |mut acc, (k, v)| {
                    acc.insert(k, v.into());
                    acc
                },
            )),
        }
    }
}
