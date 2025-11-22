/// Shortcuter
/// a simple program by to make shortcuts like make
/// but more advanced althought with Lua
use loader;
use std::{env, path::PathBuf};
use win_utf8_rs::enable_utf8;

fn main() {
    // UTF-8 für Windows aktivieren
    let _ = enable_utf8();

    // Start Message
    println!("\x1b[32mShortcuter\x1b[0m");

    // Programmargumente
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please supply min 1 argument or run with config for more Infos!");
        return;
    }

    // args[1] ist der Funktionsname
    let func_name = &args[1];

    // Show Config
    if func_name == "config" {
        loader::print_info();
        return;
    }

    let mut list = false;

    // Display all Registered Functions
    if func_name == "list" {
        list = true;
    }

    // Pfad der exe
    let exe_path = env::current_exe().unwrap();
    let exe_dir: PathBuf = exe_path.parent().unwrap().to_path_buf();

    // Config laden
    let config = match loader::config_rs::load_config(exe_dir) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error while Loading the Config: {}", e);
            return;
        }
    };

    if list {
        // List all Functions
        println!("Listing Registered Functions");
        for (key, _) in &config.funcs {
            println!("{}", key);
        }
    } else {
        // Run a function
        if let Some(func) = config.funcs.get(func_name) {
            // Funktion existiert → aufrufen
            if let Err(e) = func.call::<()>(()) {
                eprintln!("Error while calling '{}': {}", func_name, e);
            }
        } else {
            println!("No Lua function with name '{}' found.", func_name);
        }
    }
}
