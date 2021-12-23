use crate::units::*;

/*
Really, any ingredient has both a weight and volume
perhaps this should be represented
*/
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
