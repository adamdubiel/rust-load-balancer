
extern crate toml;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use resolver::GroupsConfig;

pub fn read_configuration(path: String) -> GroupsConfig {
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => { panic!("Unable to open file {}: {}", path, Error::description(&why)) }
    };
    let mut content = String::new();

    match file.read_to_string(&mut content) {
        Err(why) => { panic!("Unable to read from file {}: {}", path, Error::description(&why)) },
        Ok(_) => {}
    };

    let mut parser = toml::Parser::new(&content);
    let raw_values = match parser.parse() {
        Some(value) => value,
        _ => panic!("Failed to read configuration file {} because of following errors: {:?}", path, parser.errors)
    };

    parse_configuration(&raw_values)
}

fn parse_configuration(table: &toml::Table) -> GroupsConfig {
    let mut config = GroupsConfig::new();

    for (key, value) in table.iter() {
        let weight = value.as_integer().unwrap() as u64;
        config.add(key.to_owned(), weight);
    }

    return config;
}
