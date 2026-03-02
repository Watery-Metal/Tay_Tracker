/// Tay-File Management for Reading and Writing Schedule Data

use std::{fs::OpenOptions};
use crate::{planning::TaySchedule};

pub fn path_from_user(user_name: &str) -> String {
    /*
    Takes the user's name, and returns the path. For now, no option to change storage location.
     */
    format!("./.tay/{}_schedule.tay", user_name)
}

pub fn read_schedule(user_name: &str) -> Option<TaySchedule> {
    println!("read_schedule() was called. No implementation...");
    // let file_path = path_from_user(user_name)
    None
}

pub fn save_schedule(file_path: &str) -> () {
    println!("save_schedule() was called. No implementation...");
}