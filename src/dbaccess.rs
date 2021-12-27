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
pub fn get_recipe(name: &str) -> Recipe {
    let connection = establish_connection();
    let recipe_result = recipes::table.filter(recipes::recipe_name.like(name))
        .limit(1)
        .load::<RecipeModel>(&connection)
        .expect("Error loading recipe");

    // todo: we will panic here if no rows returned, find better way to get one row
    // todo: find out if there is a better way to join over foreign keys
    let recipe_id = recipe_result[0].id;
    let ingredients_result = ingredients::table.filter(ingredients::recipe_id.eq(recipe_id))
        .load::<IngredientModel>(&connection)
        .expect("Error loading ingredients");
    
    println!("Displaying {} recipes", recipe_result.len());
    println!("Displaying {} ingredients", ingredients_result.len());
    for i in ingredients_result {
        println!("{}\n", i.ingredient_name);
    }

    // todo: populate from queried data
    Recipe::new("test")
}