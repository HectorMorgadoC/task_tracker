mod common;
mod crud;
mod data;
mod interface;
mod menu;

use crate::menu::menu::option_menu;

fn main() {
    let path: &str = "data.json";
    option_menu(path);
}
