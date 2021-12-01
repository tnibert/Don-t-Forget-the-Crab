use std::collections::HashMap;
use std::rc::Rc;

// todo: struct Recipe

struct Ingredient {
    name: String,
    amount: f32,
    unit: Unit
}

impl Ingredient {
    fn combine (&self, other: Ingredient) -> Result<Ingredient, &'static str> {
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
    let mg = Unit::weight(String::from("mg"), 1);
    let g = Unit::weight(String::from("g"), 1000);
    let kg = Unit::weight(String::from("kg"), 1000 * 1000);

    let sugar1 = Ingredient {
        name: String::from("sugar"),
        amount: 3.0,
        unit: g.clone()
    };

    let sugar2 = Ingredient {
        name: String::from("sugar"),
        amount: 3.0,
        unit: g.clone()
    };

    let shopping_list_item = sugar1.combine(sugar2);
    //println!("{}", shopping_list_item.name);
    //println!("{}", shopping_list_item.amount);


}
