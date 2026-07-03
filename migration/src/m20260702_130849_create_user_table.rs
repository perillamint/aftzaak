use sea_orm_migration::prelude::*;
use sea_query::Expr;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260702_130849_create_user_table"
    }
}

#[derive(DeriveIden)]
pub(crate) enum User {
    Table,
    Id,
    Email,
    Password,
    DisplayName,
    State,
    CreatedAt,
    UpdatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // ---- user ----
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(User::Email).text().not_null().unique_key())
                    .col(ColumnDef::new(User::Password).text())
                    .col(ColumnDef::new(User::DisplayName).text())
                    .col(
                        ColumnDef::new(User::State)
                            .boolean()
                            .not_null()
                            .default(Expr::value(true)),
                    )
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Lookup index on email
        manager
            .create_index(
                Index::create()
                    .name("idx__user__email")
                    .table(User::Table)
                    .col(User::Email)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        Ok(())
    }
}
