use super::m20260702_130849_create_user_table::User;
use super::m20260720_144717_create_role_table::Role;
use sea_orm_migration::prelude::*;
use sea_query::Expr;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260720_144727_create_user_role_table"
    }
}

#[derive(DeriveIden)]
enum UserRole {
    Table,
    Id,
    UserId,
    RoleId,
    CreatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserRole::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserRole::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(UserRole::UserId).uuid().not_null())
                    .col(ColumnDef::new(UserRole::RoleId).uuid().not_null())
                    .col(
                        ColumnDef::new(UserRole::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk__user_role__user")
                            .from(UserRole::Table, UserRole::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk__user_role__role")
                            .from(UserRole::Table, UserRole::RoleId)
                            .to(Role::Table, Role::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        // Unique: a user can't have the same role assigned twice
        manager
            .create_index(
                Index::create()
                    .name("uq__user_role__user_id_role_id")
                    .table(UserRole::Table)
                    .col(UserRole::UserId)
                    .col(UserRole::RoleId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Reverse lookup: which users have this role?
        manager
            .create_index(
                Index::create()
                    .name("idx__user_role__role_id")
                    .table(UserRole::Table)
                    .col(UserRole::RoleId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx__user_role__created_at")
                    .table(UserRole::Table)
                    .col(UserRole::CreatedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserRole::Table).to_owned())
            .await?;
        Ok(())
    }
}
