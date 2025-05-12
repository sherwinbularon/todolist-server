use serde::{Deserialize, Serialize, Deserializer};
use validator::Validate;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref ALPHANUMERIC_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9 ]+$").unwrap();
}

#[derive(Clone, Serialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTask {
    #[serde(deserialize_with = "deserialize_any_string")]
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    #[validate(length(min = 1, max = 255))]
    pub title: String,
}

fn deserialize_any_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringVisitor;

    impl<'de> serde::de::Visitor<'de> for StringVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string or number")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v.to_string())
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v.to_string())
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v.to_string())
        }

        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v.to_string())
        }
    }
    #[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub completed: bool,
}

    deserializer.deserialize_any(StringVisitor)
}
