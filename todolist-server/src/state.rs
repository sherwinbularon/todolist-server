use crate::models::Task;
use std::sync::Mutex;

pub type TaskList = Mutex<Vec<Task>>;