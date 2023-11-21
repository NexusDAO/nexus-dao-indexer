CREATE TABLE mapping (
    key_id TEXT PRIMARY KEY,
    value_id TEXT NOT NULL,
    mapping_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    mapping_name TEXT NOT NULL,
    program_name TEXT NOT NULL,
    removed BOOLEAN NOT NULL
);