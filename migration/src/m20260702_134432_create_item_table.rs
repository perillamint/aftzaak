use sea_orm_migration::prelude::*;
use sea_query::Expr;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260702_134932_create_item_table"
    }
}

#[derive(DeriveIden)]
pub(crate) enum Item {
    Table,
    Id,
    Title,
    MimeType,
    SizeBytes,
    Checksum,
    StorageUri,
    Metadata,
    CreatedAt,
    UpdatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Item::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Item::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Item::Title).text().not_null())
                    .col(ColumnDef::new(Item::MimeType).text())
                    .col(ColumnDef::new(Item::SizeBytes).big_integer())
                    .col(ColumnDef::new(Item::Checksum).text())
                    .col(ColumnDef::new(Item::StorageUri).text())
                    .col(ColumnDef::new(Item::Metadata).json_binary())
                    .col(
                        ColumnDef::new(Item::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Item::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // GIN index on item.metadata
        manager
            .create_index(
                Index::create()
                    .name("idx__item__metadata")
                    .table(Item::Table)
                    .col(Item::Metadata)
                    .full_text()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx__item__created_at")
                    .table(Item::Table)
                    .col(Item::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx__item__updated_at")
                    .table(Item::Table)
                    .col(Item::UpdatedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Item::Table).to_owned())
            .await?;
        Ok(())
    }
}
