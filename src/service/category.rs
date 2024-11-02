use crate::{abstract_trait::{CategoryServiceTrait, DynCategoryRepository, DynCategoryService}, domain::{ApiResponse, CategoryResponse, CreateCategoryRequest, UpdateCategoryRequest}, repository, utils::AppError};
use async_trait::async_trait;


pub struct CategoryService{
    repository: DynCategoryRepository
}

impl CategoryService{
    pub fn new(repository: DynCategoryRepository) -> Self{
        Self { repository }
    }
}

#[async_trait]
impl CategoryServiceTrait for CategoryService {
    async fn get_categories(&self) -> Result<Vec<ApiResponse<CategoryResponse>>, AppError> {
        let categories = self.repository.find_all().await.map_err(|e| {
            eprintln!("Error fetching categories: {:?}", e);
            AppError::DbError(e)
        })?;
        
        let response = categories.into_iter().map(|category| {
            ApiResponse {
                status: "success".to_string(),
                message: "Categories retrieved successfully".to_string(),
                data: CategoryResponse::from(category),
            }
        }).collect();
        
        Ok(response)
    }

    async fn get_category(&self, id: i32) -> Result<Option<ApiResponse<CategoryResponse>>, AppError> {
        let category = self.repository.find_by_id(id).await.map_err(|e| {
            eprintln!("Error fetching category with id {}: {:?}", id, e);
            AppError::DbError(e)
        })?;
        
        if let Some(category) = category {
            Ok(Some(ApiResponse {
                status: "success".to_string(),
                message: "Category retrieved successfully".to_string(),
                data: CategoryResponse::from(category),
            }))
        } else {
            Err(AppError::NotFound(format!("Category with id {} not found", id)))
        }
    }

    async fn create_category(&self, input: &CreateCategoryRequest) -> Result<ApiResponse<CategoryResponse>, AppError> {

        
        let category = self.repository.create(input).await.map_err(|e| {
            eprintln!("Error creating category: {:?}", e);
            AppError::DbError(e)
        })?;
        
        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Category created successfully".to_string(),
            data: CategoryResponse::from(category),
        })
    }

    async fn update_category(&self, input: &UpdateCategoryRequest) -> Result<Option<ApiResponse<CategoryResponse>>, AppError> {
        
        
        let category = self.repository.update(input).await.map_err(|e| {
            eprintln!("Error updating category with id {}: {:?}", input.id, e);
            AppError::DbError(e)
        })?;
        
        Ok(Some(ApiResponse {
            status: "success".to_string(),
            message: "Category updated successfully".to_string(),
            data: CategoryResponse::from(category),
        }))
    }

    async fn delete_category(&self, id: i32) -> Result<ApiResponse<()>, AppError> {
        self.repository.delete(id).await.map_err(|e| {
            eprintln!("Error deleting category with id {}: {:?}", id, e);
            AppError::DbError(e)
        })?;
        
        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Category deleted successfully".to_string(),
            data: (),
        })
    }
}

