use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Tag {
    /// Whether or not the contents of this tag should completely replace tag contents from different
    /// lower priority data packs with the same resource location.
    ///
    /// When `false` the tag's content is appended to the contents of the higher priority data packs instead.
    pub replace: Option<bool>,

    /// A list of mix and match of object names and tag names.
    /// For tags, recursive reference is possible, but a circular reference causes a loading failure.
    pub values: Vec<String>,
}
