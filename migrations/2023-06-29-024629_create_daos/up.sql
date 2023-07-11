CREATE TABLE daos (
  id BIGINT PRIMARY KEY,
  name TEXT NOT NULL,
  dao_type BIGINT NOT NULL,
  creator TEXT NOT NULL,
  token_info_id BIGINT NOT NULL,
  icon TEXT NOT NULL,
  description TEXT NOT NULL,
  official_link TEXT NOT NULL,
  proposal_count BIGINT NOT NULL,
  pass_proposal_count BIGINT NOT NULL,
  vote_count BIGINT NOT NULL,
  passed_votes_proportion BIGINT NOT NULL,
  passed_tokens_proportion BIGINT NOT NULL
);