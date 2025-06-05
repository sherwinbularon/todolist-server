use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::TcpListener;

mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("❌ Failed to connect to PostgreSQL");

    // Get the PORT from env or default to 8000
    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let address = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&address)?;
    println!("🚀 Server running at http://{}", address);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .route("/tasks", web::get().to(handlers::get_tasks))
            .route("/tasks", web::post().to(handlers::create_task))
            .route("/tasks/{id}", web::put().to(handlers::update_task))
            .route("/tasks/{id}", web::delete().to(handlers::delete_task))
            .route("/health", web::get().to(handlers::health_check))
    })
    .listen(listener)?
    .run()
    .await
}