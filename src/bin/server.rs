extern crate clap;
extern crate ladder;
#[macro_use]
extern crate serde_derive;
extern crate toml;

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

    let cargo_config: CargoConfig = toml::from_str(&util::read_file_content(
        &(current_dir_string.clone() + "/Cargo.toml"),
    ).unwrap())
        .unwrap();

    let matches = App::new(cargo_config.package.name.as_str())
        .version(cargo_config.package.version.as_str())
        .about(cargo_config.package.description.as_str())
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
