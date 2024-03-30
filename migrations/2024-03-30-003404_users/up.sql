-- Your SQL goes here
CREATE TABLE users (
  id UUID NOT NULL PRIMARY KEY,
  email VARCHAR(50) NOT NULL UNIQUE,
  hash VARCHAR(150) NOT NULL,
  created_at TIMESTAMP NOT NULL
);