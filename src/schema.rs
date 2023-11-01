// @generated automatically by Diesel CLI.

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
