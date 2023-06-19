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
        network -> Text,
        height -> Text,
        timestamp -> Text,
    }
}
