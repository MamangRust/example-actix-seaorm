use std::sync::Arc;

use async_trait::async_trait;

use crate::{domain::{ApiResponse, UserResponse, RegisterRequest, LoginRequest}, utils::AppError};


pub type DynAuthService = Arc<dyn AuthServiceTrait + Send + Sync>;

#[async_trait]
pub trait AuthServiceTrait {
    async fn register_user(
        &self,
        input: &RegisterRequest
    ) -> Result<ApiResponse<UserResponse>, AppError>;
    async fn login_user(&self, input: &LoginRequest) -> Result<ApiResponse<String>, AppError>;
    fn verify_token(&self, token: &str) -> Result<i64, AppError>;
}