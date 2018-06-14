# Change Log

All user visible changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/), as described
for Rust libraries in [RFC #1105](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)

## Unreleased

* Added error-chain crate and now handle cases: Can't open `,pgpass` file, connecting to non-existant alias, connecting to duplicated alias
* Proof of concept that this tool can be used to read from `.pgpass` on macOS and start up `psql`.
