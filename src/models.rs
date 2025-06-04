use serde::{Deserialize, Serialize, Deserializer};
use uuid::Uuid;
use validator::Validate;
use lazy_static::lazy_static;
use regex::Regex;
use sqlx::FromRow;

lazy_static! {
    pub static ref ALPHANUMERIC_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9 ]+$").unwrap();
}

#[derive(Debug, Serialize, FromRow)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
}
#[derive(Debug, Deserialize, Validate)]
pub struct CreateTask {
    #[serde(deserialize_with = "deserialize_any_string")]
    #[validate(length(min = 1, max = 255))]
    #[validate(custom = "validate_alphanumeric")]
    pub title: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTask {
    #[serde(default, deserialize_with = "deserialize_any_option_string")]
    #[validate(length(min = 1, max = 255))]
    #[validate(custom = "validate_optional_alphanumeric")]
    pub title: Option<String>,

    pub completed: Option<bool>,
}

// ─────────────────────────────────────
// Validators
// ─────────────────────────────────────

fn validate_alphanumeric(title: &str) -> Result<(), validator::ValidationError> {
    if ALPHANUMERIC_REGEX.is_match(title) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("invalid_characters"))
    }
}
fn validate_optional_alphanumeric(title: &str) -> Result<(), validator::ValidationError> {
    if !title.chars().all(|c| c.is_alphanumeric() || c.is_whitespace()) {
        return Err(validator::ValidationError::new("invalid_characters"));
    }
    Ok(())
}

// ─────────────────────────────────────
// Deserializers
// ─────────────────────────────────────

fn deserialize_any_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringVisitor;

    impl<'de> serde::de::Visitor<'de> for StringVisitor {
        type Value = String;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("a string or number")
        }

        fn visit_str<E>(self, v: &str) -> Result<String, E>
        where
            E: serde::de::Error,
        {
            Ok(v.to_string())
        }

        fn visit_i64<E>(self, v: i64) -> Result<String, E>
        where
            E: serde::de::Error,
        {
            Ok(v.to_string())
        }

        fn visit_u64<E>(self, v: u64) -> Result<String, E>
        where
            E: serde::de::Error,
        {
            Ok(v.to_string())
        }

        fn visit_f64<E>(self, v: f64) -> Result<String, E>
        where
            E: serde::de::Error,
        {
            Ok(v.to_string())
        }
    }

    deserializer.deserialize_any(StringVisitor)
}

fn deserialize_any_option_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct OptionStringVisitor;

    impl<'de> serde::de::Visitor<'de> for OptionStringVisitor {
        type Value = Option<String>;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("a string, number, or null")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserialize_any_string(deserializer).map(Some)
        }
    }

    deserializer.deserialize_option(OptionStringVisitor)
}