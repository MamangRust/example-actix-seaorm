use crate::{
    domain::{CreatePostRequest, UpdatePostRequest},
    state::AppState, utils::AppError,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;

#[get("/posts")]
async fn get_posts(data: web::Data<AppState>) -> impl Responder {
    match data.di_container.post_service.get_all_posts().await {
        Ok(posts) => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Posts fetched successfully",
            "data": posts
        })),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to fetch posts",
        })),
    }
}

#[get("/posts/{id}")]
async fn get_post(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    match data
        .di_container
        .post_service
        .get_post(id.into_inner())
        .await
    {
        Ok((api_response)) => HttpResponse::Ok().json(api_response),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to fetch post",
        })),
    }
}

#[get("/posts/{id}/relation")]
async fn get_post_relation(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    match data
        .di_container
        .post_service
        .get_post_relation(id.into_inner())
        .await
    {
        Ok(api_response) => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Post relation fetched successfully",
            "data": api_response
        })),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to fetch post relation",
        })),
    }
}



#[post("/posts")]
async fn create_post(
    data: web::Data<AppState>,
    body: web::Json<CreatePostRequest>,
) -> impl Responder {
    match data.di_container.post_service.create_post(&body).await {
        Ok(post) => HttpResponse::Created().json(post),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to create post",
        })),
    }
}
#[put("/posts/{id}")]
async fn update_post(
    data: web::Data<AppState>,
    id: web::Path<i32>,
    body: web::Json<UpdatePostRequest>,
) -> impl Responder {
    let mut post_request = body.into_inner();
    post_request.post_id = id.into_inner();
    
    match data.di_container.post_service.update_post(&post_request).await {
        Ok(api_response) => HttpResponse::Ok().json(api_response),
       
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to update post",
        })),
    }
}


#[delete("/posts/{id}")]
async fn delete_post(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    match data
        .di_container
        .post_service
        .delete_post(id.into_inner())
        .await
    {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to delete post",
        })),
    }
}