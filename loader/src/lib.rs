use crate::config_rs::get_config_path;

mod add_api;
pub mod config_rs;

pub fn print_info() {
    println!("Shortcuter Help");
    println!("\nReserved Keywords for callable Function Names");
    println!("    config");
    println!("    list");
    println!("\nMore Infos:");
    println!("https://github.com/ShadowDara/shortcuter");
    println!("\nConfig Path");
    let mut path = get_config_path();
    println!("{}", path.display().to_string());
    path.push("config.lua");
    println!("{}", path.display().to_string());
}
