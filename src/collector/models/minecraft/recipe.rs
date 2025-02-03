use std::collections::HashMap;

use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
#[serde(tag = "type")]
pub enum Recipe {
    /// Represents a shaped crafting recipe in a crafting table.
    #[serde(rename = "minecraft:crafting_shaped")]
    Shaped {
        /// Controls to which recipe book category the recipe belongs to.
        category: Option<String>,

        /// Used to group multiple recipes together in the recipe book.
        /// Recipes with same group identifier but different categories splits into two different groups.
        group: Option<String>,

        /// A list of single-character keys used to describe a pattern for shaped crafting.
        /// Each row in the crafting grid is one string in this list containing 3 or less keys.
        ///
        /// All strings in this list need to have the same amount of keys.
        /// A space can be used to indicate an empty spot.
        pattern: Vec<String>,

        /// All keys used for this shaped crafting recipe.
        key: HashMap<String, IngredientEntry>,

        /// The output item of the recipe.
        result: RecipeResult,

        /// Determines if a notification is shown when unlocking the recipe.
        show_notification: Option<bool>,
    },

    /// Represents a shapeless crafting recipe in a crafting table.
    #[serde(rename = "minecraft:crafting_shapeless")]
    Shapeless {
        /// Controls to which recipe book category the recipe belongs to.
        category: Option<String>,

        /// Used to group multiple recipes together in the recipe book.
        /// Recipes with same group identifier but different categories splits into two different groups.
        group: Option<String>,

        /// A list of entries for this shapeless crafting recipe. Must have 1 to 9 entries.
        ingredients: Vec<IngredientEntry>,

        /// The output item of the recipe.
        result: RecipeResult,
    },

    /// Represents a recipe in a stonecutter.
    #[serde(rename = "minecraft:stonecutting")]
    Stonecutting {
        /// The ingredient for the recipe.
        ingredient: KeyValue,
        result: String,
    },

    /// Represents a recipe in a blast furnace.
    #[serde(rename = "minecraft:blasting")]
    Blasting {
        /// Controls to which recipe book category the recipe belongs to.
        category: Option<String>,

        /// Used to group multiple recipes together in the recipe book.
        /// Recipes with same group identifier but different categories splits into two different groups.
        group: Option<String>,

        /// The ingredient for the recipe.
        ingredient: IngredientEntry,

        /// The cook time of the recipe in ticks.
        #[serde(rename = "cookingTime")]
        cooking_time: Option<i32>,

        /// The output item of the recipe.
        result: String,

        /// The output experience of the recipe.
        experience: f64,
    },

    /// Represents a recipe in a smoker.
    #[serde(rename = "minecraft:smoking")]
    Smoking {
        /// Controls to which recipe book category the recipe belongs to.
        category: Option<String>,

        /// Used to group multiple recipes together in the recipe book.
        /// Recipes with same group identifier but different categories splits into two different groups.
        group: Option<String>,

        /// The ingredient for the recipe.
        ingredient: IngredientEntry,

        /// The cook time of the recipe in ticks.
        #[serde(rename = "cookingTime")]
        cooking_time: Option<i32>,

        /// The output item of the recipe.
        result: String,

        /// The output experience of the recipe.
        experience: f64,
    },

    /// Represents a recipe in a furnace.
    #[serde(rename = "minecraft:smelting")]
    Smelting {
        /// Controls to which recipe book category the recipe belongs to.
        category: Option<String>,

        /// Used to group multiple recipes together in the recipe book.
        /// Recipes with same group identifier but different categories splits into two different groups.
        group: Option<String>,

        /// The ingredient for the recipe.
        ingredient: IngredientEntry,

        /// The cook time of the recipe in ticks.
        #[serde(rename = "cookingTime")]
        cooking_time: Option<i32>,

        /// The output item of the recipe.
        result: String,

        /// The output experience of the recipe.
        experience: f64,
    },

    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, FromJsonQueryResult)]
#[serde(untagged)]
pub enum IngredientEntry {
    Object(KeyValue),
    Array(Vec<KeyValue>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, FromJsonQueryResult)]
#[serde(untagged)]
pub enum KeyValue {
    Tag { tag: String },
    Item { item: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct RecipeResult {
    pub item: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u8>,
    // TODO: This is missing the components field, https://minecraft.wiki/w/Data_component_format
}
