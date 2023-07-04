CREATE TABLE token_infos (
  id BIGINT PRIMARY KEY,
  name TEXT NOT NULL,
  symbol TEXT NOT NULL,
  supply BIGINT NOT NULL,
  decimals BIGINT NOT NULL,
  max_mint_amount BIGINT NOT NULL,
  minted_amount BIGINT NOT NULL,
  dao_id BIGINT NOT NULL,
  only_creator_can_mint BOOLEAN NOT NULL
);