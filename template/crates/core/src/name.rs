use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// The identity a config with no `name:` runs under.
const DEFAULT_NAME: &str = "{{project-name}}";

/// Why a string could not become a [`ServiceName`].
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ServiceNameError {
    /// The string was empty.
    #[error("a service name cannot be empty")]
    Empty,

    /// The string held a character outside the allowed set.
    #[error("`{0}` is not a service name: only `a-z`, `0-9` and `-` are allowed")]
    Charset(String),
}

/// A service identity, validated once at construction.
///
/// It reaches log fields, metric labels and URLs alike, so the charset is
/// narrow enough to be safe in all three — parse it at the edge and the rest of
/// the program never re-checks it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ServiceName(String);

impl ServiceName {
    /// Borrows the name as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for ServiceName {
    fn default() -> Self {
        Self(DEFAULT_NAME.to_owned())
    }
}

impl TryFrom<String> for ServiceName {
    type Error = ServiceNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ServiceNameError::Empty);
        }

        if !value
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        {
            return Err(ServiceNameError::Charset(value));
        }

        Ok(Self(value))
    }
}

impl FromStr for ServiceName {
    type Err = ServiceNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::try_from(value.to_owned())
    }
}

impl From<ServiceName> for String {
    fn from(name: ServiceName) -> Self {
        name.0
    }
}

impl AsRef<str> for ServiceName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for ServiceName {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::{DEFAULT_NAME, ServiceName, ServiceNameError};

    #[test]
    fn a_plain_name_parses() {
        assert_eq!(
            "api-gateway".parse::<ServiceName>().unwrap().as_str(),
            "api-gateway"
        );
    }

    #[test]
    fn the_default_is_a_name_that_would_parse() {
        assert_eq!(ServiceName::default(), DEFAULT_NAME.parse().unwrap());
    }

    #[test]
    fn an_empty_name_is_refused() {
        assert_eq!("".parse::<ServiceName>(), Err(ServiceNameError::Empty));
    }

    #[test]
    fn an_uppercase_name_is_refused() {
        // Metric labels and URLs treat case differently; one spelling only.
        assert!(matches!(
            "API".parse::<ServiceName>(),
            Err(ServiceNameError::Charset(_))
        ));
    }

    #[test]
    fn the_offender_is_named_in_the_error() {
        let error = "api gateway"
            .parse::<ServiceName>()
            .unwrap_err()
            .to_string();

        assert!(error.contains("api gateway"), "got {error}");
    }
}
