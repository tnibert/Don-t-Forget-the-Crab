use crate::schema::recipes;
use crate::schema::ingredients;

// todo: evaluate db for further normalization

#[derive(Queryable, Selectable)]
#[diesel(table_name = recipes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct RecipeModel {
    pub id: i32,
    pub recipe_name: String,
    pub notes: Option<String>
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = ingredients)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct IngredientModel {
    pub id: i32,
    pub recipe_id: i32,
    pub ingredient_name: String,
    pub amount: f32,
    pub unit: String,
    pub notes: Option<String>
}
