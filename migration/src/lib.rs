pub use sea_orm_migration::prelude::*;

mod m20260702_130849_create_user_table;
mod m20260702_134932_create_item_table;
mod m20260702_134555_create_facet_table;
mod m20260702_135010_create_item_facet_value_table;
mod m20260702_234555_create_user_oidc_id_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260702_130849_create_user_table::Migration),
            Box::new(m20260702_134932_create_item_table::Migration),
            Box::new(m20260702_134555_create_facet_table::Migration),
            Box::new(m20260702_135010_create_item_facet_value_table::Migration),
            Box::new(m20260702_234555_create_user_oidc_id_table::Migration),
        ]
    }
}
