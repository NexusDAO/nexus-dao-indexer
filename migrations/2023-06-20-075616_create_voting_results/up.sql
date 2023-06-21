CREATE TABLE voting_results (
  proposal_id TEXT PRIMARY KEY,
  adopt BIGINT NOT NULL,
  reject BIGINT NOT NULL
);