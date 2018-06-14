psql_connect
---
Using `psql` to log into remote Postgres databases is tedious and I don't like typing out the shell command.
Users can set up `.pgpass' file as suggested in the [Postgres docs],
however the connection strings can be very long and hard to enter.

This tool will list every connection string a user has configured in the `.pgpass` file 
and let you select which instance to connect to.

`psql_connect` currently relies on adding an alias to the `.pgpass` format.

> hostname:port:database:username:password:alias

If no alias is present for the connection, `psql_connect` will display the hostname of that database.
There is presently no way to connect to databases that don't have an alias.
The roadmap includes adding in features to connect by number (as output by `psqlconnect --list`)
and creating aliases via the cli tool.

[Postgres docs]: https://www.postgresql.org/docs/9.6/static/libpq-pgpass.html
