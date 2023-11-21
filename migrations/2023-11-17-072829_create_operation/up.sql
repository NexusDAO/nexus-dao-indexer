CREATE TABLE operation (
    id SERIAL PRIMARY KEY,
    type TEXT NOT NULL,
    program_name TEXT NOT NULL,
    mapping_id TEXT NOT NULL,
    key_id TEXT,
    value_id TEXT,
    mapping_name TEXT NOT NULL,
    key TEXT,
    value TEXT
);