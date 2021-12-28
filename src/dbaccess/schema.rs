table! {
    ingredients (id) {
        id -> Int4,
        recipe_id -> Int4,
        ingredient_name -> Varchar,
        // todo: confirm whether or not this should be Float8
        amount -> Float,
        unit -> Varchar,
    }
}

table! {
    recipes (id) {
        id -> Int4,
        recipe_name -> Varchar,
        notes -> Nullable<Varchar>,
    }
}

joinable!(ingredients -> recipes (recipe_id));

allow_tables_to_appear_in_same_query!(
    ingredients,
    recipes,
);
