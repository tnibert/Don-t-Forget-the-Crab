mod conversion;
use conversion::*;
use std::collections::HashMap;

// todo: remove unnecessary clone() calls and use the borrow checker correctly
// todo: refactor logic cleanly out of main.rs

fn main() {
    // create units
    //let mg = base_unit(&UnitType::Weight);

    let g = Unit {
        name: String::from("g"),
        relative_to_base: 1000.0,
        measuring: UnitType::Weight
    };
    
    let kg = Unit {
        name: String::from("kg"),
        relative_to_base: 1000.0 * 1000.0,
        measuring: UnitType::Weight
    };
    
    let num = base_unit(&UnitType::Count);

    // create recipes
    // todo: store and retrieve recipes from database
    let mut eggsandtoast = Recipe::new();
    eggsandtoast.ingredients.push(Ingredient {
        name: "egg".to_string(),
        amount: 2.0,
        unit: num.clone()
    });
    eggsandtoast.ingredients.push(Ingredient {
        name: "bread".to_string(),
        amount: 2.0,
        unit: num.clone()
    });
    eggsandtoast.ingredients.push(Ingredient {
        name: "cheese".to_string(),
        // holy shit that's some cheesy toast
        amount: 2.0,
        unit: kg.clone()
    });

    let mut eggmuffin = Recipe::new();
    eggmuffin.ingredients.push(Ingredient {
        name: "egg".to_string(),
        amount: 1.0,
        unit: num.clone()
    });
    eggmuffin.ingredients.push(Ingredient {
        name: "english muffin".to_string(),
        amount: 1.0,
        unit: num.clone()
    });
    eggmuffin.ingredients.push(Ingredient {
        name: "cheese".to_string(),
        amount: 5.0,
        unit: g.clone()
    });

    // all recipes to add to shopping list
    let recipes = [eggsandtoast, eggmuffin];

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

    // todo: create a unit test
    for item in grocery_list {
        println!("{:?}", item);
    }
}
