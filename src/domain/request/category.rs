use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Clone,Debug)]
pub struct CreateCategoryRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone,Debug)]
pub struct UpdateCategoryRequest {
    pub id: i32,
    pub name: Option<String>,
}