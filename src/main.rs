// Define a Task struct with fields for task ID, description, and completion status
#[derive(Debug, Clone)]
struct Task {
    _id: usize,
    _description: String,
    _is_completed: bool,
}

// Define a ToDoList struct to manage a list of tasks
struct ToDoList {
    _tasks: Vec<Task>,
}

// Implementation of methods for the ToDoList struct
impl ToDoList {
    // Create a new ToDoList with an empty task vector
    fn _new() -> ToDoList {
        ToDoList { _tasks: Vec::new() }
    }

    // Add a new task to the ToDoList with a unique ID
    fn _add_task(&mut self, _description: &str) -> Task {
        let _new_task = Task {
            _id: self._tasks.len() + 1,
            _description: String::from(_description),
            _is_completed: false,
        };

        // Clone the new task and add it to the task vector
        self._tasks.push(_new_task.clone());
        _new_task
    }

    // Mark a task as completed based on the provided ID
    fn _complete_task(&mut self, _id: usize) -> Option<&Task> {
        // Find the task with the given ID in the task vector
        if let Some(_task) = self._tasks.iter_mut().find(|t| t._id == _id) {
            // Mark the task as completed and return a reference to it
            _task._is_completed = true;
            Some(_task)
        } else {
            // Return None if the task with the given ID is not found
            None
        }
    }

    // Print the details of all tasks in the ToDoList
    fn _list_tasks(&self) {
        for _task in &self._tasks {
            // Print the debug representation of each task
            println!("{:?}", _task);
        }
    }
}

// Main function to demonstrate the ToDoList functionality
fn main() {
    // Create a new ToDoList instance
    let mut todo_list = ToDoList::_new();

    // Add tasks to the ToDoList
    let _task1 = todo_list._add_task("Learn Rust");
    let _task2 = todo_list._add_task("Learn Python");
    let _task3 = todo_list._add_task("Learn C++");

    // Print the initial list of tasks
    todo_list._list_tasks();

    // Complete the first task and print the updated list of tasks
    todo_list._complete_task(_task1._id);
    todo_list._list_tasks();
}
