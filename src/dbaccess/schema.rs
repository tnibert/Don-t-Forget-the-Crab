table! {
    ingredients (id) {
        id -> Int4,
        recipe_id -> Int4,
        ingredient_name -> Varchar,
        amount -> Float,
        unit -> Varchar,
    }
}

table! {
    recipes (id) {
        id -> Int4,
        recipe_name -> Varchar,
    }
}

joinable!(ingredients -> recipes (recipe_id));

allow_tables_to_appear_in_same_query!(
    ingredients,
    recipes,
);
