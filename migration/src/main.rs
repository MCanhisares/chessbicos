mod migrations;
use std::env;

use migration::Migrator;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::prelude::*;

async fn db_connector() -> Result<DatabaseConnection, DbErr> {
    let db_url = env::var("DB_URL").unwrap_or_else(|_| "".to_string());
    let db_opt = ConnectOptions::new(db_url);
    let db: DatabaseConnection = Database::connect(db_opt).await?;
    println!("Database connected: {:?}", db);
    Ok(db)
}

async fn run(db: &DatabaseConnection) -> Result<(), DbErr> {
    let schema_manager = SchemaManager::new(db); // To investigate the schema
    Migrator::refresh(db).await?;
    assert!(schema_manager.has_table("users").await?);
    assert!(schema_manager.has_table("time_control").await?);
    assert!(schema_manager.has_table("game").await?);

    Ok(())
}

pub fn main() {
    async_std::task::block_on(async {
        let db = db_connector().await.unwrap();
        run(&db).await.unwrap();
    });
}
