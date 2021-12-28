// todo: ability to convert between weight, volume, and count for a given ingredient?

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

// define units for retrieval by name
pub fn get_unit(s: &str) -> Unit {
    // todo: make s lower case
    match s {
        // base units
        "mg" => base_unit(&UnitType::Weight),
        "ml" => base_unit(&UnitType::Volume),
        "" => base_unit(&UnitType::Count),

        // weights
        "g" | "gram" | "grams" => Unit {
            name: "g".to_string(),
            relative_to_base: 1000.0,
            measuring: UnitType::Weight
        },
        "kg" | "kilogram" | "kilograms" => Unit {
            name: "kg".to_string(),
            relative_to_base: 1000.0 * 1000.0,
            measuring: UnitType::Weight
        },
        "lb" | "lbs" | "pound" | "pounds" => Unit {
            name: "lbs".to_string(),
            relative_to_base: 453592.4,
            measuring: UnitType::Weight
        },
        "oz" | "ounce" | "ounces" => Unit {
            name: "oz".to_string(),
            relative_to_base: 28349.52,
            measuring: UnitType::Weight
        },

        // volumes
        "l" | "liter" | "liters" => Unit {
            name: "l".to_string(),
            relative_to_base: 1000.0,
            measuring: UnitType::Volume
        },
        "gal" | "gallon" | "gallons" => Unit {
            name: "gallons".to_string(),
            relative_to_base: 4546.09,
            measuring: UnitType::Volume
        },
        "fluid ounce" | "fl oz" => Unit {
            name: "imperial fluid ounce".to_string(),
            relative_to_base: 28.41306,
            measuring: UnitType::Volume
        },
        // this is imperial, todo: account for Australian measurement as well
        "cup" | "cups" => Unit {
            name: "cups".to_string(),
            relative_to_base: 284.1306,
            measuring: UnitType::Volume
        },
        "teaspoon" | "tsp" => Unit {
            name: "tsp".to_string(),
            relative_to_base: 0.1689364,
            measuring: UnitType::Volume
        },
        "tablespoon" | "tbsp" => Unit {
            name: "tbsp".to_string(),
            relative_to_base: 0.05631213,
            measuring: UnitType::Volume
        },

        _ => panic!("Unsupported unit {}", s)
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_unit() {
        let pound = get_unit("lbs");
        //assert!(true); 
        assert_eq!(pound.relative_to_base, 453592.4);
        //assert_ne!(1, 2);
    }
}