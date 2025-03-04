mod interface;
mod data;
mod menu;
mod crud;

use crate::menu::menu::{main_menu,option_menu};

fn main() {

    option_menu(main_menu());


}



