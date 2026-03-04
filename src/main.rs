use std::env;

mod tay_files;
mod custom_errors;
mod menus;
mod planning;
mod style;

static LOGO : &str = r"  _____             _____               _             
 |_   _|_ _ _   _  |_   _| __ __ _  ___| | _____ _ __ 
   | |/ _` | | | |   | || '__/ _` |/ __| |/ / _ \ '__|
   | | (_| | |_| |   | || | | (_| | (__|   <  __/ |   
   |_|\__,_|\__, |___|_||_|  \__,_|\___|_|\_\___|_|   
            |___/_____|                               ";

fn main() {
    start_routine();

    // Check for command-line start
    let start_arguments: Vec<String> = env::args().collect();

    let user_name : String;
    if start_arguments.len() == 0 {
        //TODO fix the broken CLI parsing.
        println!("Taking Username from CLI arguments.");
        user_name = start_arguments[0].clone()
    } else {
        user_name = menus::fetch_user();
    }

    if let Some(user_schedule) = menus::initialize_schedule(user_name) {
        menus::parent_menu(user_schedule);
        println!("Thanks for using Tay_Tracker!\nCome again soon!");
    } else {
        println!("Thanks for using Tay_Tracker!");
    }
}

fn start_routine() {
    // We clear the terminal, and display a logo before polling the user for information.
    print!("{}[2J", 27 as char);
    println!("{}", LOGO);
    println!("\n\n\n\n");
}