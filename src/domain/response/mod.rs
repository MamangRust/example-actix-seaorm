use serde::Serialize;
use core::fmt;
use std::fmt::Formatter;

mod category;
mod post;
mod comment;
mod user;

pub use self::category::CategoryResponse;
pub use self::post::{
    PostResponse,
    PostRelationResponse
};
pub use self::comment::CommentResponse;
pub use self::user::UserResponse;


#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub message: String,
    pub data: T,
}

impl<T: std::fmt::Debug> fmt::Display for ApiResponse<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ApiResponse {{ status: {}, message: {}, data: {:?} }}",
            self.status,
            self.message,
            self.data
        )
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}