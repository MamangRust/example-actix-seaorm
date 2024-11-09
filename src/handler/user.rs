use actix_web::{post, put, delete, get, web, HttpResponse, Responder};
use serde_json::json;
use crate::domain::{CreateUserRequest, UpdateUserRequest};
use crate::state::AppState;
use crate::middleware::JwtMiddleware;

#[post("/user")]
async fn create_user(data: web::Data<AppState>, _jwt_guard: JwtMiddleware, body: web::Json<CreateUserRequest>) -> impl Responder {
    match data.di_container.user_service.create_user(&body).await {
        Ok(response) => HttpResponse::Created().json(response),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to create user",
        })),
    }
}

#[get("/user/email")]
async fn find_user_by_email(data: web::Data<AppState>, _jwt_guard: JwtMiddleware, email: web::Path<String>) -> impl Responder {
    match data.di_container.user_service.find_user_by_email(&email).await {
        Ok(Some(response)) => HttpResponse::Ok().json(response),
        Ok(None) => HttpResponse::NotFound().json(json!({
            "status": "fail",
            "message": "User not found",
        })),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to fetch user",
        })),
    }
}

#[put("/user/{id}")]
async fn update_user(data: web::Data<AppState>, _jwt_guard: JwtMiddleware, id: web::Path<i32>,body: web::Json<UpdateUserRequest>) -> impl Responder {
    let mut update_request = body.into_inner();

    

    update_request.id = Some(id.into_inner());

    match data.di_container.user_service.update_user(&update_request).await {
        Ok(Some(response)) => HttpResponse::Ok().json(response),
        Ok(None) => HttpResponse::NotFound().json(json!({
            "status": "fail",
            "message": "User not found",
        })),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to update user",
        })),
    }
}

#[delete("/user")]
async fn delete_user(data: web::Data<AppState>,_jwt_guard: JwtMiddleware, email: web::Path<String>) -> impl Responder {
    match data.di_container.user_service.delete_user(&email).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "User deleted successfully",
        })),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to delete user",
        })),
    }
}
