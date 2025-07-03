mod task;

use task::{Task, TaskStatus};

struct TodoList {
    tasks: Vec<task::Task>,
    next_id: u32,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_task(&mut self, description: String) {
        self.tasks.push(task::Task::new(
            self.next_id,
            description,
            task::TaskStatus::Todo,
        ));
        self.next_id += 1;
    }

    pub fn complete_task(&mut self, task_id: u32) -> Option<()> {
        for item in &mut self.tasks {
            if item.id() == task_id {
                item.complete();
                return Some(());
            }
        }
        return None;
    }

    pub fn list_tasks(&self) {
        println!("List of all tasks:");
        for item in &self.tasks {
            item.show();
        }
        println!("Done!");
    }
}

fn main() {
    let mut task_list: TodoList = TodoList::new();
    task_list.add_task(String::from("Learn Rust."));
    task_list.add_task(String::from("Build a project."));
    task_list.list_tasks();
    task_list.complete_task(1);
    task_list.list_tasks();
}
