table! {
    ingredients (id) {
        id -> Int4,
        recipe_id -> Int4,
        ingredient_name -> Varchar,
        amount -> Float4,
        unit -> Varchar,
        notes -> Nullable<Varchar>,
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
