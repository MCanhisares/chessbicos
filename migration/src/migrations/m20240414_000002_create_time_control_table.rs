use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240414_000002_create_time_control_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TimeControl::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TimeControl::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TimeControl::DisplayName).string().not_null())
                    .col(ColumnDef::new(TimeControl::Time).integer().not_null())
                    .col(
                        ColumnDef::new(TimeControl::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(ColumnDef::new(TimeControl::UpdatedAt).timestamp())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TimeControl::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TimeControl {
    Table,
    Id,
    DisplayName,
    Time,
    CreatedAt,
    UpdatedAt,
}
