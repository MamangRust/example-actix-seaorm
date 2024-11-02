use crate::{
    domain::{CreateCommentRequest, UpdateCommentRequest},
    state::AppState,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;

#[get("/comments")]
async fn get_comments(data: web::Data<AppState>) -> impl Responder {
    match data.di_container.comment_service.get_comments().await {
        Ok(comments) => HttpResponse::Ok().json(comments),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to fetch comments",
        })),
    }
}

#[get("/comments/{id}")]
async fn get_comment(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    match data
        .di_container
        .comment_service
        .get_comment(id.into_inner())
        .await
    {
        Ok(Some(comment)) => HttpResponse::Ok().json(comment),
        Ok(None) => HttpResponse::NotFound().json(json!({
            "status": "fail",
            "message": "Comment not found",
        })),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to fetch comment",
        })),
    }
}

#[post("/comments")]
async fn create_comment(
    data: web::Data<AppState>,
    body: web::Json<CreateCommentRequest>,
) -> impl Responder {
    match data
        .di_container
        .comment_service
        .create_comment(&body)
        .await
    {
        Ok(comment) => HttpResponse::Created().json(comment),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to create comment",
        })),
    }
}

#[put("/comments/{id}")]
async fn update_comment(
    data: web::Data<AppState>,
    body: web::Json<UpdateCommentRequest>,
) -> impl Responder {
   
    

    match data
        .di_container
        .comment_service
        .update_comment(&body)
        .await
    {
        Ok(Some(comment)) => HttpResponse::Ok().json(comment),
        Ok(None) => HttpResponse::NotFound().json(json!({
            "status": "fail",
            "message": "Comment not found",
        })),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to update comment",
        })),
    }
}

#[delete("/comments/{id}")]
async fn delete_comment(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    match data
        .di_container
        .comment_service
        .delete_comment(id.into_inner())
        .await
    {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Comment deleted successfully",
        })),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to delete comment",
        })),
    }
}
