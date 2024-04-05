use sea_orm::{Database, DatabaseConnection, ConnectOptions, DbErr};

pub async fn db_connector() -> Result<DatabaseConnection, DbErr> {
    let db_opt = ConnectOptions::new("postgres://postgres:admin@localhost:5432");
    let db: DatabaseConnection = Database::connect(db_opt).await?;
    println!("Database connected: {:?}", db);
    Ok(db)
}