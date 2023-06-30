CREATE TABLE balances (
  key TEXT PRIMARY KEY,
  owner TEXT NOT NULL,
  amount BIGINT NOT NULL,
  token_info_id BIGINT NOT NULL
);