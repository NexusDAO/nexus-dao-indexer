CREATE TABLE dao_table (
  organization_name TEXT PRIMARY KEY,
  fund_rank BIGINT NOT NULL,
  total_funds TEXT NOT NULL,
  token_count TEXT NOT NULL,
  token_price TEXT NOT NULL,
  token_name TEXT NOT NULL,
  token_holder_count BIGINT NOT NULL,
  token_staker_count BIGINT NOT NULL,
  proposal_count  BIGINT NOT NULL,
  vote_count  BIGINT NOT NULL,
  proposal_pass_rate  BIGINT NOT NULL
);