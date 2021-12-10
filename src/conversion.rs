// todo: rename this file, organize logically

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