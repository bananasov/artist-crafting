use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::collector::models::minecraft::Tag;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub archive_name: String,
    pub path_in_archive: String,
    pub namespace: Option<String>,
    pub tag_type: String,
    pub tag_data: Tag,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
