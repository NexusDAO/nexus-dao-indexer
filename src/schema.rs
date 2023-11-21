// @generated automatically by Diesel CLI.

diesel::table! {
    mapping (key_id) {
        key_id -> Text,
        value_id -> Text,
        mapping_id -> Text,
        key -> Text,
        value -> Text,
        mapping_name -> Text,
        program_name -> Text,
        removed -> Bool,
    }
}

diesel::table! {
    operation (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Text,
        program_name -> Text,
        mapping_id -> Text,
        key_id -> Nullable<Text>,
        value_id -> Nullable<Text>,
        mapping_name -> Text,
        key -> Nullable<Text>,
        value -> Nullable<Text>,
    }
}

diesel::table! {
    ratify (id) {
        id -> Int4,
        ratification_id -> Text,
        height -> Int8,
        #[sql_name = "type"]
        type_ -> Text,
        starting_round -> Nullable<Text>,
        total_stake -> Nullable<Text>,
        block_reward -> Nullable<Text>,
        puzzle_reward -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(mapping, operation, ratify,);
