mod handlers;
mod models;
mod state;

use actix_web::{web, App, HttpServer};
use state::TaskList;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let task_list = web::Data::new(TaskList::new(Vec::new()));

    println!("🚀 Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(task_list.clone())
            .route("/tasks", web::get().to(handlers::get_tasks))
            .route("/tasks", web::post().to(handlers::create_task))
            .route("/tasks/{id}", web::put().to(handlers::update_task))
            .route("/tasks/{id}", web::delete().to(handlers::delete_task))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}