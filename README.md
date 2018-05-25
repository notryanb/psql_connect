psql_connect
---
Learning to build CLI wrapper in rust.
Using `psql` to log into remote Postgres databases is tedious (I don't like typing out the shell command).
Users can set up `.pgpass file`, however long commands are sometimes needed and amazon RDS connection strings can be long.


This tool will list every connection string a user has configured in the `.pgpass` file 
and let you select which instance to connect to.


