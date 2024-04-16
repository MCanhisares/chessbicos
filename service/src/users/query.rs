use ::entity::entities::{users, users::Entity as Users};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_user_by_id(db: &DbConn, id: i32) -> Result<Option<users::Model>, DbErr> {
        Users::find_by_id(id).one(db).await
    }

    pub async fn find_user_by_username(
        db: &DbConn,
        username: &str,
    ) -> Result<Option<users::Model>, DbErr> {
        Users::find()
            .filter(users::Column::Username.contains(username))
            .one(db)
            .await
    }

    pub async fn find_user_by_username_password(
        db: &DbConn,
        username: &str,
        password: &str,
    ) -> Result<Option<users::Model>, DbErr> {
        Users::find()
            .filter(users::Column::Username.contains(username))
            .filter(users::Column::Password.contains(password))
            .one(db)
            .await
    }

    // If ok, returns (post models, num pages).
    // pub async fn find_posts_in_page(
    //     db: &DbConn,
    //     page: u64,
    //     posts_per_page: u64,
    // ) -> Result<(Vec<post::Model>, u64), DbErr> {
    //     // Setup paginator
    //     let paginator = Post::find()
    //         .order_by_asc(post::Column::Id)
    //         .paginate(db, posts_per_page);
    //     let num_pages = paginator.num_pages().await?;

    //     // Fetch paginated posts
    //     paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    // }
}
