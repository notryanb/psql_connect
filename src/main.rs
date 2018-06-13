extern crate clap;

use clap::{Arg, App, SubCommand};

mod pg_pass;

use std::process::Command;
use self::pg_pass::*;

fn main() -> std::io::Result<()> {
    let result = parse_pg_pass()?;

    let matches = App::new("Psql Connect")
        .version("0.1")
        .author("Ryan Blecher <notryanb@gmail.com")
        .about("Easily connect to a postgres database configured via a `.pg_pass` file")
        .arg(Arg::with_name("list")
             .short("l")
             .long("list")
             .help("Lists all database connections by number and alias"))
        .arg(Arg::with_name("connect")
             .short("c")
             .long("connect")
             .value_name("alias")
             .help("Connects to a alias provided in `.pg_pass` file")
             .takes_value(true))
        .get_matches();

    if matches.is_present("list") {
        let aliases = result.list_aliases();
        aliases.iter()
            .enumerate()
            .for_each(|(idx, alias)| println!("{}: {}", idx + 1, alias.unwrap()));
    }

    if let Some(alias) = matches.value_of("connect") {
        let selected = result.select_config(alias).unwrap();
        println!("Connecting to: {}", alias);
        connect_to_config(&selected);
    }


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

