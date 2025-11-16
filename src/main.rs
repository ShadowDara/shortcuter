/// Shortcuter
/// a simple program by to make shortcuts like make
/// but more advanced althought with Lua

use win_utf8_rs::enable_utf8;

mod json_loader;
mod lua_loader;

fn main() {
    // Enable UTF 8 for Windows
    let _ = enable_utf8();

    println!("Hello, world!");
}
