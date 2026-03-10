use crate::planning::{TaySchedule, ScheduleItem, taytime::TayTime};
use crate::style;


pub struct Project{
    id: u16,
    name: String,
    due_opt: Option<TayTime>,
    schedule: TaySchedule
}

impl Project {
    pub fn create(id: u16, name: String, due_opt: Option<TayTime>) -> Project {
        let sched_name = name.clone();
        Project { id, name, due_opt, schedule : TaySchedule::new(sched_name) }
    }
}

impl ScheduleItem for Project {
    fn concise_display(&self, depth: u8) {
        let output1 =  if self.due_opt.is_some() {
            format!("{}| Project #{}: {}", style::padding(depth), self.id, self.name)
        } else {
            format!("{}| Project #{} Due {}: {}", style::padding(depth), self.id, self.due_opt.as_ref().unwrap().stamp(), self.name)
        };
        println!("{}", output1);
    }
}