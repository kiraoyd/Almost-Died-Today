-- Add migration script here
DELETE FROM users; --start fresh

--reset primary key to 1
SELECT setval(pg_get_serial_sequence('users', 'id'), 1, false);

INSERT INTO users(email, password)
VALUES('email@email.com', 'password');

--password hash is for "banana"
INSERT INTO users(email, password)
VALUES('me@gmail.com', '$argon2i$v=19$m=4096,t=3,p=1$QVNLU09NRU9ORUZPUlRIRVNBTFQ$alJD+Rx+5Lx82XZX9AuPFK5Vf4HWhX3DPhsJ7aNfEK0')