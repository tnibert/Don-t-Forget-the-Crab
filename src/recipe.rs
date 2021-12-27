use crate::ingredient::*;

// todo: load recipes from a database

// todo (maybe): make into an iterator
pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<Ingredient>
}

impl Recipe {
    pub fn new(name: &str) -> Recipe {
        Self {
            name: name.to_string(),
            ingredients: Vec::<Ingredient>::new()
        }
    }

    // todo: constructor to initialize with vector
}