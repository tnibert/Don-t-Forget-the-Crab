#[macro_use]
extern crate diesel;
extern crate dotenv;

mod conversion;
use conversion::*;
mod units;
use units::*;
mod recipe;
use recipe::*;
mod ingredient;
use ingredient::*;
mod dbaccess;
use dbaccess::*;

// todo: remove unnecessary clone() calls throughout program and use the borrow checker correctly
// todo: encapsulate in modules, remove unnecessary pubs

fn main() {
    // create units
    //let mg = base_unit(&UnitType::Weight);

    let g = get_unit("g");
    let kg = get_unit("kg");
    let num = get_unit("");

    // create recipes
    // todo: store and retrieve recipes from database
    let mut eggsandtoast = Recipe::new("eggs and toast");
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

    let mut eggmuffin = Recipe::new("egg muffin");
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
    let recipes = vec![eggsandtoast, eggmuffin];
    let grocery_list = recipes_to_grocery_list(recipes);

    /*
    todo: create integration test, assert grocery list is as follows
    Ingredient { name: "bread", amount: 2.0, unit: Unit { name: "", relative_to_base: 1.0, measuring: Count } }
    Ingredient { name: "cheese", amount: 2005000.0, unit: Unit { name: "mg", relative_to_base: 1.0, measuring: Weight } }
    Ingredient { name: "egg", amount: 3.0, unit: Unit { name: "", relative_to_base: 1.0, measuring: Count } }
    Ingredient { name: "english muffin", amount: 1.0, unit: Unit { name: "", relative_to_base: 1.0, measuring: Count } }
    */
    for item in grocery_list {
        println!("{:?}", item);
    }

    get_recipe("test");
}
