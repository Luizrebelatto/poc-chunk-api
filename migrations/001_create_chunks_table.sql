CREATE TABLE IF NOT EXISTS chunks (
    id SERIAL PRIMARY KEY,
    filename VARCHAR(255) NOT NULL UNIQUE,
    file_path VARCHAR(500) NOT NULL,
    size BIGINT NOT NULL,
    content_type VARCHAR(100),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_chunks_filename ON chunks(filename);

CREATE INDEX IF NOT EXISTS idx_chunks_created_at ON chunks(created_at DESC);

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_chunks_updated_at BEFORE UPDATE ON chunks
FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
