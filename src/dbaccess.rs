pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use crate::schema::recipes;
use crate::schema::ingredients;
use crate::models::RecipeModel;
use crate::models::IngredientModel;
use crate::recipe::Recipe;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

// todo: populate db and test
pub fn get_recipe(name: &str) -> Option<Recipe> {
    let connection = establish_connection();
    let recipe_result = recipes::table.filter(recipes::recipe_name.like(name))
        .limit(1)
        .load::<RecipeModel>(&connection)
        .expect("Error loading recipe");

    // todo: is there a better way to get first row without explicitly checking len?
    // todo: is there a better way to join over foreign keys?
    if recipe_result.len() == 0 {
        return None;
    }
    let recipe_id = recipe_result[0].id;

    // todo: populate ingredients in recipe
    let ingredients_result = ingredients::table.filter(ingredients::recipe_id.eq(recipe_id))
        .load::<IngredientModel>(&connection)
        .expect("Error loading ingredients");
    
    println!("Displaying {} recipes", recipe_result.len());
    println!("Displaying {} ingredients", ingredients_result.len());
    for i in ingredients_result {
        println!("{}\n", i.ingredient_name);
    }

    Some(Recipe::new(&recipe_result[0].recipe_name))
}