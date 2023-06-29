CREATE TABLE stake_amounts (
  owner TEXT PRIMARY KEY,
  amount BIGINT NOT NULL,
  token_info_id BIGINT NOT NULL
);