use crate::planning::{TaySchedule, ScheduleItem};
use crate::style;


pub struct Project{
    id: u16,
    name: String,
    schedule: TaySchedule
}

impl ScheduleItem for Project {
    fn concise_display(&self, depth: u8) {
        let output1 = format!("{}| Project #{}: {}", style::padding(depth), self.id, self.name);
        println!("{}", output1);
    }
}