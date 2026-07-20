use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260720_144717_create_role_table"
    }
}

#[derive(DeriveIden)]
pub(crate) enum Role {
    Table,
    Id,
    Name,
    Permissions,
    CreatedAt,
    UpdatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Role::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Role::Name).text().not_null().unique_key())
                    .col(
                        ColumnDef::new(Role::Permissions)
                            .array(ColumnType::Text)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Role::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Role::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx__role__created_at")
                    .table(Role::Table)
                    .col(Role::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx__role__updated_at")
                    .table(Role::Table)
                    .col(Role::UpdatedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await?;
        Ok(())
    }
}
