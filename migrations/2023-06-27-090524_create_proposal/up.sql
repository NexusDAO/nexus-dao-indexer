CREATE TABLE proposal (
  id BIGINT PRIMARY KEY,
  title TEXT NOT NULL,
  proposer TEXT NOT NULL,
  summary TEXT NOT NULL,
  body TEXT NOT NULL,
  dao_id BIGINT  NOT NULL,
  created BIGINT NOT NULL,
  duration BIGINT NOT NULL,
  proposer_type BIGINT NOT NULL,
  adopt BIGINT  NOT NULL,
  reject BIGINT NOT NULL,
  status BIGINT NOT NULL  
);