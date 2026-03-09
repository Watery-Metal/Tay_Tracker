// Menu Loops for various stages of the program
use std::io;
use crate::{tay_files, planning::{TaySchedule, taytime::TayTime},
    menus::user_things::{UserChoice, UserSelection, parse_user_command}
    };

pub mod user_things;

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

pub fn user_set_time(prompt: Option<&str>) -> Option<TayTime> {
    let mut response: String = String::new();
    if prompt.is_some() {
        println!("{}", prompt.unwrap());
    } else {
        println!("Enter your Date (yyyy mm dd hh):");
    }
    loop {
        io::stdin().read_line(&mut response).expect("Coudn't read from user-interface");
        let mut dig_vec : Vec<&str> = response.trim().split_ascii_whitespace().collect();
        match dig_vec.len() {
            0 => {
                println!("Nothing was received; Aborting");
                return None
            }
            1 => {
                if let Ok(year) = dig_vec[0].parse::<u16>() {
                    return Some(TayTime::create(year, None, None, None))
                } else {
                    println!("Unable to parse input as a year; Aborting.");
                    return None
                }
            }
            2 | 3 | 4 => {
                //Handling Year seperate since it's a different data-type
                let ye_pos = dig_vec.remove(0);
                let year : u16;
                if let Ok(parsed_year) = ye_pos.parse::<u16>() {
                    year = parsed_year
                } else {
                    println!("Unable to parse input as a year; Aborting.");
                    return None
                }
                let mut parsed_res = dig_vec.into_iter()
                    .map(|digit| {digit.parse::<u8>()})
                    .collect::<Vec<Result<u8,_>>>();
                parsed_res.retain(|pars_res| {pars_res.is_ok()});
                    
                let fields = parsed_res.into_iter()
                    .map(|kept| {Some(kept.unwrap())})
                    .collect::<Vec<Option<u8>>>();
                println!("Debugging: I was able to identify {} fields from this input!", fields.len() + 1);
                match fields.len() {
                    0 => {return Some(TayTime::create(year, None, None, None))}
                    1 => {return Some(TayTime::create(year, fields[0],None, None))}
                    2 => {return Some(TayTime::create(year, fields[0], fields[1], None))}
                    3 => {return Some(TayTime::create(year, fields[0], fields[1],fields[2]))}
                    _ => {println!("Impossible branch reached in the user_set_time() function. Bother your programmer!");
                    return None
                }
                }
            }
            _ => {
                println!("Too many arguments; Aborting");
                return None
            }
        }
    }
}

pub fn general_fetch(prompt: Option<&str>, check: bool) -> String {
    let mut response: String = String::new();
    if prompt.is_some() {
        println!("{}", prompt.unwrap());
    } else {
        println!("Enter your text:");
    }
    loop {
        io::stdin().read_line(&mut response).expect("Couldn't read from user-interface");
        if check {
            if confirm(Some("Does your input look correct? (y/n):")) {
                return response.trim().to_owned()
            } else {
                response = String::new();
                println!("Please re-enter your text:");
                continue
            }
        } else {
            break
        }
    }
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


pub fn parent_menu(mut sched : TaySchedule){
    /*
    Main Menu for the user's schedule
     */
    loop {
        let activity_tuple = parse_user_command(Some("\nYou are in the main menu. What would you like to do?"));
        match activity_tuple {
            (UserChoice::Cancel, _) => {
                if confirm(Some("Would you like to quit Tay_Tracker? (y/n)")) {break}
                continue
            }
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
                sched.add_schedule_item(UserSelection::Task);
            }
            (UserChoice::Add, UserSelection::Agenda) => {
                sched.add_schedule_item(UserSelection::Agenda);
            }
            (UserChoice::Add, UserSelection::Project) => {
                sched.add_schedule_item(UserSelection::Project);
            }
            _ => {
                println!("Unhandled Command returned! Bother your programmer!");
            }
        }
    }
}
