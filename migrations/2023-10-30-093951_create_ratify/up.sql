CREATE TABLE ratify (
  id SERIAL PRIMARY KEY,
  ratification_id TEXT NOT NULL,
  height BIGINT NOT NULL,
  type TEXT NOT NULL,
  starting_round TEXT,
  total_stake TEXT,
  block_reward TEXT,
  puzzle_reward TEXT
);