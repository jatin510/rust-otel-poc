use actix_web::{web, HttpResponse, Responder};
use std::sync::Mutex;
use tracing::{debug, error, info, instrument};

use crate::models::{save_users_to_file, User};

// AppState for sharing data between handlers
pub struct AppState {
    pub users: Mutex<Vec<User>>,
}


#[instrument]
fn hello() {
    let mut x = 0;
    for i in 0..1000000 {
        x += 1;
    }
    info!(x);
}

// Health check endpoint
#[instrument]
pub async fn health_check() -> impl Responder {
    println!("health check called");
    info!("Health check endpoint called");
    debug!("Returning healthy status");
    hello();
    HttpResponse::Ok().json(serde_json::json!({ "status": "healthy" }))
}

// GET endpoint for user information
#[instrument(skip(app_state))]
pub async fn get_users(app_state: web::Data<AppState>) -> impl Responder {
    info!("Get users endpoint called");
    let users = app_state.users.lock().unwrap();
    let user_count = users.len();
    info!("Returning {} users", user_count);
    HttpResponse::Ok().json(&*users)
}

// GET endpoint for a specific user
#[instrument(skip(app_state))]
pub async fn get_user(app_state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let user_id = path.into_inner();
    info!(user_id = %user_id, "Get user endpoint called");

    let users = app_state.users.lock().unwrap();
    match users.iter().find(|u| u.id == user_id) {
        Some(user) => {
            info!(user_id = %user_id, "User found");
            HttpResponse::Ok().json(user)
        }
        None => {
            info!(user_id = %user_id, "User not found");
            HttpResponse::NotFound().json(serde_json::json!({ "error": "User not found" }))
        }
    }
}

// POST endpoint for creating users
#[instrument(skip(app_state, user))]
pub async fn create_user(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    info!(user_id = %user.id, user_name = %user.name, "Create user endpoint called");

    let mut users = app_state.users.lock().unwrap();

    // Check if user already exists
    if users.iter().any(|u| u.id == user.id) {
        info!(user_id = %user.id, "User already exists");
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "User with this ID already exists"
        }));
    }

    // Add the new user
    let new_user = user.into_inner();
    users.push(new_user.clone());

    // Persist users to disk
    match save_users_to_file(&users) {
        Ok(_) => {
            info!(user_id = %new_user.id, "User created successfully");
            HttpResponse::Created().json(new_user)
        }
        Err(e) => {
            error!(user_id = %new_user.id, error = %e, "Failed to save user");
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to save user"
            }))
        }
    }
}
