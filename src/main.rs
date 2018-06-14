#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate clap;

mod pg_pass;
mod errors;

use clap::{App, Arg};
use errors::*;

use std::process::Command;
use self::pg_pass::*;

fn main() {
    if let Err(ref e) = run() {
        println!("psql_connect: Error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}",
                     backtrace);
        }

        ::std::process::exit(1);
    }
}


fn run() -> Result<()> {
    let config_list = match parse_pg_pass() {
        Ok(configs) => configs,
        Err(e) => return Err(e)
    };

    let matches = App::new("psql_connect")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Ryan Blecher <notryanb@gmail.com")
        .about("Easily connect to a postgres database configured via a `.pg_pass` file")
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .help("Lists all database connections by number and alias"),
        )
        .arg(
            Arg::with_name("connect")
                .short("c")
                .long("connect")
                .value_name("alias")
                .help("Connects to a alias provided in `.pg_pass` file")
                .takes_value(true),
        )
        .get_matches();

    if matches.is_present("list") {
        let aliases = config_list.list_aliases();
        aliases
            .iter()
            .enumerate()
            .for_each(|(idx, alias)| println!("{}: {}", idx + 1, alias));
    }

    if let Some(alias) = matches.value_of("connect") {
        let selected = config_list.select_config(alias)?;
        connect_to_config(&selected);
    }

    Ok(())
}



pub fn connect_to_config(config: &PgConfig) {
    let psql = Command::new("psql")
        .arg("-w")
        .arg(format!("--host={}", config.hostname))
        .arg(format!("--port={}", config.port))
        .arg(format!("--username={}", config.username))
        .arg(format!("--dbname={}", config.dbname))
        .spawn()
        .expect("failed to execute process");

    let output = psql.wait_with_output().expect("Couldn't wait on `psql`");
    output.stdout;
}
