use crate::style;
use crate::planning::ScheduleItem;
use crate::planning::taytime::TayTime;

pub struct Agenda{
    id: u16,
    name: String,
    due: TayTime
}

impl Agenda{
    pub fn create(id: u16, name: String, due: TayTime) -> Agenda {
        Agenda{id, name, due}
    }
}

impl ScheduleItem for Agenda {
    fn concise_display(&self, depth: u8) {
        let output1 = format!("{}| Agenda #{}, Due {}: {}", style::padding(depth), self.id, self.due.stamp() ,self.name);
        println!("{}", output1);
    }
}