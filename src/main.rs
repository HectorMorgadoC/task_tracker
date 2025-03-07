mod interface;
mod data;
mod menu;
mod crud;
mod common;

use crate::menu::menu::{main_menu,option_menu};

fn main() {

    option_menu(main_menu());


}



