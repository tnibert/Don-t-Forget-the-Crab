use crate::ingredient::*;

// todo: load recipes from a database

// todo (maybe): make into an iterator
pub struct Recipe {
    pub ingredients: Vec<Ingredient>
}

impl Recipe {
    // construct with slice of ingredients - effectively variable # args
    pub fn new() -> Recipe {
        Self {
            ingredients: Vec::<Ingredient>::new()
        }
    }
}