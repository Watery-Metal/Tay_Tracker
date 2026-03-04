use std::io;

#[derive(PartialEq)]
pub enum UserChoice{
    Help,
    Cancel,
    Add,
    Scream,
    Edit(u16),
    Delete(u16),
    Complete(u16)
}

pub enum UserSelection{
    Agenda,
    Project,
    Task,
    Name,
    Date,
    NoSelection //Feature for when Choices don't require selections. User should never access this directly
}

pub fn parse_user_command(prompt: Option<&str>) -> (UserChoice, UserSelection) {
    /*
    General Function for accepting possibly many instructions from the user at once.
     */
    if prompt.is_some() {
        println!("{}", prompt.unwrap());
    } else {
        println!("What would you like to do?");
    }
    
    loop {
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Coudn't read from user-interface.");
        let arg_vec: Vec<&str> = response
            .trim()
            .split_ascii_whitespace()
            .collect();
        match arg_vec.len() {
            0 => {
                println!("No arguments were received. For information, type \"help\"\nWhat would you like to do:");
                continue
            }
            1 => {
                //Help, or ask for creation invocation of ScheduleItem. Otherwise, invalid.
                let user_choice = check_choices(arg_vec[0], None);
                match user_choice {
                    Some(UserChoice::Help) => {return (UserChoice::Help, UserSelection::NoSelection)}
                    Some(UserChoice::Cancel) => {return (UserChoice::Cancel, UserSelection::NoSelection)}
                    Some(UserChoice::Scream) => {return (UserChoice::Scream, UserSelection::NoSelection)}
                    _ => {
                        println!("Hmm, I didn't get enough arguments to recover a command...");
                    }
                }
            }
            _ => {
                //TODO
            }
        }
    }
    
}

fn check_choices(argument: &str, possible_id: Option<u16>) -> Option<UserChoice> {
    /*
    Used to parse a UserChoice from an arbitrary user argument.
    As a usage guide, we must have checked for id provision before invoking this function.
     */
    match argument.to_lowercase().trim() {
        "help" | "h" => {return Some(UserChoice::Help)}
        "add" | "a" => {return Some(UserChoice::Add)}
        "edit" | "e" => {
            if let Some(id_no) = possible_id {
                return Some(UserChoice::Edit(id_no))
            } else {
                // TODO: Give user the ability to specify later, maybe
                println!("Please specify by id waht you want to edit.");
                None
            }
            }

        "delete" | "d" => {
            if let Some(id_no) = possible_id {
                return Some(UserChoice::Delete(id_no))
            } else {
                println!("Please specify by id what you want to delete.");
                return None
            }
            }
        "complete" | "c" => {
            if let Some(id_no) = possible_id {
                return Some(UserChoice::Complete(id_no))
            } else {
                println!("Please specify by id what you want to complete.");
                return None
            }
            }
        "cancel" | "quit" | "q" => {
            return Some(UserChoice::Cancel)
        }
        "scream" => {
            return Some(UserChoice::Scream)
        }
        _ => {return None}
    }
}

fn check_selections(argument: &str)-> Option<UserSelection> {
    /*
    Used to parse a UserSelection from an arbitrary user argument.
     */
    match argument.to_lowercase().trim() {
        "agenda" => {return Some(UserSelection::Agenda)}
        "project" => {return Some(UserSelection::Project)}
        "task" => {return Some(UserSelection::Task)}
        "name" => {return Some(UserSelection::Name)}
        "date" => {return Some(UserSelection::Date)}
        _ => {return None}
        
    }
}

fn check_id(argument: &str) -> Option<u16> {
    /*
    Check if an arbitrary user_argument possibly matches an id number
     */
    if let Ok(id) = argument.parse::<u16>() {
        return Some(id)
    }
    return None
}
