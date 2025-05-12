use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use validator::Validate;
use crate::models::{CreateTask, Task};
use crate::state::TaskList;
use serde::Deserialize;

pub async fn get_tasks(data: web::Data<TaskList>) -> impl Responder {
    let tasks = data.lock().unwrap();
    HttpResponse::Ok().json(tasks.clone())
}

pub async fn create_task(
    task: web::Json<CreateTask>,
    data: web::Data<TaskList>,
) -> impl Responder {
    // Validate the input
    if let Err(e) = task.validate() {
        return HttpResponse::BadRequest().json(format!("Validation error: {:?}", e));
    }

    let mut tasks = data.lock().unwrap();

    // Ensure unique title
    if tasks.iter().any(|t| t.title.eq_ignore_ascii_case(&task.title)) {
        return HttpResponse::BadRequest().body("duplicate");
    }

    let new_task = Task {
        id: Uuid::new_v4().to_string(),
        title: task.title.clone(),
        completed: false,
    };

    tasks.push(new_task.clone());
    HttpResponse::Created().json(new_task)
}

#[derive(Deserialize)]
pub struct UpdateTaskPayload {
    pub title: String,
    pub completed: bool,
}

pub async fn update_task(
    task_id: web::Path<String>,
    data: web::Data<TaskList>,
    payload: web::Json<UpdateTaskPayload>,
) -> impl Responder {
    let mut tasks = data.lock().unwrap(); 

    if let Some(task) = tasks.iter_mut().find(|t| t.id == *task_id) {
        task.title = payload.title.clone();
        task.completed = payload.completed;
        HttpResponse::Ok().json(task.clone())
    } else {
        HttpResponse::NotFound().body("Task not found")
    }
}

pub async fn delete_task(
    task_id: web::Path<String>,
    data: web::Data<TaskList>,
) -> impl Responder {
    let mut tasks = data.lock().unwrap();
    let len_before = tasks.len();
    tasks.retain(|t| t.id != *task_id);
    if tasks.len() < len_before {
        HttpResponse::Ok().body("Task deleted")
    } else {
        HttpResponse::NotFound().body("Task not found")
    }
}