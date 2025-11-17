// Config

use std::fs;
use dirs_next;
use std::path::PathBuf;
use mlua::Lua;

use crate::add_api::add_api;

pub struct Config {
    // CONFIG VALUES
    wait_time: u64,
    show_info: bool,
    loglevel: u8,
}

// Default-Trait for Config
impl Default for Config {
    fn default() -> Self {
        Config {
            wait_time: 3,    // z. B. 3 s als Standardwert
            show_info: true, // Standard auf true
            loglevel: 3,     // Set Loglevel to Debug
        }
    }
}

pub fn loadconfig(localpath: String) -> Config {
    let global_path = get_config_path();

    // Read the Content for the config File
    let contents: String = match fs::read_to_string(&global_path) {
        Ok(c) => c,
        Err(_) => {
            println!("Config file not found, using default Config.");
            return Config::default();
        }
    };

    // Create a new Lua Instance
    let lua = Lua::new();

    // Register all APIs
    let _ = add_api(&lua);

    return Config::default();
}

// Function to get the Config Path
fn get_config_path() -> PathBuf {
    let mut path: PathBuf = dirs_next::config_dir().expect("could not find config_dir()");

    path.push("@shadowdara");
    path.push("flua");
    path.push("config.lua");

    return path;
}
