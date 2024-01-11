#[derive(Debug, Clone)]
struct Task {
    _id: usize,
    _description: String,
    _is_completed: bool,
    _tasks: Vec<Task>,
}

impl Task {
    fn _add_task(&mut self, _description: &str) -> Task {
        let new_task = Task {
            _id: self._tasks.len() + 1,
            _description: String::from(_description),
            _is_completed: false,
            _tasks: self._tasks.clone(),
        };
        self._tasks.push(new_task.clone());
        new_task
    }
}




fn main() {
    
}
