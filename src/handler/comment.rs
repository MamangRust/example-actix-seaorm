use crate::{
    domain::{CreateCommentRequest, UpdateCommentRequest}, middleware::JwtMiddleware, state::AppState
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;
use tracing::{info, error};

#[get("/comments")]
async fn get_comments(data: web::Data<AppState>, _jwt_guard: JwtMiddleware) -> impl Responder {
    info!("Fetching all comments");
    match data.di_container.comment_service.get_comments().await {
        Ok(comments) => {
            info!("Successfully fetched comments");
            HttpResponse::Ok().json(comments)
        },
        Err(e) => {
            error!("Failed to fetch comments: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to fetch comments",
            }))
        }
    }
}

#[get("/comments/{id}")]
async fn get_comment(data: web::Data<AppState>, id: web::Path<i32>, _jwt_guard: JwtMiddleware) -> impl Responder {
    let comment_id = id.into_inner();
    
    info!("Fetching comment with ID: {}", comment_id);

    match data
        .di_container
        .comment_service
        .get_comment(comment_id)
        .await
    {
        Ok(Some(comment)) => {
            info!("Comment found: {:?}", comment);
            HttpResponse::Ok().json(comment)
        },
        Ok(None) => {
            info!("Comment with ID {} not found", comment_id);
            HttpResponse::NotFound().json(json!({
                "status": "fail",
                "message": "Comment not found",
            }))
        },
        Err(e) => {
            error!("Failed to fetch comment with ID {}: {:?}", comment_id, e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to fetch comment",
            }))
        }
    }
}

#[post("/comments")]
async fn create_comment(
    data: web::Data<AppState>,
    body: web::Json<CreateCommentRequest>,
    _jwt_guard: JwtMiddleware
) -> impl Responder {
    info!("Creating comment for post ID: {}", body.id_post_comment);
    match data
        .di_container
        .comment_service
        .create_comment(&body)
        .await
    {
        Ok(comment) => {
            info!("Successfully created comment: {:?}", comment);
            HttpResponse::Created().json(comment)
        },
        Err(e) => {
            error!("Failed to create comment: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to create comment",
            }))
        }
    }
}

#[put("/comments/{id}")]
async fn update_comment(
    data: web::Data<AppState>,
    body: web::Json<UpdateCommentRequest>,
    _jwt_guard: JwtMiddleware
) -> impl Responder {
    info!("Updating comment with ID: {}", body.id_post_comment);
    match data
        .di_container
        .comment_service
        .update_comment(&body)
        .await
    {
        Ok(Some(comment)) => {
            info!("Successfully updated comment: {:?}", comment);
            HttpResponse::Ok().json(comment)
        },
        Ok(None) => {
            info!("Comment with ID {} not found for update", body.id_post_comment);
            HttpResponse::NotFound().json(json!({
                "status": "fail",
                "message": "Comment not found",
            }))
        },
        Err(e) => {
            error!("Failed to update comment with ID {}: {:?}", body.id_post_comment, e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to update comment",
            }))
        }
    }
}

#[delete("/comments/{id}")]
async fn delete_comment(data: web::Data<AppState>, id: web::Path<i32>, _jwt_guard: JwtMiddleware) -> impl Responder {
    let comment_id = id.into_inner();
    
    info!("Deleting comment with ID: {}", comment_id);



    match data
        .di_container
        .comment_service
        .delete_comment(comment_id)
        .await
    {
        Ok(_) => {
            info!("Successfully deleted comment with ID: {}", comment_id);
            HttpResponse::Ok().json(json!({
                "status": "success",
                "message": "Comment deleted successfully",
            }))
        },
        Err(e) => {
            error!("Failed to delete comment with ID {}: {:?}", comment_id, e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to delete comment",
            }))
        }
    }
}
