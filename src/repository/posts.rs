use crate::abstract_trait::PostsRepositoryTrait;
use crate::domain::{CreatePostRequest, PostRelationResponse, UpdatePostRequest};
use crate::entities::{comments, posts};
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, QueryFilter,
    Set,
};

pub struct PostRepository {
    db_pool: DatabaseConnection,
}

impl PostRepository {
    pub fn new(db_pool: DatabaseConnection) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl PostsRepositoryTrait for PostRepository {
    async fn get_all_posts(&self) -> Result<Vec<posts::Model>, DbErr> {
        posts::Entity::find().all(&self.db_pool).await
    }

    async fn get_post(&self, post_id: i32) -> Result<Option<posts::Model>, DbErr> {
        posts::Entity::find_by_id(post_id).one(&self.db_pool).await
    }

    async fn get_post_relation(&self, post_id: i32) -> Result<Vec<PostRelationResponse>, DbErr> {
        let post_with_comments = posts::Entity::find()
            .filter(posts::Column::Id.eq(post_id))
            .find_with_related(comments::Entity)
            .all(&self.db_pool)
            .await?;

        let result = post_with_comments
            .into_iter()
            .flat_map(|(post, comments)| {
                comments.into_iter().map(move |comment| {
                    PostRelationResponse::from_post_and_comment(&post, &comment)
                })
            })
            .collect::<Vec<_>>();

        Ok(result)
    }

    async fn create_post(&self, input: &CreatePostRequest) -> Result<posts::Model, DbErr> {
        let new_post = posts::ActiveModel {
            title: Set(input.title.to_string()),
            body: Set(input.body.to_string()),
            img: Set(input.img.to_string()),
            category_id: Set(input.category_id),
            user_id: Set(input.user_id),
            user_name: Set(input.user_name.to_string()),
            ..Default::default()
        };

        new_post.insert(&self.db_pool).await
    }

    async fn update_post(&self, input: &UpdatePostRequest) -> Result<posts::Model, DbErr> {
        let post = posts::Entity::find_by_id(input.post_id)
            .one(&self.db_pool)
            .await?
            .ok_or(DbErr::RecordNotFound("Post not found".to_owned()))?;

        let mut post: posts::ActiveModel = post.into();
        post.title = Set(input.title.to_string());
        post.body = Set(input.body.to_string());
        post.img = Set(input.img.to_string());
        post.category_id = Set(input.category_id);
        post.user_id = Set(input.user_id);
        post.user_name = Set(input.user_name.to_string());

        post.update(&self.db_pool).await
    }

    async fn delete_post(&self, post_id: i32) -> Result<(), DbErr> {
        let post = posts::Entity::find_by_id(post_id)
            .one(&self.db_pool)
            .await?
            .ok_or(DbErr::RecordNotFound("Post not found".to_owned()))?;

        post.delete(&self.db_pool).await?;
        Ok(())
    }
}
