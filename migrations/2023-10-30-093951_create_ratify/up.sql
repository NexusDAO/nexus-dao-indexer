CREATE TABLE ratify (
  ratification_id TEXT PRIMARY KEY,
  height BIGINT NOT NULL,
  type TEXT NOT NULL,
  starting_round TEXT,
  total_stake TEXT,
  block_reward TEXT,
  puzzle_reward TEXT
);