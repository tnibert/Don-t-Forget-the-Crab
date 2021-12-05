use std::collections::HashMap;

// todo: remove unnecessary clone() calls and use the borrow checker correctly
// todo: refactor logic cleanly out of main.rs

// todo (maybe): make into an iterator
struct Recipe {
    ingredients: Vec<Ingredient>
}

impl Recipe {
    // construct with slice of ingredients - effectively variable # args
    fn new() -> Recipe {
        //let mut i: Vec<Ingredient> = args.to_vec();
        Self {
            ingredients: Vec::<Ingredient>::new()
        }
    }
}

#[derive(Clone, Debug)]
struct Ingredient {
    name: String,
    amount: f32,
    unit: Unit
}

impl Ingredient {
    // todo: use Result, not Panic
    fn combine(&self, other: &Ingredient) -> Ingredient { //Result<Ingredient, &'static str> {
        if (self.name == other.name) {
            return Ingredient {
                name: self.name.clone(),
                // todo: normalize the amounts across units
                amount: self.amount + other.amount,
                unit: self.unit.clone()
            }

        } else {
            // can't combine
            //return Err("these things are not the same");
            panic!("these things are not the same");
        }
    }
}

#[derive(Clone, Debug)]
enum Unit {
    weight(String, u32),
    volume(String, u32),
    count(String, u32)
}

fn hashmap_example() {
    // hashmap example - for reference
    let mut sample: HashMap<String, i32> = HashMap::new();

    sample.insert("one".to_string(), 1);
    sample.insert("two".to_string(), 2);
    sample.insert("three".to_string(), 3);
    sample.insert("four".to_string(), 4);
    for (key, value) in sample.iter() {
        println!("{} - {}", key, value);
    }
    println!("four: {:?}", sample.get("four").unwrap());
}

fn main() {
    // create units
    let mg = Unit::weight(String::from("mg"), 1);
    let g = Unit::weight(String::from("g"), 1000);
    let kg = Unit::weight(String::from("kg"), 1000 * 1000);
    
    let num = Unit::count(String::from(""), 1);

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
        name: "cheese slice".to_string(),
        amount: 1.0,
        unit: num.clone()
    });

    // all recipes to add to shopping list
    let recipes = [eggsandtoast, eggmuffin];

    // clone all ingredients from recipes into hashmap of ingredient name to vector of that ingredient
    let mut ingredient_map: HashMap<String, Vec<Ingredient>> = HashMap::new();
    for r in recipes.iter() {
        for i in r.ingredients.iter() {
            let myvec = ingredient_map.get_mut(&i.name);
            match myvec {
                Some(val) => val.push(i.clone()),
                None => ingredient_map.insert(i.name.clone(), vec![i.clone()])
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
            unit: g.clone()
        };
        grocery_list.push(value.iter().fold(empty.clone(), |a, x| a.combine(x)));
    }

    // todo: create a unit test
    for item in grocery_list {
        println!("{:?}", item);
    }
}
