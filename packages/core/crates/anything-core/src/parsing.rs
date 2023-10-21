use crate::error::{AnythingError, AnythingResult};

pub fn parse_from_value_to_string(
    target: &str,
    object: &serde_json::Value,
) -> AnythingResult<String> {
    match object.get(target) {
        Some(value) => {
            if let Some(value) = value.as_str() {
                Ok(value.to_string())
            } else {
                Err(AnythingError::ParsingError(format!(
                    "could not parse {} as string",
                    target
                )))
            }
        }
        None => Err(AnythingError::ParsingError(format!(
            "could not parse {} as string",
            target
        ))),
    }
}
