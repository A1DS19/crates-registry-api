-- Your SQL goes here
CREATE TABLE rustacean (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  created_at TIMESTAMP DEFAULT NOW() NOT NULL
)
