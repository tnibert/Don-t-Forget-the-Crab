#[derive(Queryable)]
pub struct RecipeModel {
    pub id: i32,
    pub recipe_name: String,
}

#[derive(Queryable)]
pub struct IngredientModel {
    pub id: i32,
    pub recipe_id: i32,
    pub ingredient_name: String,
    pub amount: f32,
    pub unit: String
}