use sea_orm_migration::prelude::*;
use sea_query::Expr;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260702_134555_create_facet_table"
    }
}

#[derive(DeriveIden)]
pub(crate) enum Facet {
    Table,
    Id,
    Key,
    DisplayName,
    ValueType,
    IsMultiValue,
    SortOrder,
    CreatedAt,
    UpdatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Facet::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Facet::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Facet::Key).text().not_null().unique_key())
                    .col(ColumnDef::new(Facet::DisplayName).text().not_null())
                    .col(ColumnDef::new(Facet::ValueType).text().not_null())
                    .col(
                        ColumnDef::new(Facet::IsMultiValue)
                            .boolean()
                            .not_null()
                            .default(Expr::value(false)),
                    )
                    .col(
                        ColumnDef::new(Facet::SortOrder)
                            .integer()
                            .not_null()
                            .default(Expr::value(0i32)),
                    )
                    .col(
                        ColumnDef::new(Facet::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Facet::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .check((
                        "ck__facet__value_type",
                        Expr::col(Facet::ValueType).is_in([
                            "text",
                            "number",
                            "boolean",
                            "timestamp",
                        ]),
                    ))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx__facet__created_at")
                    .table(Facet::Table)
                    .col(Facet::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx__facet__updated_at")
                    .table(Facet::Table)
                    .col(Facet::UpdatedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Facet::Table).to_owned())
            .await?;
        Ok(())
    }
}
