mod conversion;
use conversion::*;

// todo: remove unnecessary clone() calls throughout program and use the borrow checker correctly
// todo: load recipes from a database

fn main() {
    // create units
    //let mg = base_unit(&UnitType::Weight);

    let g = get_unit("g");
    let kg = get_unit("kg");
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
    let recipes = vec![eggsandtoast, eggmuffin];
    let grocery_list = recipes_to_grocery_list(recipes);

    // todo: create a unit test
    for item in grocery_list {
        println!("{:?}", item);
    }
}
