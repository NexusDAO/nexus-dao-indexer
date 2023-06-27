// @generated automatically by Diesel CLI.

diesel::table! {
    dao (id) {
        id -> Int8,
        name -> Text,
        dao_type -> Int8,
        creater -> Text,
        token_info_id -> Int8,
        icon -> Text,
        description -> Text,
        official_link -> Text,
        proposal_count -> Int8,
        pass_proposal_count -> Int8,
        vote_count -> Int8,
        passed_votes_proportion -> Int8,
        passed_tokens_proportion -> Int8,
    }
}

diesel::table! {
    daos_schema (name) {
        name -> Text,
        dao_type -> Int8,
        creater -> Text,
        icon -> Text,
        description -> Text,
        official_link -> Text,
    }
}

diesel::table! {
    hold_token (address) {
        address -> Text,
        amount -> Int8,
        token_info_id -> Int8,
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
        id -> Int8,
        title -> Text,
        proposer -> Text,
        summary -> Text,
        body -> Text,
        dao_id -> Int8,
        created -> Int8,
        duration -> Int8,
        proposer_type -> Int8,
        adopt -> Int8,
        reject -> Int8,
        status -> Int8,
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
    token (owner) {
        owner -> Text,
        gates -> Int8,
        token_info_id -> Int8,
        amount -> Int8,
        expires -> Int8,
        staked_at -> Int8,
    }
}

diesel::table! {
    token_info (id) {
        id -> Int8,
        name -> Text,
        symbol -> Text,
        supply -> Int8,
        decimals -> Int8,
        max_mint_amount -> Int8,
        minted_amount -> Int8,
        dao_id -> Int8,
        only_creator_can_mint -> Bool,
    }
}

diesel::table! {
    token_info_schema (name) {
        name -> Text,
        symbol -> Text,
        supply -> Int8,
        decimals -> Int8,
        max_mint_amount -> Int8,
    }
}

diesel::table! {
    vote (voter) {
        voter -> Text,
        proposal_id -> Int8,
        token_id -> Int8,
        is_agreed -> Bool,
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
    daos_schema,
    hold_token,
    profile,
    proposal,
    record,
    stake,
    token,
    token_info,
    token_info_schema,
    vote,
    voting_results,
);
