#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;
mod m20231103_114510_notes;

mod m20231224_205059_measures;
mod m20240101_213454_drop_singular_name;

mod m20240101_215625_rename_measure_name_column;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20231103_114510_notes::Migration),
            Box::new(m20231224_205059_measures::Migration),
            Box::new(m20240101_213454_drop_singular_name::Migration),
            Box::new(m20240101_215625_rename_measure_name_column::Migration),
        ]
    }
}