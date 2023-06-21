// @generated automatically by Diesel CLI.

diesel::table! {
    dao (id) {
        id -> Text,
        creater -> Text,
        info -> Text,
        token -> Text,
    }
}

diesel::table! {
    dao_info (name) {
        name -> Text,
        dao_type -> Text,
        icon -> Text,
        description -> Text,
        official_link -> Text,
    }
}

diesel::table! {
    dao_table (organization_name) {
        organization_name -> Text,
        fund_rank -> Int8,
        total_funds -> Text,
        token_count -> Text,
        token_price -> Text,
        token_name -> Text,
        token_holder_count -> Int8,
        token_staker_count -> Int8,
        proposal_count -> Int8,
        vote_count -> Int8,
        proposal_pass_rate -> Int8,
    }
}

diesel::table! {
    profile (address) {
        address -> Text,
        name -> Text,
        avatar -> Text,
        bio -> Text,
    }
}

diesel::table! {
    proposal (id) {
        id -> Text,
        title -> Text,
        proposal_type -> Text,
        summary -> Text,
        body -> Text,
        proposer -> Text,
        stake -> Text,
        dao -> Text,
        created -> Int8,
        duration -> Int8,
    }
}

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

diesel::table! {
    stake (id) {
        id -> Text,
        owner -> Text,
        amount -> Text,
        token -> Text,
        created -> Int8,
        duration -> Int8,
    }
}

diesel::table! {
    token (id) {
        id -> Text,
        owner -> Text,
        amount -> Text,
        token_info -> Text,
    }
}

diesel::table! {
    token_info (name) {
        name -> Text,
        symbol -> Text,
        supply -> Text,
        decimals -> Text,
        contract -> Text,
    }
}

diesel::table! {
    vote (proposal_id) {
        proposal_id -> Text,
        choice -> Bool,
    }
}

diesel::table! {
    voting_results (proposal_id) {
        proposal_id -> Text,
        adopt -> Int8,
        reject -> Int8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    dao,
    dao_info,
    dao_table,
    profile,
    proposal,
    record,
    stake,
    token,
    token_info,
    vote,
    voting_results,
);
