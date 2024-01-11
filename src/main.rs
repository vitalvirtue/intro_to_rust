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
    fn _new() -> ToDoList {
        ToDoList { _tasks: Vec::new() }
    }

    fn _add_task(&mut self, _description: &str) -> Task {
        let _new_task = Task {
            _id: self._tasks.len() + 1,
            _description: String::from(_description),
            _is_completed: false,
        };

        self._tasks.push(_new_task.clone());
        _new_task
    }

    fn _complete_task(&mut self, _id: usize) -> Option<&Task> {
        if let Some(_task) = self._tasks.iter_mut().find(|t| t._id == _id) {
            _task._is_completed = true;
            Some(_task)
        } else {
            None
        }
    }

    fn _list_tasks(&self) {
        for _task in &self._tasks {
            println!(
                "{:?}", _task
            );
        }
    }
}   

fn main() {
    let mut todo_list = ToDoList::_new();

    let _task1 = todo_list._add_task("Learn Rust");
    let _task2 = todo_list._add_task("Learn Python");
    let _task3 = todo_list._add_task("Learn C++");

    todo_list._list_tasks();

    todo_list._complete_task(_task1._id);
    todo_list._list_tasks();
}
