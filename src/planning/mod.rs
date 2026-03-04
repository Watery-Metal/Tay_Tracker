// Houses Schedule logic
use crate::planning::{task::Task, project::Project, agenda::Agenda};

mod task;
mod agenda;
mod project;

pub trait ScheduleItem {
    fn concise_display(&self, depth: u8);
}

pub struct TaySchedule{
    name : String,
    size : u16,
    task_list: Vec<Task>,
    agen_list: Vec<Agenda>,
    proj_list: Vec<Project>
}

impl TaySchedule {
    pub fn new(user_name: String) -> TaySchedule {
        TaySchedule { name: user_name, size: 0, task_list: Vec::new(), agen_list: Vec::new(), proj_list: Vec::new() }
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
}