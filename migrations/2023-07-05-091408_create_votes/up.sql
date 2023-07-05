CREATE TABLE votes (
  key TEXT PRIMARY KEY,
  voter TEXT NOT NULL,
  proposal_id BIGINT NOT NULL,
  is_agreed BOOLEAN NOT NULL,
  time BIGINT NOT NULL,
  amount BIGINT NOT NULL
);
