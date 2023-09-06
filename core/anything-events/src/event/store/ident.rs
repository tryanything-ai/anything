use std::{fmt::Display, ops::Deref};

use crate::{error::AnythingError, types::AnythingResult};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize, PartialOrd, Ord)]
pub struct Identifier(String);

impl Identifier {
    pub fn new<S: Into<String>>(s: S) -> AnythingResult<Self> {
        let s = s.into();
        if !Self::is_valid_identifier(&s) {
            return Err(AnythingError::InvalidIdentifier(s));
        }
        Ok(Self(s))
    }

    pub fn is_valid_identifier(s: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new("^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
        }
        RE.is_match(s)
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Identifier {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_identifiers() {
        let id = Identifier::new("bob_and_jaime").unwrap();
        assert_eq!(*id, "bob_and_jaime".to_string())
    }

    #[test]
    fn it_disallows_empty_identifiers() {
        let err = Identifier::new("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "invalid identifier: ")
    }

    #[test]
    fn it_disallows_spaces() {
        let err = Identifier::new("bob and harold".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "invalid identifier: bob and harold")
    }
}
