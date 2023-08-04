# Possible Buggy Compiler Error

Git Branch: PKIDbug

In ```backend/db.rs``` look for the two TODO comments on line 55 and 56.
When trying to use .map on the row.id on lin 55, the compiler throws the error ```i32 is not an iterator```

Run ```cargo check``` from /backend/ to see the error reproduced.

Look at ```backend/src/models/asteroid.rs``` to see how the Rust struct for an asteroid is defined.

Note the two TODO comments on line 26 and 27 indicating why the error is likely being thrown.

And see ```backend/migrations/...add_asteroid_table.sql``` to see the raw SQL designation of the table, note that id is a PKID and name is constrained to NOT NULL
