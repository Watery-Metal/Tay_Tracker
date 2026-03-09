use crate::planning::ScheduleItem;
use crate::style;


pub struct Task{
    id: u16,
    name: String
}

impl Task{
    pub fn create(id: u16, name: String) -> Task {
        Task{id, name}
    }
}

impl ScheduleItem for Task {
    fn concise_display(&self, depth: u8) {
        let output1 = format!("{}| Task #{}: {}", style::padding(depth), self.id, self.name);
        println!("{}", output1);
    }
}