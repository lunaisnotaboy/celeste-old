//* Code for reading from & parsing the config file *//
use configparser::ini::Ini;
use std::fs;

// TODO: Error handling, functions to return parts of the config to other functions
pub fn read_config() -> Ini {
    // Read in the config file
    let config_file = fs::read_to_string("config.ini").expect("Failed to read file");

    // Parse the config
    let mut config = Ini::new();
    config.read(config_file);

    return config;
}
