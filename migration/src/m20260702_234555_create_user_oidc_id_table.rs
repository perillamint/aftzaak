use super::m20260702_130849_create_user_table::User;
use sea_orm_migration::prelude::*;
use sea_query::Expr;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260702_234555_create_user_oidc_id_table"
    }
}

#[derive(DeriveIden)]
enum UserOidcId {
    Table,
    Id,
    UserId,
    Issuer,
    Subject,
    CreatedAt,
    UpdatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserOidcId::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserOidcId::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserOidcId::UserId).uuid().not_null())
                    .col(ColumnDef::new(UserOidcId::Issuer).text().not_null())
                    .col(ColumnDef::new(UserOidcId::Subject).text().not_null())
                    .col(
                        ColumnDef::new(UserOidcId::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(UserOidcId::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk__user_oidc_id__user")
                            .from(UserOidcId::Table, UserOidcId::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Unique OIDC ID pair
        manager
            .create_index(
                Index::create()
                    .name("uq__user_oidc_id__issuer_subject")
                    .table(UserOidcId::Table)
                    .col(UserOidcId::Issuer)
                    .col(UserOidcId::Subject)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx__user_oidc_id__user_id")
                    .table(UserOidcId::Table)
                    .col(UserOidcId::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx__user_oidc_id__created_at")
                    .table(UserOidcId::Table)
                    .col(UserOidcId::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx__user_oidc_id__updated_at")
                    .table(UserOidcId::Table)
                    .col(UserOidcId::UpdatedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserOidcId::Table).to_owned())
            .await?;
        Ok(())
    }
}
