psql_connect
---
Learning to build a CLI wrapper in rust.
Using `psql` to log into remote Postgres databases is tedious (I don't like typing out the shell command).
Users can set up `.pgpass file` as suggested in the Postgres docs, 
however the connection strings can be very long and hard to enter.

This tool will list every connection string a user has configured in the `.pgpass` file 
and let you select which instance to connect to.


