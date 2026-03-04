use crate::style;
use crate::planning::ScheduleItem;

static NODUEDATE: &str = "No due Date.";

pub struct Agenda{
    id: u16,
    name: String,
    due: String
}

impl Agenda{
    fn create(id: u16, name: String, due_opt: Option<String>) -> Agenda {
        let due: String;
        if due_opt.is_some() {
            due = due_opt.unwrap();
        } else {
            due = NODUEDATE.to_owned();
        }
        Agenda{id, name, due}
    }
}

impl ScheduleItem for Agenda {
    fn concise_display(&self, depth: u8) {
        let output1 = format!("{}| Agenda #{}: {}", style::padding(depth), self.id, self.name);
        println!("{}", output1);
    }
}