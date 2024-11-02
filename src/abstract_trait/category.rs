use std::sync::Arc;

use sea_orm::DbErr;
use async_trait::async_trait;

use crate::{domain::{ApiResponse, CategoryResponse, CreateCategoryRequest, UpdateCategoryRequest}, entities::{categories, Categories}, utils::AppError};


pub type DynCategoryRepository = Arc<dyn CategoryRepositoryTrait + Send + Sync>;
pub type DynCategoryService = Arc<dyn CategoryServiceTrait + Send + Sync>;



#[async_trait]
pub trait CategoryRepositoryTrait {
    async fn find_all(&self) -> Result<Vec<categories::Model>, DbErr>;
    async fn find_by_id(&self, id: i32) -> Result<Option<categories::Model>, DbErr>;
    async fn create(&self, input: &CreateCategoryRequest) -> Result<categories::Model, DbErr>;
    async fn update(&self, input: &UpdateCategoryRequest) -> Result<categories::Model, DbErr>;
    async fn delete(&self, id: i32) -> Result<(), DbErr>;
}

#[async_trait]
pub trait CategoryServiceTrait {
    async fn get_categories(&self) -> Result<Vec<ApiResponse<CategoryResponse>>, AppError>;
    async fn get_category(&self, id: i32) -> Result<Option<ApiResponse<CategoryResponse>>, AppError>;
    async fn create_category(&self, input: &CreateCategoryRequest) -> Result<ApiResponse<CategoryResponse>, AppError>;
    async fn update_category(&self, input: &UpdateCategoryRequest) -> Result<Option<ApiResponse<CategoryResponse>>, AppError>;
    async fn delete_category(&self, id: i32) -> Result<ApiResponse<()>, AppError>;
}