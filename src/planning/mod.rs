use crate::menus::{self, user_set_time};
// Houses Schedule logic
use crate::planning::{task::Task, project::Project, agenda::Agenda};
use crate::menus::{user_things::UserSelection, confirm};

mod task;
mod agenda;
mod project;
pub mod taytime;

pub trait ScheduleItem {
    fn concise_display(&self, depth: u8);
}

pub struct TaySchedule{
    name : String,
    size : u16,
    tid_list: Vec<u16>,
    task_list: Vec<Task>,
    aid_list: Vec<u16>,
    agen_list: Vec<Agenda>,
    pid_list: Vec<u16>,
    proj_list: Vec<Project>
}

impl TaySchedule {
    pub fn new(user_name: String) -> TaySchedule {
        TaySchedule {name: user_name, 
            size: 0, 
            task_list: Vec::new(), 
            agen_list: Vec::new(), 
            proj_list: Vec::new(),
            tid_list : Vec::new(),
            aid_list : Vec::new(),
            pid_list : Vec::new() }
    }

    pub fn concise_display_all(&self) {
        /*
        Call the concise display methods for every object in the schedule
         */
        println!("Schedule for {}:", self.name);
        let recursion = 0;
        for entry in &self.task_list{
            entry.concise_display(recursion);
        }
        for entry in &self.agen_list{
            entry.concise_display(recursion);
        }
        for entry in &self.proj_list{
            entry.concise_display(recursion);
        }
    }

    pub fn add_schedule_item(&mut self, selection: UserSelection) {
        let next_id = self.size;
        let title = menus::general_fetch(Some("Please enter your title:"), true);
        match selection {
            UserSelection::Task => {
                let incoming_task = Task::create(next_id, title);
                self.task_list.push(incoming_task);
                self.tid_list.push(next_id);
                self.size += 1;
            }
            UserSelection::Agenda => {
                let due_date = user_set_time(Some("Please enter a due-date for this Agenda (yyyy mm dd hh):"));
                if due_date.is_none() {
                    println!("No Due-Date was received, and so the agenda has been dropped.\nIf you would like to add a schedule item with no due-date, try \"task\"");
                    return
                } else {
                    let incoming_agenda = Agenda::create(next_id, title, due_date.unwrap());
                    self.agen_list.push(incoming_agenda);
                    self.aid_list.push(next_id);
                    self.size += 1;
                }
            }
            UserSelection::Project => {
                let due_date = if confirm(Some("Would you like this project to have a due date? (y/n):")) {
                    user_set_time(Some("Set a due date (yyyy mm dd hh):"))
                } else {None};
                
            }
            _ => {
                println!("Warning: Somehow, your schedule has just tried to update with something other than a Schedule-item.\nPlease bug the programmer.\n");
            }
        }

    }
}