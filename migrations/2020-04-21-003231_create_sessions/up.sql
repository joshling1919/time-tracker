CREATE TABLE sessions (
  id SERIAL PRIMARY KEY,
  details TEXT NOT NULL,
  started_at TIMESTAMP,
  finished_at TIMESTAMP
)
