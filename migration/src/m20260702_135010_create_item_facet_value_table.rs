use super::m20260702_134432_create_item_table::Item;
use super::m20260702_134555_create_facet_table::Facet;
use sea_orm_migration::prelude::*;
use sea_query::Expr;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260702_135010_create_item_facet_value_table"
    }
}

#[derive(DeriveIden)]
enum ItemFacetValue {
    Table,
    Id,
    ItemId,
    FacetId,
    IsMultiValue,
    ValueText,
    ValueNumeric,
    ValueBoolean,
    ValueTimestamp,
    CreatedAt,
    UpdatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ItemFacetValue::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ItemFacetValue::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ItemFacetValue::ItemId).uuid().not_null())
                    .col(ColumnDef::new(ItemFacetValue::FacetId).uuid().not_null())
                    .col(
                        ColumnDef::new(ItemFacetValue::IsMultiValue)
                            .boolean()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ItemFacetValue::ValueText).text())
                    .col(ColumnDef::new(ItemFacetValue::ValueNumeric).decimal())
                    .col(ColumnDef::new(ItemFacetValue::ValueBoolean).boolean())
                    .col(ColumnDef::new(ItemFacetValue::ValueTimestamp).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(ItemFacetValue::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(ItemFacetValue::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk__item_facet_value__item")
                            .from(ItemFacetValue::Table, ItemFacetValue::ItemId)
                            .to(Item::Table, Item::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk__item_facet_value__facet")
                            .from(ItemFacetValue::Table, ItemFacetValue::FacetId)
                            .to(Facet::Table, Facet::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("uq__item_facet_value__single_value")
                    .table(ItemFacetValue::Table)
                    .col(ItemFacetValue::ItemId)
                    .col(ItemFacetValue::FacetId)
                    .unique()
                    .cond_where(Expr::col(ItemFacetValue::IsMultiValue).eq(false))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("uq__item_facet_value__multi_text")
                    .table(ItemFacetValue::Table)
                    .col(ItemFacetValue::ItemId)
                    .col(ItemFacetValue::FacetId)
                    .col(ItemFacetValue::ValueText)
                    .unique()
                    .cond_where(
                        Condition::all()
                            .add(Expr::col(ItemFacetValue::IsMultiValue).eq(true))
                            .add(Expr::col(ItemFacetValue::ValueText).is_not_null()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("uq__item_facet_value__multi_numeric")
                    .table(ItemFacetValue::Table)
                    .col(ItemFacetValue::ItemId)
                    .col(ItemFacetValue::FacetId)
                    .col(ItemFacetValue::ValueNumeric)
                    .unique()
                    .cond_where(
                        Condition::all()
                            .add(Expr::col(ItemFacetValue::IsMultiValue).eq(true))
                            .add(Expr::col(ItemFacetValue::ValueNumeric).is_not_null()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("uq__item_facet_value__multi_boolean")
                    .table(ItemFacetValue::Table)
                    .col(ItemFacetValue::ItemId)
                    .col(ItemFacetValue::FacetId)
                    .col(ItemFacetValue::ValueBoolean)
                    .unique()
                    .cond_where(
                        Condition::all()
                            .add(Expr::col(ItemFacetValue::IsMultiValue).eq(true))
                            .add(Expr::col(ItemFacetValue::ValueBoolean).is_not_null()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("uq__item_facet_value__multi_timestamp")
                    .table(ItemFacetValue::Table)
                    .col(ItemFacetValue::ItemId)
                    .col(ItemFacetValue::FacetId)
                    .col(ItemFacetValue::ValueTimestamp)
                    .unique()
                    .cond_where(
                        Condition::all()
                            .add(Expr::col(ItemFacetValue::IsMultiValue).eq(true))
                            .add(Expr::col(ItemFacetValue::ValueTimestamp).is_not_null()),
                    )
                    .to_owned(),
            )
            .await?;

        // Faceting/counting indexes
        manager
            .create_index(
                Index::create()
                    .name("idx__item_facet_value__facet_id__value_text")
                    .table(ItemFacetValue::Table)
                    .col(ItemFacetValue::FacetId)
                    .col(ItemFacetValue::ValueText)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx__item_facet_value__facet_id__value_numeric")
                    .table(ItemFacetValue::Table)
                    .col(ItemFacetValue::FacetId)
                    .col(ItemFacetValue::ValueNumeric)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx__item_facet_value__facet_id__value_boolean")
                    .table(ItemFacetValue::Table)
                    .col(ItemFacetValue::FacetId)
                    .col(ItemFacetValue::ValueBoolean)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx__item_facet_value__facet_id__value_timestamp")
                    .table(ItemFacetValue::Table)
                    .col(ItemFacetValue::FacetId)
                    .col(ItemFacetValue::ValueTimestamp)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx__item_facet_value__item_id")
                    .table(ItemFacetValue::Table)
                    .col(ItemFacetValue::ItemId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx__item_facet_value__created_at")
                    .table(ItemFacetValue::Table)
                    .col(ItemFacetValue::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx__item_facet_value__updated_at")
                    .table(ItemFacetValue::Table)
                    .col(ItemFacetValue::UpdatedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ItemFacetValue::Table).to_owned())
            .await?;
        Ok(())
    }
}
