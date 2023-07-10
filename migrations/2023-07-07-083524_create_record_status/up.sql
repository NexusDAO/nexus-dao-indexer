CREATE TABLE record_status (
  record_ciphertext TEXT PRIMARY KEY,
  program TEXT NOT NULL,
  function TEXT NOT NULL,
  is_spent BOOL NOT NULL,
  block_hash TEXT NOT NULL,
  transaction_id TEXT NOT NULL,
  transition_id TEXT NOT NULL,
  height BIGINT NOT NULL,
  network BIGINT NOT NULL,
  timestamp BIGINT NOT NULL
);