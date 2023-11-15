use std::io;
mod task;
use crate::task::TaskList;

fn main() {
    let mut task_vec = TaskList::new("./src/saved_task.txt");
    loop{
        ask_for_command(&mut task_vec);
    }
}

fn ask_for_command(task_vec : &mut TaskList){ 
    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("Failed to read command");
    command.pop();
    let mut splited_command : Vec<_>= command.split('/').collect();
    while splited_command.len() < 3{
        splited_command.push("");
    }
    let type_command: &str = splited_command[0];
    let title: &str = splited_command[1];
    let task: &str = splited_command[2];
    match type_command {
        "new_task" => task_vec.add_task(title, task),
        "display" => task_vec.display_task(),
        "remove" => task_vec.remove_task(title),
        "change" => task_vec.change_state(title),
        _ if type_command != "" => println!("Unknown command"),
        _ => {},
    }
}