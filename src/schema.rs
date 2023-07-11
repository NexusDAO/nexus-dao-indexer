// @generated automatically by Diesel CLI.

diesel::table! {
    auto_increment (key) {
        key -> Int8,
        value -> Int8,
    }
}

diesel::table! {
    balances (key) {
        key -> Text,
        owner -> Text,
        amount -> Int8,
        token_info_id -> Int8,
    }
}

diesel::table! {
    daos (id) {
        id -> Int8,
        name -> Text,
        dao_type -> Int8,
        creator -> Text,
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
        creator -> Text,
        icon -> Text,
        description -> Text,
        official_link -> Text,
    }
}

diesel::table! {
    extend_pledge_period (key) {
        key -> Int8,
        value -> Int8,
    }
}

diesel::table! {
    profiles (address) {
        address -> Text,
        name -> Text,
        avatar -> Text,
        bio -> Text,
    }
}

diesel::table! {
    proposals (id) {
        id -> Int8,
        title -> Text,
        proposer -> Text,
        summary -> Text,
        body -> Text,
        dao_id -> Int8,
        created -> Int8,
        duration -> Int8,
        #[sql_name = "type"]
        type_ -> Int8,
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
    stake_amounts (key) {
        key -> Text,
        owner -> Text,
        amount -> Int8,
        token_info_id -> Int8,
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
    token_infos (id) {
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
    votes (key) {
        key -> Text,
        voter -> Text,
        proposal_id -> Int8,
        is_agreed -> Bool,
        time -> Int8,
        amount -> Int8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    auto_increment,
    balances,
    daos,
    daos_schema,
    extend_pledge_period,
    profiles,
    proposals,
    record,
    stake_amounts,
    token,
    token_infos,
    votes,
);
