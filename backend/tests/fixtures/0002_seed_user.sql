-- Add migration script here
DELETE FROM users; --start fresh

--reset primary key to 1
SELECT setval(pg_get_serial_sequence('users', 'id'), 1, false);

INSERT INTO users(email, password)
VALUES('email@email.com', 'password');