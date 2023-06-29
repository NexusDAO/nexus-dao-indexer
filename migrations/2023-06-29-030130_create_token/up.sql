CREATE TABLE token (
  owner TEXT PRIMARY KEY,
  gates BIGINT NOT NULL,
  token_info_id BIGINT NOT NULL,
  amount BIGINT NOT NULL,
  expires BIGINT NOT NULL,
  staked_at BIGINT NOT NULL
);