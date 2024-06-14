use serde::Deserialize;
use std::{collections::HashMap, fs};

// use toml;
//

type CargoToml = HashMap<String, Package>;

#[derive(Debug, Deserialize)]
struct Package {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    script: String,
}

pub fn read_toml() {
    let file = fs::read_to_string("config/packages.toml").expect("Failed to open file");

    let script_data: CargoToml = toml::from_str(&file).expect("Failed to deserialize file");

    println!("{:#?}", script_data)
}
