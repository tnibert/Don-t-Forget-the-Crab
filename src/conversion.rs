use std::collections::HashMap;

// todo: rename this file, organize logically
// todo: encapsulate better, remove unnecessary pubs

// define base units for retrieval
pub fn base_unit(t: &UnitType) -> Unit {
    match t {
        UnitType::Weight => Unit {
            name: "mg".to_string(),
            relative_to_base: 1.0,
            measuring: UnitType::Weight
        },
        UnitType::Volume => Unit {
            name: String::from("ml"),
            relative_to_base: 1.0,
            measuring: UnitType::Volume
        },
        UnitType::Count => Unit {
            name: String::from(""),
            relative_to_base: 1.0,
            measuring: UnitType::Count
        }
    }
}

// todo (maybe): make into an iterator
pub struct Recipe {
    pub ingredients: Vec<Ingredient>
}

impl Recipe {
    // construct with slice of ingredients - effectively variable # args
    pub fn new() -> Recipe {
        //let mut i: Vec<Ingredient> = args.to_vec();
        Self {
            ingredients: Vec::<Ingredient>::new()
        }
    }
}

#[derive(Clone, Debug)]
pub struct Ingredient {
    pub name: String,
    pub amount: f32,
    pub unit: Unit
}

impl Ingredient {
    // todo: use Result, not Panic
    pub fn combine(&self, other: &Ingredient) -> Ingredient { //Result<Ingredient, &'static str> {
        if self.name == other.name {      // && self.unit.measuring == other.unit.measuring
            return Ingredient {
                name: self.name.clone(),
                // normalize the amounts across units
                amount: self.amount * self.unit.relative_to_base + other.amount * other.unit.relative_to_base,
                unit: base_unit(&self.unit.measuring)
            }

        } else {
            // can't combine
            //return Err("these things are not the same");
            panic!("these things are not the same");
        }
    }

    // todo: add new() method
}

#[derive(Clone, Debug)]
pub enum UnitType {
    Weight,
    Volume,
    Count
}

#[derive(Clone, Debug)]
pub struct Unit {
    pub name: String,
    pub relative_to_base: f32,
    pub measuring: UnitType
}

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