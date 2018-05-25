extern crate clap;

// use clap::{Arg, App, SubCommand};

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
pub struct PgConfigList {
    configs: Vec<PgConfig>,
}

impl PgConfigList {
    pub fn add(&mut self, config: PgConfig) {
        self.configs.push(config); 
    }

    // pub fn import_types(&self) -> Option<&[String]> {
    //     self.import_types.as_ref().map(|v| &**v)
    // }

    pub fn list_aliases(&self) -> Vec<Option<&String>> {
        self
            .configs
            .iter()
            .map(|cfg| cfg.alias.as_ref())
            .collect()
    }
}

#[derive(Debug)]
pub struct PgConfig {
    alias: Option<String>,
    hostname: String,
    port: u16,
    dbname: String,
    username: String,
    password: String,
}

impl PgConfigList {
    pub fn new() -> PgConfigList {
        PgConfigList { 
            configs: Vec::new(),
        }
    }
}


fn main() -> std::io::Result<()> {
    println!("Yo!");
    let result = parse_pg_pass()?;
    // let config = &result.configs[0];

    let aliases = result.list_aliases();
    println!("Aliases: {:?}", aliases);

    // connect_to_config(&config);

    println!("Exiting psql_connect");

    Ok(())
}

pub fn parse_pg_pass() -> Result<PgConfigList, Error> {
    let home = env::home_dir().unwrap();
    let file_path = Path::new(".pgpass");
    let pg_pass_path = home.join(file_path);
    let pg_pass_file = File::open(pg_pass_path)?;
    let reader = BufReader::new(pg_pass_file);
    let mut config_list = PgConfigList::new();

    for line in reader.lines() {
        let mut params = line.as_ref().unwrap().split(':');
        let hostname = params.next().unwrap().into();
        let port = params.next().unwrap().parse::<u16>().unwrap();
        let dbname = params.next().unwrap().into();
        let username = params.next().unwrap().into();
        let password = params.next().unwrap().into();
        let alias = Some(params.next().unwrap().into());

        let config = PgConfig {
            alias: alias,
            hostname: hostname,
            port: port,
            username: username,
            password: password,
            dbname: dbname,
        };

        config_list.add(config);
    }

    Ok(config_list)
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
