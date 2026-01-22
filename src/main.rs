mod tay_files;
mod custom_errors;
mod menus;

static LOGO : &str = r"  _____             _____               _             
 |_   _|_ _ _   _  |_   _| __ __ _  ___| | _____ _ __ 
   | |/ _` | | | |   | || '__/ _` |/ __| |/ / _ \ '__|
   | | (_| | |_| |   | || | | (_| | (__|   <  __/ |   
   |_|\__,_|\__, |___|_||_|  \__,_|\___|_|\_\___|_|   
            |___/_____|                               ";

fn main() {
    start_routine();
}

fn start_routine() {
    // We clear the terminal, and display a logo before polling the user for information.
    print!("{}[2J", 27 as char);
    println!("{}", LOGO);
    println!("\n\n\n\n");
}