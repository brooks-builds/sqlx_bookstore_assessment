# Seeds for the SQLx Rust Assessment

The tests for this assessment require that the database is seeded with the following data.

Please DO NOT try to create a script to convert the CSV to SQL statements. This course is assessing your ability to use SQLx with SQL and Rust. We recommend you hand-writing the SQL yourself.

## books

```csv
book_id,name
1,Brave New World
2,Moby Dick
3,Omoo
4,Rip Van Winkle
5,The Raven and Other Poems
6,Mastering the Art of Programming: A Comprehensive Guide for Beginners
```

## authors

```csv
author_id,name
1,Aldous Huxley
2,Herman Melville
3,Washington Irving
4,Edgar Allan Poe
5,Alistair Thompson
6,Emily Sinclair
```

## book authors

```csv
author_id,book_id
1,1
2,2
2,3
3,4
4,5
5,6
6,6
```