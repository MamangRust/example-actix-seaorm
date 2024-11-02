use crate::{
    domain::{CreateCategoryRequest, UpdateCategoryRequest},
    state::AppState,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;

#[get("/categories")]
async fn get_categories(data: web::Data<AppState>) -> impl Responder {
    match data.di_container.category_service.get_categories().await {
        Ok(categories) => HttpResponse::Ok().json(categories),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to fetch categories",
        })),
    }
}

#[get("/categories/{id}")]
async fn get_category(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    match data
        .di_container
        .category_service
        .get_category(id.into_inner())
        .await
    {
        Ok(Some(category)) => HttpResponse::Ok().json(category),
        Ok(None) => HttpResponse::NotFound().json(json!({
            "status": "fail",
            "message": "Category not found",
        })),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to fetch category",
        })),
    }
}

#[post("/categories")]
async fn create_category(
    data: web::Data<AppState>,
    body: web::Json<CreateCategoryRequest>,
) -> impl Responder {
    match data
        .di_container
        .category_service
        .create_category(&body)
        .await
    {
        Ok(category) => HttpResponse::Created().json(category),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to create category",
        })),
    }
}

#[put("/categories/{id}")]
async fn update_category(
    data: web::Data<AppState>,
    id: web::Path<i32>,
    body: web::Json<UpdateCategoryRequest>,
) -> impl Responder {
    let mut update_request = body.into_inner();

    update_request.id = id.into_inner();

    match data
        .di_container
        .category_service
        .update_category(&update_request)
        .await
    {
        Ok(Some(category)) => HttpResponse::Ok().json(category),
        Ok(None) => HttpResponse::NotFound().json(json!({
            "status": "fail",
            "message": "Category not found",
        })),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to update category",
        })),
    }
}

#[delete("/categories/{id}")]
async fn delete_category(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    match data
        .di_container
        .category_service
        .delete_category(id.into_inner())
        .await
    {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Category deleted successfully",
        })),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to delete category",
        })),
    }
}
