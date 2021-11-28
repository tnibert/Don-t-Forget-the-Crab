use std::collections::HashMap;
use std::rc::Rc;

struct Ingredient {
    name: String,
    amount: f32,
    unit: Unit
}

impl Ingredient {
    fn combine (&self, other: Ingredient) {
        if (self.name == other.name) {
            // combine

        } else {
            // can't combine
        }
    }
}

struct Unit {
    unit_type: MeasurementType,
    // todo: can have multiple identifiers - e.g. mg and milligrams
    identifier: String,
    // if None, we are the base unit
    // would be better to self reference?
    base_unit: Option<Rc<Unit>>, //Option<Box<Unit>>,
    ratio_to_base: u32
}

// how do we enforce that all Units of a given type have the same base_unit? - constructor
// do we even need to?
enum MeasurementType {
    weight,
    volume,
    count
}

fn main() {
    let milligram = Unit {
        unit_type: MeasurementType::weight,
        identifier: String::from("mg"),
        base_unit: Option::None,
        ratio_to_base: 1
    };

    let mg_ref = Rc::new(milligram);

    let gram = Unit {
        unit_type: MeasurementType::weight,
        identifier: String::from("g"),
        base_unit: Option::Some(Rc::clone(&mg_ref)),
        ratio_to_base: 1000
    };
    
    let kilogram = Unit {
        unit_type: MeasurementType::weight,
        identifier: String::from("kg"),
        base_unit: Option::Some(Rc::clone(&mg_ref)),
        ratio_to_base: 1000 * 1000
    };

    println!("count for mg_ref = {}", Rc::strong_count(&mg_ref));
}
