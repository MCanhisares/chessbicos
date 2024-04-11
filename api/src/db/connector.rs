use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, DbErr, Statement,
};
use std::env;

pub async fn db_connector() -> Result<DatabaseConnection, DbErr> {
    let db_url = env::var("DB_URL").unwrap_or_else(|_| "".to_string());
    let db_opt = ConnectOptions::new(db_url);
    let db: DatabaseConnection = Database::connect(db_opt).await?;
    println!("Database connected: {:?}", db);
    Ok(db)
}

pub async fn create_db(db: DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let db_url = env::var("DB_URL").unwrap_or_else(|_| "".to_string());
    let db_name = env::var("DB_NAME").unwrap_or_else(|_| "".to_string());
    let db = &match db.get_database_backend() {
        DbBackend::Postgres => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("DROP DATABASE IF EXISTS \"{}\";", db_name),
            ))
            .await?;
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", db_name),
            ))
            .await?;

            let url = format!("{}/{}", db_url, db_name);
            Database::connect(&url).await?
        }
        DbBackend::Sqlite => db,
        DbBackend::MySql => db,
    };
    Ok(())
}
