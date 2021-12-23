use std::collections::HashMap;
use crate::recipe::*;
use crate::ingredient::*;
use crate::units::*;

fn recipes_to_map(recipes: Vec<Recipe>) -> HashMap<String, Vec<Ingredient>> {
    // clone all ingredients from recipes into hashmap of ingredient name to vector of that ingredient
    let mut ingredient_map: HashMap<String, Vec<Ingredient>> = HashMap::new();
    for r in recipes.iter() {
        for i in r.ingredients.iter() {
            let myvec = ingredient_map.get_mut(&i.name);
            match myvec {
                // return value of each match arm must be same type, use ; to convert to statement and discard return to ()
                Some(val) => val.push(i.clone()),
                None => {
                    ingredient_map.insert(i.name.clone(), vec![i.clone()]);
                }
            };
        }
    }
    ingredient_map
}

fn map_to_grocery_list(ingredient_map: HashMap<String, Vec<Ingredient>>) -> Vec<Ingredient> {
    // to populate with the final grocery list
    let mut grocery_list: Vec<Ingredient> = Vec::new();
    
    // fold each vector to obtain the amount to buy from the store
    for (key, value) in &ingredient_map {
        let empty = Ingredient {
            name: key.to_string(),
            amount: 0.0,
            unit: base_unit(&value.get(0).unwrap().unit.measuring)
        };
        grocery_list.push(value.iter().fold(empty.clone(), |a, x| a.combine(x)));
    }
    grocery_list
}

pub fn recipes_to_grocery_list(recipes: Vec<Recipe>) -> Vec<Ingredient> {
    let ingredient_map = recipes_to_map(recipes);
    map_to_grocery_list(ingredient_map)
}
