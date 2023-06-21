CREATE TABLE proposal (
  id TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  proposal_type TEXT NOT NULL,
  summary TEXT NOT NULL,
  body TEXT NOT NULL,
  proposer TEXT NOT NULL,
  stake TEXT NOT NULL,
  dao TEXT NOT NULL,
  created BIGINT NOT NULL,
  duration BIGINT NOT NULL
);