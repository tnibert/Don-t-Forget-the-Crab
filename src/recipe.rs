use crate::ingredient::*;

pub struct Recipe {
    pub name: String,
    ingredients: Vec<Ingredient>
}

impl Recipe {
    // todo: add a second constructor to initialize with a vector
    pub fn new(name: &str) -> Recipe {
        Self {
            name: name.to_string(),
            ingredients: Vec::<Ingredient>::new()
        }
    }

    pub fn push(&mut self, ingredient: Ingredient) {
        self.ingredients.push(ingredient);
    }

    pub fn iter(&self) -> RecipeIterator {
        RecipeIterator {progress: 0,
                        ingredients: self.ingredients.clone()}
    }
}

pub struct RecipeIterator {
    progress: usize,
    ingredients: Vec<Ingredient>
}

impl Iterator for RecipeIterator {
    type Item = Ingredient;

    fn next(&mut self) -> Option<Self::Item> {
        if self.progress < self.ingredients.len() {
            let ret = Some(self.ingredients[self.progress].clone());
            self.progress += 1;
            ret
        } else {
            // reset
            self.progress = 0;
            None
        }
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::get_unit;

    #[test]
    fn test_push() {
        let mut r = Recipe::new("myrecipe");
        r.push(Ingredient {
            name: "cheese".to_string(),
            amount: 5.0,
            unit: get_unit("g")
        });
        assert_eq!(r.ingredients[0].name, "cheese");
        assert_eq!(r.ingredients.len(), 1);
    }

    #[test]
    fn test_iterate() {
        let mut r = Recipe::new("myrecipe");
        r.push(Ingredient {
            name: "a".to_string(),
            amount: 5.0,
            unit: get_unit("g")
        });
        r.push(Ingredient {
            name: "b".to_string(),
            amount: 5.0,
            unit: get_unit("g")
        });
        r.push(Ingredient {
            name: "c".to_string(),
            amount: 5.0,
            unit: get_unit("g")
        });
        let result: Vec<Ingredient> = r.iter().collect();

        assert_eq!(result[0].name, "a");
        assert_eq!(result[1].name, "b");
        assert_eq!(result[2].name, "c");
        assert_eq!(result.len(), 3);
    }
}