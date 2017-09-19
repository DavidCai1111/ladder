extern crate clap;
extern crate ladder;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::fs::File;
use std::env;
use clap::App;
use ladder::{config, server, util};

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

    let cargoConfig: CargoConfig = toml::from_str(&util::read_file_content(
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

    server::run_server(&config::parse(&(current_dir_string + config)).unwrap());
}
