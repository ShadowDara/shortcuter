use dirs_next;
use mlua::{Function, Lua, Result, Table};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::add_api::add_api;

pub struct Config {
    // pub wait_time: u64,
    // pub show_info: bool,
    // pub loglevel: u8,
    pub funcs: HashMap<String, Function>, // kein 'lua
}

impl Default for Config {
    fn default() -> Self {
        Config {
            // wait_time: 3,
            // show_info: true,
            // loglevel: 3,
            funcs: HashMap::new(),
        }
    }
}

// Function to load the Config File
pub fn load_config(local_path: PathBuf) -> Result<Config> {
    let global_path = get_config_path();

    let mut global_file = global_path;
    global_file.push("config.lua");

    // Datei lesen
    let global_contents = match fs::read_to_string(&global_file) {
        Ok(c) => c,
        Err(_) => {
            println!("Config file not found, using default Config.");
            return Ok(Config::default());
        }
    };

    // Lua-Instanz
    let lua = Lua::new();

    // API registrieren
    let _ = add_api(&lua);

    // Lua-Argument-Tabelle
    let lua_arg = lua.create_table()?;
    lua.globals().set("arg", lua_arg)?;

    // Lua ausführen
    lua.load(&global_contents).exec()?;

    // Lua-Tabelle auslesen
    let config_table: Table = lua.globals().get("c")?;

    // let wait_time: u64 = config_table.get("wait_time").unwrap_or(3);
    // let show_info: bool = config_table.get("show_info").unwrap_or(true);
    // let loglevel: u8 = config_table.get("loglevel").unwrap_or(3);

    // Funktionen auslesen
    let funcs_table: Table = config_table.get("funcs").unwrap_or(lua.create_table()?);
    let mut funcs: HashMap<String, Function> = HashMap::new();

    for pair in funcs_table.pairs::<String, Function>() {
        let (name, func) = pair?;
        funcs.insert(name, func);
    }

    Ok(Config {
        // wait_time,
        // show_info,
        // loglevel,
        funcs,
    })
}

// Config-Pfad bestimmen
pub fn get_config_path() -> PathBuf {
    let mut path: PathBuf = dirs_next::config_dir().expect("could not find config_dir()");
    path.push("@shadowdara");
    path.push("sc");
    path
}
