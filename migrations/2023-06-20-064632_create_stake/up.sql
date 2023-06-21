CREATE TABLE stake (
  id TEXT PRIMARY KEY,
  owner TEXT NOT NULL,
  amount TEXT NOT NULL,
  token TEXT NOT NULL,
  created BIGINT NOT NULL,
  duration BIGINT NOT NULL
);