use crate::{
    domain::{CreatePostRequest, UpdatePostRequest}, middleware::JwtMiddleware, state::AppState
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;
use tracing::{info, error};

#[get("/posts")]
async fn get_posts(data: web::Data<AppState>) -> impl Responder {
    info!("Fetching all posts...");
    match data.di_container.post_service.get_all_posts().await {
        Ok(posts) => {
            info!("Posts fetched successfully: {:?}", posts);
            HttpResponse::Ok().json(json!({
                "status": "success",
                "message": "Posts fetched successfully",
                "data": posts
            }))
        },
        Err(e) => {
            error!("Failed to fetch posts: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to fetch posts",
            }))
        }
    }
}

#[get("/posts/{id}")]
async fn get_post(data: web::Data<AppState>, id: web::Path<i32>, _jwt_guard: JwtMiddleware) -> impl Responder {
    // Clone the `id` to use it later in the log statement
    let post_id = id.into_inner();
    info!("Fetching post with ID: {}", post_id);
    
    match data
        .di_container
        .post_service
        .get_post(post_id)
        .await
    {
        Ok(Some(post)) => {
            info!("Post found: {:?}", post);
            HttpResponse::Ok().json(post)
        },
        Ok(None) => {
            info!("Post with ID {} not found", post_id);
            HttpResponse::NotFound().json(json!({
                "status": "fail",
                "message": "Post not found",
            }))
        },
        Err(e) => {
            error!("Failed to fetch post: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to fetch Post",
            }))
        }
    }
}


#[get("/posts/{id}/relation")]
async fn get_post_relation(data: web::Data<AppState>, id: web::Path<i32>, _jwt_guard: JwtMiddleware) -> impl Responder {
    info!("Fetching post relation for post ID: {}", id);
    match data
        .di_container
        .post_service
        .get_post_relation(id.into_inner())
        .await
    {
        Ok(api_response) => {
            info!("Post relation fetched successfully: {:?}", api_response);
            HttpResponse::Ok().json(api_response)
        },
        Err(e) => {
            error!("Failed to fetch post relation: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to fetch post relation",
            }))
        }
    }
}

#[post("/posts")]
async fn create_post(
    data: web::Data<AppState>,
    body: web::Json<CreatePostRequest>,
    _jwt_guard: JwtMiddleware
) -> impl Responder {
    info!("Creating post with data: {:?}", body);
    match data.di_container.post_service.create_post(&body).await {
        Ok(post) => {
            info!("Post created successfully: {:?}", post);
            HttpResponse::Created().json(post)
        },
        Err(e) => {
            error!("Failed to create post: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to create post",
            }))
        }
    }
}

#[put("/posts/{id}")]
async fn update_post(
    data: web::Data<AppState>,
    id: web::Path<i32>,
    body: web::Json<UpdatePostRequest>,
    _jwt_guard: JwtMiddleware
) -> impl Responder {

    let post_id = id.into_inner();
    let mut post_request = body.into_inner();
    post_request.post_id = Some(post_id);

    info!("Updating post with ID {} and data: {:?}", post_id, post_request);
    match data.di_container.post_service.update_post(&post_request).await {
        Ok(api_response) => {
            info!("Post updated successfully: {:?}", api_response);
            HttpResponse::Ok().json(api_response)
        },
        Err(e) => {
            error!("Failed to update post: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to update post",
            }))
        }
    }
}

#[delete("/posts/{id}")]
async fn delete_post(data: web::Data<AppState>, id: web::Path<i32>, _jwt_guard: JwtMiddleware) -> impl Responder {
    info!("Deleting post with ID: {}", id);
    match data
        .di_container
        .post_service
        .delete_post(id.into_inner())
        .await
    {
        Ok(data) => {
            info!("Post deleted successfully: {:?}", data);
            HttpResponse::Ok().json(data)
        },
        Err(e) => {
            error!("Failed to delete post: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to delete post",
            }))
        }
    }
}
