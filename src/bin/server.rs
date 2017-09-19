extern crate clap;
extern crate ladder;
extern crate serde;
#[macro_use]
extern crate serde_derive;
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
    package: CargoConfigPackage,
}

#[derive(Debug, Deserialize)]
struct CargoConfigPackage {
    name: String,
    version: String,
    description: String,
    authors: Vec<String>,
}

fn main() {
    let current_dir_string = String::from(env::current_dir().unwrap().to_str().unwrap());

    let cargoConfig: CargoConfig = toml::from_str(&read_file_content(
        &(current_dir_string.clone() + "/Cargo.toml"),
    ).unwrap())
        .unwrap();

    let matches = App::new(cargoConfig.package.name.as_str())
        .version(cargoConfig.package.version.as_str())
        .about(cargoConfig.package.description.as_str())
        .arg(
            clap::Arg::with_name("config")
                .short("c")
                .long("config")
                .help("Sets a custom config file"),
        )
        .get_matches();

    let config = matches.value_of("config").unwrap_or("/test/config.json");

    let ladderConfig = config::parse(&(current_dir_string + config)).unwrap();
}
