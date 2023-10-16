-- Add migration script here
CREATE TABLE IF NOT EXISTS books (
    book_id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL
);