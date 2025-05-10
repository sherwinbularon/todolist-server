use serde::{Deserialize, Serialize};
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
    #[validate(length(max = 255))]
    #[validate(regex(path = "crate::models::ALPHANUMERIC_REGEX"))]
    pub title: String,
}