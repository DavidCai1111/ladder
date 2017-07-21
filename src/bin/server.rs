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

#[derive(Debug, Deserialize)]
struct CargoConfig {
  name: String,
  version: String,
  description: String,
  author: Vec<String>,
}

fn main() {
  let cargoTomlPath = &(String::from(env::current_dir().unwrap().to_str().unwrap()) + "Cargo.toml");

  let cargoConfig: CargoConfig =
    toml::from_str(&read_file_content(cargoTomlPath).unwrap()).unwrap();

  App::new(cargoConfig.name.as_str())
    .version(cargoConfig.version.as_str())
    .about(cargoConfig.description.as_str())
    .get_matches();
}
