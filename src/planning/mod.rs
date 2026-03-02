// Houses Schedule logic
use crate::planning::{task::Task, project::Project, agenda::Agenda};

mod task;
mod agenda;
mod project;

pub struct TaySchedule{
    name : String,
    size : u16,
    task_list: Vec<Task>,
    agen_list: Vec<Agenda>,
    proj_list: Vec<Project>
}