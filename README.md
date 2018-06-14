psql_connect
---
Using `psql` to log into remote Postgres databases is tedious and I don't like typing out the shell command.
Users can set up `.pgpass' file as suggested in the [Postgres docs],
however the connection strings can be very long and hard to enter.

This tool will list every connection string a user has configured in the `.pgpass` file 
and let you select which instance to connect to.

[Postgres docs]: https://www.postgresql.org/docs/9.6/static/libpq-pgpass.html
