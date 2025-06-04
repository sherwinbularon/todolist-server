use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use validator::Validate;
use crate::models::{CreateTask, Task, UpdateTask};
use sqlx::PgPool;

// ✅ Health check endpoint
pub async fn health_check(db_pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query("SELECT 1")
        .fetch_one(db_pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().body("✅ DB connection OK"),
        Err(err) => {
            eprintln!("❌ Health check failed: {}", err);
            HttpResponse::InternalServerError().body("❌ DB connection failed")
        }
    }
}

pub async fn get_tasks(db: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, Task>("SELECT * FROM tasks")
        .fetch_all(db.get_ref())
        .await;

    match result {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(e) => {
            eprintln!("Database query failed: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to fetch tasks")
        }
    }
}

pub async fn create_task(
    db: web::Data<PgPool>,
    task: web::Json<CreateTask>,
) -> impl Responder {
    if let Err(e) = task.validate() {
        return HttpResponse::BadRequest().json(format!("Validation error: {:?}", e));
    }

    let duplicate = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM tasks WHERE LOWER(title) = LOWER($1)"
    )
    .bind(&task.title)
    .fetch_one(db.get_ref())
    .await;

    match duplicate {
        Ok(count) if count > 0 => return HttpResponse::BadRequest().body("Duplicate title"),
        Err(e) => {
            eprintln!("Error checking duplicates: {:?}", e);
            return HttpResponse::InternalServerError().body("Failed to check for duplicates");
        }
        _ => {}
    }

    let id = Uuid::new_v4();
    let result = sqlx::query_as::<_, Task>(
        "INSERT INTO tasks (id, title, completed) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(id)
    .bind(&task.title)
    .bind(false)
    .fetch_one(db.get_ref())
    .await;

    match result {
        Ok(new_task) => HttpResponse::Created().json(new_task),
        Err(e) => {
            eprintln!("Insert failed: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to create task")
        }
    }
}

pub async fn update_task(
    db: web::Data<PgPool>,
    task_id: web::Path<Uuid>,
    payload: web::Json<UpdateTask>,
) -> impl Responder {
    if let Err(e) = payload.validate() {
        return HttpResponse::BadRequest().json(format!("Validation error: {:?}", e));
    }

    let existing = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = $1")
        .bind(*task_id)
        .fetch_optional(db.get_ref())
        .await;

    if let Ok(Some(task)) = existing {
        let updated_title = payload.title.clone().unwrap_or(task.title.clone());
        let updated_completed = payload.completed.unwrap_or(task.completed);

        let result = sqlx::query_as::<_, Task>(
            "UPDATE tasks SET title = $1, completed = $2 WHERE id = $3 RETURNING *"
        )
        .bind(updated_title)
        .bind(updated_completed)
        .bind(*task_id)
        .fetch_one(db.get_ref())
        .await;

        match result {
            Ok(updated_task) => HttpResponse::Ok().json(updated_task),
            Err(e) => {
                eprintln!("Update failed: {:?}", e);
                HttpResponse::InternalServerError().body("Failed to update task")
            }
        }
    } else {
        HttpResponse::NotFound().body("Task not found")
    }
}

pub async fn delete_task(
    db: web::Data<PgPool>,
    task_id: web::Path<Uuid>,
) -> impl Responder {
    let result = sqlx::query("DELETE FROM tasks WHERE id = $1")
        .bind(*task_id)
        .execute(db.get_ref())
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => HttpResponse::Ok().body("Task deleted"),
        Ok(_) => HttpResponse::NotFound().body("Task not found"),
        Err(e) => {
            eprintln!("Delete failed: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to delete task")
        }
    }
}