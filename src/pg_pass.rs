use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

#[derive(Debug, PartialEq)]
pub struct PgConfigList {
    pub configs: Vec<PgConfig>,
}

impl PgConfigList {
    pub fn new() -> PgConfigList {
        PgConfigList { 
            configs: Vec::new(),
        }
    }

    pub fn add(&mut self, config: PgConfig) {
        self.configs.push(config); 
    }

    pub fn list_aliases(&self) -> Vec<Option<&String>> {
        self
            .configs
            .iter()
            .map(|cfg| cfg.alias.as_ref())
            .collect()
    }

    pub fn select_config(&self, alias: &str) -> Option<&PgConfig> {
        let config: Vec<_> = self
            .configs
            .iter()
            .filter(|cfg| cfg.alias.as_ref().unwrap() == alias)
            .collect();

        match config.len() {
            0 => None,
            1 => Some(config[0]),
            _ => None,
        }

    }
}

#[derive(Debug, PartialEq)]
pub struct PgConfig {
    pub alias: Option<String>,
    pub hostname: String,
    pub port: u16,
    pub dbname: String,
    pub username: String,
    pub password: String,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selecting_non_existant_config() {
        let config_list = PgConfigList::new();
        let result = config_list.select_config("test");
        assert_eq!(result, None);
    }
    
    #[test]
    fn selecting_existing_config() {
        let config = PgConfig {
            alias: Some("test".into()),
            hostname: "test".into(),
            port: 1234,
            dbname: "test".into(),
            username: "test".into(),
            password: "test".into(),
        };
        
        let expected = PgConfig {
            alias: Some("test".into()),
            hostname: "test".into(),
            port: 1234,
            dbname: "test".into(),
            username: "test".into(),
            password: "test".into(),
        };

        let config_list = PgConfigList { configs: vec![config] };
        let result = config_list.select_config("test").unwrap();
        assert_eq!(*result, expected);
    }
}
