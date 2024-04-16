use ::entity::entities::users;
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_user(
        db: &DbConn,
        form_data: users::Model,
    ) -> Result<users::Model, DbErr> {
        users::ActiveModel {
            username: Set(form_data.username.to_owned()),
            password: Set(form_data.password.to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    // pub async fn update_post_by_id(
    //     db: &DbConn,
    //     id: i32,
    //     form_data: post::Model,
    // ) -> Result<post::Model, DbErr> {
    //     let post: post::ActiveModel = Post::find_by_id(id)
    //         .one(db)
    //         .await?
    //         .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
    //         .map(Into::into)?;

    //     post::ActiveModel {
    //         id: post.id,
    //         title: Set(form_data.title.to_owned()),
    //         text: Set(form_data.text.to_owned()),
    //     }
    //     .update(db)
    //     .await
    // }

    // pub async fn delete_post(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
    //     let post: post::ActiveModel = Post::find_by_id(id)
    //         .one(db)
    //         .await?
    //         .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
    //         .map(Into::into)?;

    //     post.delete(db).await
    // }

    // pub async fn delete_all_posts(db: &DbConn) -> Result<DeleteResult, DbErr> {
    //     Post::delete_many().exec(db).await
    // }
}
