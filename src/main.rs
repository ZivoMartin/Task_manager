use std::io;

fn main() {
    let mut tache_vec = Vec::<String>::new();
    loop{
        ask_for_command(&mut tache_vec);
    }
}


fn ask_for_command(tache_vec : &mut Vec::<String>){
    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("Failed to read command");

    let mut splited_command = command.split('/');
    let type_command: &str = splited_command.next().unwrap_or("{command}");
    let content: &str = splited_command.next().unwrap_or("");
    match type_command {
        "new_task" if !content.is_empty() => new_task(tache_vec, content),
        "display" => display_task(tache_vec),
        _ if !(type_command == "\n") => println!("Unknown command"),
        _ => {},
    }
    
}

fn new_task(tache_vec : &mut Vec::<String>, content: &str){
    tache_vec.push(content.to_string());
}

fn display_task(tache_vec : &mut Vec::<String>){
    for (i, task) in tache_vec.iter().enumerate() {
        println!("Task {}: {}", i+1, task);
    }
}
