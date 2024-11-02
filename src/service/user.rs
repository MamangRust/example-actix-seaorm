use crate::{
    abstract_trait::{UserServiceTrait, DynUserRepository, DynUserService},
    domain::{ApiResponse, UserResponse, CreateUserRequest, UpdateUserRequest},
    utils::AppError,
};
use async_trait::async_trait;

pub struct UserService {
    repository: DynUserRepository,
}

impl UserService {
    pub fn new(repository: DynUserRepository) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UserServiceTrait for UserService {
    async fn create_user(
        &self,
        input: &CreateUserRequest,
    ) -> Result<ApiResponse<UserResponse>, AppError> {
        let user = self.repository.create_user(input).await.map_err(|e| {
            eprintln!("Error creating user: {:?}", e);
            AppError::DbError(e)
        })?;
        
        Ok(ApiResponse {
            status: "success".to_string(),
            message: "User created successfully".to_string(),
            data: UserResponse::from(user),
        })
    }

    async fn find_by_email_exists(&self, email: &str) -> Result<ApiResponse<bool>, AppError> {
        let exists = self.repository.find_by_email_exists(email).await.map_err(|e| {
            eprintln!("Error checking if user exists by email: {:?}", e);
            AppError::DbError(e)
        })?;
        
        Ok(ApiResponse {
            status: "success".to_string(),
            message: if exists { "User exists" } else { "User does not exist" }.to_string(),
            data: exists,
        })
    }
    

    async fn find_user_by_email(
        &self,
        email: &str,
    ) -> Result<Option<ApiResponse<UserResponse>>, AppError> {
        let user = self.repository.find_by_email(email).await.map_err(|e| {
            eprintln!("Error fetching user by email: {:?}", e);
            AppError::DbError(e)
        })?;
        
        if let Some(user) = user {
            Ok(Some(ApiResponse {
                status: "success".to_string(),
                message: "User retrieved successfully".to_string(),
                data: UserResponse::from(user),
            }))
        } else {
            Err(AppError::NotFound(format!("User with email {} not found", email)))
        }
    }

    async fn find_by_id(
        &self,
        id: i32,
    ) -> Result<Option<ApiResponse<UserResponse>>, AppError> {
        let user = self.repository.find_by_id(id).await.map_err(|e| {
            eprintln!("Error fetching user with id {}: {:?}", id, e);
            AppError::DbError(e)
        })?;
        
        if let Some(user) = user {
            Ok(Some(ApiResponse {
                status: "success".to_string(),
                message: "User retrieved successfully".to_string(),
                data: UserResponse::from(user),
            }))
        } else {
            Err(AppError::NotFound(format!("User with id {} not found", id)))
        }
    }

    async fn update_user(
        &self,
        input: &UpdateUserRequest,
    ) -> Result<Option<ApiResponse<UserResponse>>, AppError> {
        let user = self.repository.update_user(input).await.map_err(|e| {
            eprintln!("Error updating user with id {}: {:?}", input.id, e);
            AppError::DbError(e)
        })?;
        
        Ok(Some(ApiResponse {
            status: "success".to_string(),
            message: "User updated successfully".to_string(),
            data: UserResponse::from(user),
        }))
    }

    async fn delete_user(&self, email: &str) -> Result<ApiResponse<()>, AppError> {
        self.repository.delete_user(email).await.map_err(|e| {
            eprintln!("Error deleting user with email {}: {:?}", email, e);
            AppError::DbError(e)
        })?;
        
        Ok(ApiResponse {
            status: "success".to_string(),
            message: "User deleted successfully".to_string(),
            data: (),
        })
    }
}
