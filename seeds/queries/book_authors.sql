INSERT INTO book_authors (book_id, author_id) VALUES
    (
        (SELECT book_id FROM books WHERE name = 'Brave New World'),
        (SELECT author_id FROM authors WHERE name = 'Aldous Huxley')
    ),    
    (
        (SELECT book_id FROM books WHERE name = 'Moby Dick'),
        (SELECT author_id FROM authors WHERE name = 'Herman Melville')
    ),
    (
        (SELECT book_id FROM books WHERE name = 'Omoo'),
        (SELECT author_id FROM authors WHERE name = 'Herman Melville')
    ),
    (
        (SELECT book_id FROM books WHERE name = 'Rip Van Winkle'),
        (SELECT author_id FROM authors WHERE name = 'Washington Irving')
    ),
    (
        (SELECT book_id FROM books WHERE name = 'The Raven and Other Poems'),
        (SELECT author_id FROM authors WHERE name = 'Edgar Allan Poe')
    ),
    (
        (SELECT book_id FROM books WHERE name = 'Mastering the Art of Programming: A Comprehensive Guide for Beginners'),
        (SELECT author_id FROM authors WHERE name = 'Alistair Thompson')
    ),
    (
        (SELECT book_id FROM books WHERE name = 'Mastering the Art of Programming: A Comprehensive Guide for Beginners'),
        (SELECT author_id FROM authors WHERE name = 'Emily Sinclair')
    );
    