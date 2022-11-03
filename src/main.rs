use std::env;

enum Command {
    Get,
    Post(String),
    Put(usize),
    Delete(usize),
}

struct TodoItem {
    name: String,
    completed: char,
}

impl TodoItem {
    fn new(name: String) -> TodoItem {
        return TodoItem {
            name,
            completed: ' ',
        };
    }
}

struct TodoList {
    list: Vec<TodoItem>,
}

impl TodoList {
    fn new() -> TodoList {
        return TodoList { list: Vec::new() };
    }

    fn add_to_list(&mut self, name: String) {
        let todo_item: TodoItem = TodoItem::new(name);
        self.list.push(todo_item);
    }

    fn update_list(&mut self, index: usize) {
        if self.list[index].completed == ' ' {
            self.list[index].completed = 'x';
        } else {
            self.list[index].completed = ' ';
        }
    }

    fn delete_from_list(&mut self, index: usize) {
        self.list.remove(index);
    }

    fn get_todos(&self) {
        for (index, todo) in self.list.iter().enumerate() {
            println!("{}. [{}] - {}", index, todo.completed, todo.name);
        }
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let mut todo_items: TodoList = TodoList::new();

    let command: Command = match arguments[1].as_str() {
        "get" => Command::Get,
        "post" => Command::Post(arguments[2].clone()),
        "put" => Command::Put(
            arguments[2]
                .clone()
                .parse()
                .expect("Error converting to interger, please provide integer value!"),
        ),
        "delete" => Command::Delete(
            arguments[2]
                .clone()
                .parse()
                .expect("Error converting to interger, please provide integer value!"),
        ),
        _ => panic!("get, post (add your todo), put (put todo index), delete (put todo index)"),
    };

    todo_items.add_to_list("Buy groceries for home".to_string());
    todo_items.add_to_list("Take caffein for coffee".to_string());

    match command {
        Command::Get => todo_items.get_todos(),
        Command::Post(todo) => {
            todo_items.add_to_list(todo);
            todo_items.get_todos();
        }
        Command::Put(index) => {
            todo_items.update_list(index);
            todo_items.get_todos();
        }
        Command::Delete(index) => {
            todo_items.delete_from_list(index);
            todo_items.get_todos();
        }
    }
}
