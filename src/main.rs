extern crate clap;

// use clap::{Arg, App, SubCommand};

mod pg_pass;

use std::process::Command;
use self::pg_pass::*;

fn main() -> std::io::Result<()> {
    println!("Yo!");
    let result = parse_pg_pass()?;
    // let config = &result.configs[0];

    let aliases = result.list_aliases();
    let selected = result.select_config("mlr-staging");
    println!("Config: {:?}", selected);

    // connect_to_config(&config);

    println!("Exiting psql_connect");

    Ok(())
}

pub fn connect_to_config(config: &PgConfig) {
    let psql = Command::new("psql")
        .arg("-w")
        .arg(format!("--host={}", config.hostname ))
        .arg(format!("--port={}", config.port ))
        .arg(format!("--username={}", config.username ))
        .arg(format!("--dbname={}", config.dbname ))
        .spawn()
        .expect("failed to execute process");

    let output = psql.wait_with_output().expect("Couldn't wait on `psql`");
    output.stdout;
}

