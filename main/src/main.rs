/// Shortcuter
/// a simple program by to make shortcuts like make
/// but more advanced althought with Lua

use std::env;
use loader;
use win_utf8_rs::enable_utf8;

fn main() {
    // Enable UTF 8 for Windows
    let _ = enable_utf8();

    // Array of Programm Arguments
    let args: Vec<String> = env::args().collect();

    // Load the Config
    let config = loader::config_rs::loadconfig();

    println!("Hello, world!");
}
