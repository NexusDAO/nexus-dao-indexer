CREATE TABLE record (
  transition_id TEXT PRIMARY KEY,
  program TEXT NOT NULL,
  function TEXT NOT NULL,
  Inputs TEXT NOT NULL,
  Outputs TEXT NOT NULL,
  block_hash TEXT NOT NULL,
  previous_hash TEXT NOT NULL,
  transaction_id TEXT NOT NULL,
  network BIGINT NOT NULL,
  height BIGINT NOT NULL,
  timestamp BIGINT NOT NULL
);

CREATE INDEX idx_height ON record (height);