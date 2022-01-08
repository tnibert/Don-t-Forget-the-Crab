#[macro_use]
extern crate diesel;
extern crate dotenv;

mod units;
mod recipe;
mod ingredient;

mod conversion;
use conversion::*;

mod dbaccess;
use dbaccess::*;

// todo: remove unnecessary clone() calls throughout program and use the borrow checker correctly
// todo: encapsulate in modules, remove unnecessary pubs

fn main() {
    // edit this array for the meal plan
    let thanksgiving_array = ["Green Bean Casserole", "Cranberry Delight Salad", "Sweet Potato Casserole", "Corn Souffle Casserole", "Broccoli Casserole"];

    let mut recipes = Vec::new();

    for recipe_name in thanksgiving_array.iter() {
        let retrieved_recipe = get_recipe(recipe_name);
        match retrieved_recipe {
            Some(r) => {
                println!("Retrieved recipe {}", r.name);
                recipes.push(r);
            },
            None => println!("No recipe with that name exists in database")
        }
    }

    // convert meal plan recipes to grocery list
    let grocery_list = recipes_to_grocery_list(recipes);

    for item in grocery_list {
        println!("{} {} {}", item.amount, item.unit.name, item.name);
    }
}

// integration tests
#[cfg(test)]
mod tests {
    use super::*;
    use units::*;
    use recipe::*;
    use ingredient::*;

    #[test]
    fn test_combination() {
        // create units
        //let mg = base_unit(&UnitType::Weight);
    
        let g = get_unit("g");
        let kg = get_unit("kg");
        let num = get_unit("");
    
        // create recipes
        // todo: store and retrieve recipes from database
        let mut eggsandtoast = Recipe::new("eggs and toast");
        eggsandtoast.push(Ingredient {
            name: "egg".to_string(),
            amount: 2.0,
            unit: num.clone()
        });
        eggsandtoast.push(Ingredient {
            name: "bread".to_string(),
            amount: 2.0,
            unit: num.clone()
        });
        eggsandtoast.push(Ingredient {
            name: "cheese".to_string(),
            // holy shit that's some cheesy toast
            amount: 2.0,
            unit: kg.clone()
        });
    
        let mut eggmuffin = Recipe::new("egg muffin");
        eggmuffin.push(Ingredient {
            name: "egg".to_string(),
            amount: 1.0,
            unit: num.clone()
        });
        eggmuffin.push(Ingredient {
            name: "english muffin".to_string(),
            amount: 1.0,
            unit: num.clone()
        });
        eggmuffin.push(Ingredient {
            name: "cheese".to_string(),
            amount: 5.0,
            unit: g.clone()
        });
    
        // all recipes to add to shopping list
        let recipes = vec![eggsandtoast, eggmuffin];
        let grocery_list = recipes_to_grocery_list(recipes);
    
        /*
        grocery list should be as follows:
        Ingredient { name: "bread", amount: 2.0, unit: Unit { name: "", relative_to_base: 1.0, measuring: Count } }
        Ingredient { name: "cheese", amount: 2005000.0, unit: Unit { name: "mg", relative_to_base: 1.0, measuring: Weight } }
        Ingredient { name: "egg", amount: 3.0, unit: Unit { name: "", relative_to_base: 1.0, measuring: Count } }
        Ingredient { name: "english muffin", amount: 1.0, unit: Unit { name: "", relative_to_base: 1.0, measuring: Count } }
        */
        assert_eq!(grocery_list.len(), 4);

        let index = grocery_list.iter().position(|x| x.name == "bread").unwrap();
        assert_eq!(grocery_list[index].amount, 2.0);
        assert_eq!(grocery_list[index].unit.name, "");

        let index = grocery_list.iter().position(|x| x.name == "cheese").unwrap();
        assert_eq!(grocery_list[index].amount, 2005000.0);
        assert_eq!(grocery_list[index].unit.name, "mg");

        let index = grocery_list.iter().position(|x| x.name == "egg").unwrap();
        assert_eq!(grocery_list[index].amount, 3.0);
        assert_eq!(grocery_list[index].unit.name, "");

        let index = grocery_list.iter().position(|x| x.name == "english muffin").unwrap();
        assert_eq!(grocery_list[index].amount, 1.0);
        assert_eq!(grocery_list[index].unit.name, "");
    }
}