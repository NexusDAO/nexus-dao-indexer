CREATE TABLE vote (
  voter TEXT PRIMARY KEY,
  proposal_id BIGINT NOT NULL,
  token_id BIGINT NOT NULL,
  is_agreed BOOLEAN NOT NULL
);
