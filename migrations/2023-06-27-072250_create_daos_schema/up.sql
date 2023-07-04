CREATE TABLE daos_schema (
  name TEXT PRIMARY KEY,
  dao_type BIGINT NOT NULL,
  creater TEXT NOT NULL,
  icon TEXT NOT NULL,
  description TEXT NOT NULL,
  official_link TEXT NOT NULL
);