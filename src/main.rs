#[derive(Debug, Clone)]
struct Task {
    _id: usize,
    _description: String,
    _is_completed: bool,
}

struct ToDoList {
    _tasks: Vec<Task>,
}

impl ToDoList {
    fn new() -> ToDoList {
        ToDoList { _tasks: Vec::new() }
    }

    fn add_task(&mut self, _description: &str) -> Task {
        let _new_task = Task {
            _id: self._tasks.len() + 1,
            _description: String::from(_description),
            _is_completed: false,
        };

        self._tasks.push(_new_task.clone());
        _new_task
    }

    fn complete_task(&mut self, _id: usize) -> Option<&Task> {
        if let Some(_task) = self._tasks.iter_mut().find(|t| t._id == _id) {
            _task._is_completed = true;
            Some(_task)
        } else {
            None
        }
    }

    fn list_tasks(&self) {
        for _task in &self._tasks {
            println!(
                "ID: {}, Description: {}, Is Completed: {}",
                _task._id, _task._description, _task._is_completed
            );
        }
    }
}

fn main() {

}
