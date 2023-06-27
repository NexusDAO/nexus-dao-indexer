CREATE TABLE token_info_schema (
  name TEXT PRIMARY KEY,
  symbol TEXT NOT NULL,
  supply BIGINT NOT NULL,
  decimals BIGINT NOT NULL,
  max_mint_amount BIGINT NOT NULL
);