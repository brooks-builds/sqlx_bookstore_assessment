-- Add migration script here
CREATE TABLE IF NOT EXISTS book_authors (
  author_id  INT  NOT NULL REFERENCES authors(author_id) ON DELETE CASCADE,
  book_id    INT  NOT NULL REFERENCES books(book_id) ON DELETE CASCADE,
  PRIMARY KEY (author_id, book_id)
)
