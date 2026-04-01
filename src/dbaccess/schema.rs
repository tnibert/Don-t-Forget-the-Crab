// @generated automatically by Diesel CLI.

diesel::table! {
    ingredients (id) {
        id -> Integer,
        recipe_id -> Integer,
        ingredient_name -> Text,
        amount -> Float,
        unit -> Text,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    recipes (id) {
        id -> Integer,
        recipe_name -> Text,
        notes -> Nullable<Text>,
    }
}

diesel::joinable!(ingredients -> recipes (recipe_id));

diesel::allow_tables_to_appear_in_same_query!(ingredients, recipes,);
