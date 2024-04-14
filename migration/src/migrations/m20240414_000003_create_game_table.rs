use sea_orm_migration::prelude::*;

use super::{m20240414_000001_create_users_table::Users, m20240414_000002_create_time_control_table::TimeControl};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240414_000001_create_match_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Game::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Game::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Game::PlayerBlack).integer().not_null())
                    .col(ColumnDef::new(Game::PlayerWhite).integer().not_null())
                    .col(ColumnDef::new(Game::TimeControl).integer().not_null())
                    .col(ColumnDef::new(Game::Board).string().not_null())
                    .col(ColumnDef::new(Game::Turn).string().not_null().default("w"))
                    .col(ColumnDef::new(Game::BlackTime).integer().not_null())
                    .col(ColumnDef::new(Game::WhiteTime).integer().not_null())
                    .col(
                        ColumnDef::new(Game::State)
                            .string()
                            .not_null()
                            .default("active"),
                    )
                    .col(ColumnDef::new(Game::Moves).string())
                    .col(
                        ColumnDef::new(Game::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(ColumnDef::new(Game::UpdatedAt).timestamp())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_player_black")
                            .from(Game::Table, Game::PlayerBlack)
                            .to(Users::Table, Users::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_player_white")
                            .from(Game::Table, Game::PlayerWhite)
                            .to(Users::Table, Users::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_time_control")
                            .from(Game::Table, Game::TimeControl)
                            .to(TimeControl::Table, TimeControl::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Bakery table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Game::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Game {
    Table,
    Id,
    PlayerBlack,
    PlayerWhite,
    TimeControl,
    Board,
    Turn,
    BlackTime,
    WhiteTime,
    State,
    Moves,
    CreatedAt,
    UpdatedAt,
}
