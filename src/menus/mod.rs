// Menu Loops for various stages of the program
use std::io;
use crate::{tay_files, planning::TaySchedule, menus::user_things::{UserChoice, UserSelection, parse_user_command}};

mod user_things;

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

pub fn general_fetch(prompt: Option<&str>) -> String {
    let mut response: String = String::new();
    if prompt.is_some() {
        println!("{}", prompt.unwrap());
    } else {
        println!("Enter your text:");
    }
    io::stdin().read_line(&mut response).expect("Couldn't read from user-interface");
    return response.trim().to_owned()
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

pub fn initialize_schedule(user_name: String) -> Option<TaySchedule> {
    /*
    Search for the schedule of a user, and fetch if it's present. Otherwise, offer initialization.
     */
    if let Some(user_schedule) = tay_files::read_schedule(&user_name) {
        println!("Your schedule was found on file!");
        return Some(user_schedule)
    } else {
        let make_new = confirm(Some("No file was found for this username. Create a new profile? (y/n):"));
        if make_new {
            println!("A schedule has been opened for {}.", user_name);
            return Some(TaySchedule::new(user_name))
        } else {
            return None
        }
    }
}


pub fn parent_menu(sched : TaySchedule){
    /*
    Main Menu for the user's schedule
     */
    loop {
        let activity_tuple = parse_user_command(Some("\nYou are in the main menu. What would you like to do?"));
        match activity_tuple {
            (UserChoice::Cancel, _) => {break}
            (UserChoice::Scream, UserSelection::NoSelection) => {sched.concise_display_all();}
            (UserChoice::Help, UserSelection::NoSelection) => {
                // TODO: make a nicer way to print big text blocks at once.
                // TODO: update the Help menu with all the commands.
                println!("\n    Right now, you're in the main menu for your schedule. To do things, you enter commands to the terminal, and hit enter.\n");
                println!("    \"Help\" and \"Quit\" are single word commands you can use to get clarification on your current menu, or back out of whatever you're doing, at any time. Additionally, you can use the command \"Scream\" to show the basic info of everything containing in the current schedule. Note that in the menu for a Project, this command will only call display on the Project itself. To see your full schedule, be sure to scream in this menu!");
                println!("    To add a new Task, Project, or Agenda into your schedule, you can type \"Add\", as well as what type of item you want to add. If I've implemented everything nicely, there are even some abbreviations that will work, and you can type commands more or less in any order!");
                println!("    To make changes, you can use the commands \"Edit\", \"Delete\", and \"Complete\", followed (or preceeded) by the ID number of what you want to change.");
            }
            (UserChoice::Add, UserSelection::Task) => {

            }

            _ => {
                println!("Unhandled Command returned! Bother your programmer!");
            }
        }
    }
}

fn get_name() -> Option<String> {

}