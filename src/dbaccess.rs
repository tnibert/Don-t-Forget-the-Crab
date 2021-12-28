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
use crate::units::get_unit;
use crate::ingredient::Ingredient;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

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
    
    println!("{} recipe", recipe_result.len());
    println!("{} ingredients", ingredients_result.len());

    let mut myrecipe = Recipe::new(&recipe_result[0].recipe_name);
    // todo: populate ingredients in db and test
    for i in ingredients_result {
        println!("{}\n", i.ingredient_name);
        myrecipe.ingredients.push(Ingredient {
            name: i.ingredient_name,
            amount: i.amount,
            unit: get_unit(&i.unit)
        });
    }

    Some(myrecipe)
}

// todo: add function to add a recipe to the database