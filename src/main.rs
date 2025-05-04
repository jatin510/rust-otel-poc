mod handlers;
mod models;
mod telemetry;
#[cfg(test)]
mod handlers_test;

use actix_web::{web, App, HttpServer, middleware::Logger};
use std::sync::Mutex;
use tracing::info;

use crate::handlers::{AppState, create_user, get_user, get_users, health_check};
use crate::models::load_users_from_file;
use crate::telemetry::init_tracing;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing and OpenTelemetry
    init_tracing();
    
    info!("Starting server...");
    
    // Initialize application state
    let app_state = web::Data::new(AppState {
        users: Mutex::new(load_users_from_file()),
    });
    
    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .route("/health", web::get().to(health_check))
            .route("/users", web::get().to(get_users))
            .route("/users", web::post().to(create_user))
            .route("/users/{id}", web::get().to(get_user))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
