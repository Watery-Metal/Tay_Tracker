// Menu Loops for various stages of the program
use std::io;
use crate::tay_files;


pub fn fetch_user() -> String {
    let mut user_name: String = String::new();
    println!("Enter your username:");
    loop{
        io::stdin().read_line(&mut user_name).expect("Couldn't read from user-interface.");
        let prompt = format!("Is {} correct?", user_name.trim());
        let name_good = confirm(Some(&prompt));
        if name_good {
            return user_name.trim().to_owned()
        } else {
            user_name = String::new();
            println!("Re-enter username:");
        }
    }
    
    
}

pub fn confirm(prompt: Option<&str>) -> bool {
    /*
    Prompt the user to confirm or deny with yes/no, or y/n. Case Insensitive
    The argument allows you to change the confirmation prompt.
     */
    if prompt.is_some(){
        println!("{}", prompt.unwrap());
    } else {
        println!("Confirm (y/n):");
    }

    loop {
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Coudn't read from user-interface.");
        match response.to_lowercase().trim() {
            "yes" | "y" => {return true}
            "no" | "n" => {return false}
            _ => {
                println!("Response was unable to be parsed for confirmation. Please confirm with \"y\" or \"n\"");
            }
        }
    }
}

pub fn initialize_schedule(user_name: String) {
    if let Some(user_schedule) = tay_files::read_schedule(&user_name) {
        println!();
    } else {
        let decision = confirm(None);
    }
}