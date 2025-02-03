use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "recipes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub archive_name: String,
    pub path_in_archive: String,
    pub namespace: Option<String>,
    pub recipe_type: String,
    #[sea_orm(column_type = "Json")]
    pub recipe_data: Value,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
