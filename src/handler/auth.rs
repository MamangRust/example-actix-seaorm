use actix_web::{
    
    get, post, web,  HttpResponse, Responder,
};
use serde_json::json;

use crate::{domain::{LoginRequest, RegisterRequest}, middleware::JwtMiddleware, state::AppState};

#[post("/auth/register")]
async fn register_user_handler(
    body: web::Json<RegisterRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    match data.di_container.auth_service.register_user(&body).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}

#[post("/auth/login")]
async fn login_user_handler(
    body: web::Json<LoginRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    match data.di_container.auth_service.login_user(&body).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::Unauthorized().json(e),
    }
}

#[get("/auth/user")]
async fn get_user(data: web::Data<AppState>, jwt_guard: JwtMiddleware) -> impl Responder {
    let user = match data.di_container.user_service.find_by_id(jwt_guard.user_id).await {
        Ok(Some(user)) => user,  
        Ok(None) => {
            return HttpResponse::NotFound().json(json!({
                "status": "fail",
                "message": "User not found",
            }));
        },
        Err(_) => {
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Could not fetch user",
            }));
        },
    };

   
    let response = json!({
        "status": "success",
        "message": "User fetched successfully",
        "data": {
            "user": user
        }
    });

    HttpResponse::Ok().json(response)
}
