mod migrations;
use migrations::*;
pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240414_000001_create_users_table::Migration),
            Box::new(m20240414_000002_create_time_control_table::Migration),
            Box::new(m20240414_000003_create_game_table::Migration),
        ]
    }
}
