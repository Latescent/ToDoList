pub enum TaskStatus {
    Todo,
    Done,
}

pub struct Task {
    id: u32,
    description: String,
    status: TaskStatus,
}

impl Task {
    pub fn new(id: u32, description: String, status: TaskStatus) -> Task {
        Task {
            id,
            description,
            status,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn complete(&mut self) {
        self.status = TaskStatus::Done;
    }

    pub fn show(&self) {
        let status: String = match self.status {
            TaskStatus::Done => String::from("Done"),
            TaskStatus::Todo => String::from("Todo"),
        };
        println!("#{}: {} - {}", self.id, self.description, status);
    }
}
