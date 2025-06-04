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

    // Automatically choose a free port by binding to port 0
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let address = listener.local_addr()?;

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
    })
    .listen(listener)? // use the dynamically selected port
    .run()
    .await
}