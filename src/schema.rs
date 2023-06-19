// @generated automatically by Diesel CLI.

diesel::table! {
    record (transition_id) {
        transition_id -> Text,
        program -> Text,
        function -> Text,
        inputs -> Text,
        outputs -> Text,
        block_hash -> Text,
        previous_hash -> Text,
        transaction_id -> Text,
        network -> Int8,
        height -> Int8,
        timestamp -> Int8,
    }
}
