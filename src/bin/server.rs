#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate ladder;
extern crate clap;
extern crate toml;

use std::fs::File;
use std::env;
use clap::App;
use ladder::util::read_file_content;
use ladder::config;

const idType: i32 = 0;
const idIP0: i32 = 1;
const idDmLen: i32 = 1;
const idDm0: i32 = 2;

const typeIPv4: i32 = 1;
const typeDm: i32 = 3;
const typeIPv6: i32 = 4;

const lenIPv4: usize = 6;
const lenIPv6: usize = 18;
const lenDmBase: usize = 2;
const lenHmacSha1: usize = 10;

const addrMask: u8 = 0xf;

#[derive(Debug, Deserialize)]
struct CargoConfig {
    name: String,
    version: String,
    description: String,
    author: Vec<String>,
}

fn main() {
    let current_dir_string = String::from(env::current_dir().unwrap().to_str().unwrap());

    let cargoConfig: CargoConfig =
        toml::from_str(&read_file_content(&(current_dir_string.clone() + "/Cargo.toml")).unwrap())
            .unwrap();

    let ladderConfig = config::parse(&(current_dir_string + "/config.json")).unwrap();

    App::new(cargoConfig.name.as_str())
        .version(cargoConfig.version.as_str())
        .about(cargoConfig.description.as_str())
        .get_matches();
}
