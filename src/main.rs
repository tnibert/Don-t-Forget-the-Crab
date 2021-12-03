use std::collections::HashMap;

// todo: struct Recipe

struct Ingredient {
    name: String,
    amount: f32,
    unit: Unit
}

impl Ingredient {
    fn combine (&self, other: &Ingredient) -> Result<Ingredient, &'static str> {
        if (self.name == other.name) {
            return Ok(Ingredient {
                name: self.name.clone(),
                // todo: normalize the amounts across units
                amount: self.amount + other.amount,
                unit: self.unit.clone()
            })

        } else {
            // can't combine
            return Err("these things are not the same");
        }
    }
}

#[derive(Clone)]
enum Unit {
    weight(String, u32),
    volume(String, u32),
    count(String, u32)
}

fn main() {
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

    let mg = Unit::weight(String::from("mg"), 1);
    let g = Unit::weight(String::from("g"), 1000);
    let kg = Unit::weight(String::from("kg"), 1000 * 1000);

    //let shopping_list_item = sugar1.combine(sugar2);
    //println!("{}", shopping_list_item.name);
    //println!("{}", shopping_list_item.amount);

    // hashmap and fold()
    // each key is the name of the ingredient
    // each value is a vector of the ingredients
    // then fold() to reduce each to a single value
    // don't worry about units yet, just use grams

    let mut ingredient_map = HashMap::new();

    ingredient_map.insert("sugar".to_string(), Vec::<Ingredient>::new());

    let sugar = ingredient_map.get_mut("sugar").unwrap();

    sugar.push(Ingredient {
        name: "sugar".to_string(),
        amount: 3.0,
        unit: g.clone()
    });
    
    sugar.push(Ingredient {
        name: "sugar".to_string(),
        amount: 4.0,
        unit: g.clone()
    });

    let empty = Ok(Ingredient {
        name: "sugar".to_string(), 
        amount: 0.0,
        unit: g.clone()
    });
    let total = sugar.iter().fold(empty, |a, x| a.unwrap().combine(x));
    println!("{}", total.unwrap().amount);
}
