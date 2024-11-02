use crate::{abstract_trait::{CommentServiceTrait, DynCommentRepository, DynCommentService}, domain::{ApiResponse, CommentResponse, CreateCommentRequest, UpdateCommentRequest}, repository, utils::AppError};
use async_trait::async_trait;

pub struct CommentService {
    repository: DynCommentRepository,
}

impl CommentService {
    pub fn new(repository: DynCommentRepository) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl CommentServiceTrait for CommentService {
    async fn get_comments(&self) -> Result<Vec<ApiResponse<CommentResponse>>, AppError> {
        let comments = self.repository.find_all().await.map_err(|e| {
            eprintln!("Error fetching comments: {:?}", e);
            AppError::DbError(e)
        })?;
        
        let response = comments.into_iter().map(|comment| {
            ApiResponse {
                status: "success".to_string(),
                message: "Comments retrieved successfully".to_string(),
                data: CommentResponse::from(comment),
            }
        }).collect();
        
        Ok(response)
    }

    async fn get_comment(&self, id: i32) -> Result<Option<ApiResponse<CommentResponse>>, AppError> {
        let comment = self.repository.find_by_id(id).await.map_err(|e| {
            eprintln!("Error fetching comment with id {}: {:?}", id, e);
            AppError::DbError(e)
        })?;
        
        if let Some(comment) = comment {
            Ok(Some(ApiResponse {
                status: "success".to_string(),
                message: "Comment retrieved successfully".to_string(),
                data: CommentResponse::from(comment),
            }))
        } else {
            Err(AppError::NotFound(format!("Comment with id {} not found", id)))
        }
    }

    async fn create_comment(&self, input: &CreateCommentRequest) -> Result<ApiResponse<CommentResponse>, AppError> {
        let comment = self.repository.create(input).await.map_err(|e| {
            eprintln!("Error creating comment: {:?}", e);
            AppError::DbError(e)
        })?;
        
        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Comment created successfully".to_string(),
            data: CommentResponse::from(comment),
        })
    }

    async fn update_comment(&self, input: &UpdateCommentRequest) -> Result<Option<ApiResponse<CommentResponse>>, AppError> {
        let comment = self.repository.update(input).await.map_err(|e| {
            eprintln!("Error updating comment with id {}: {:?}", input.id_post_comment, e);
            AppError::DbError(e)
        })?;
        
        Ok(Some(ApiResponse {
            status: "success".to_string(),
            message: "Comment updated successfully".to_string(),
            data: CommentResponse::from(comment),
        }))
    }

    async fn delete_comment(&self, id: i32) -> Result<ApiResponse<()>, AppError> {
        self.repository.delete(id).await.map_err(|e| {
            eprintln!("Error deleting comment with id {}: {:?}", id, e);
            AppError::DbError(e)
        })?;
        
        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Comment deleted successfully".to_string(),
            data: (),
        })
    }
}
