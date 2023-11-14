use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

struct Task{
    title: String,
    task: String,
    completed: bool
}

impl Task{
    fn new(title: String, task: String) -> Task{
        Task{title: title, task: task, completed: false}
    }
}

pub struct TaskList{
    vect: Vec::<Task>,
    file_name: String
}

impl TaskList{

    pub fn new(file_name: &str) -> TaskList{
        let mut task_vect = Vec::<Task>::new();
        let tasks_in_file = fs::read_to_string(file_name);
        match tasks_in_file {
            Ok(content) => {
                let mut lines: Vec<&str> = content.split('\n').collect();
                lines.pop();
                let mut i=0;
                while i<lines.len(){
                    task_vect.push(Task::new(lines[i].to_string(), lines[i+1].to_string()));
                    i += 2;
                }
            }
            Err(err) => {
                eprintln!("Vos taches n'ont pas pue ètre lu correctement : {:?}", err);
            }
        }
        TaskList{vect: task_vect, file_name: file_name.to_string()}
        
    }

    pub fn add_task(&mut self, title: &str, task: &str){
        if title == "" || task == "" {
            println!("Syntax error, the right way for add a task is new_task/title/task.")
        }else if !(self.get_index(title) != self.vect.len()){
            let mut fichier = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(self.file_name.clone());
            let _result = writeln!(fichier, "{title}\n{task}\n");
            self.vect.push(Task::new(title.to_string(), task.to_string()));
            println!("The task {title} has been add to the task list.");
        }else{
            println!("Its seems like you already had this task.");
        }
    }

    pub fn get_index(&self, title: &str) -> usize{
        for (i, task) in self.vect.iter().enumerate(){
            if task.title == title{
                return i;
            }
        }
        return self.vect.len();
    }

    pub fn display_task(&self){
        for task in self.vect.iter() {
            let mut s : String = format!("{}: {}    ", task.title, task.task);
            if task.completed{
                s += " complete.";
            }else{
                s += " incomplete.";
            }
            println!("{s}");
        }
        if self.vect.len() == 0{
            println!("No task yet.");
        }
    }

    pub fn remove_task(&mut self, task_to_remove: &str){
        for (i, task) in self.vect.iter().enumerate(){
            if task.title == task_to_remove {
                self.vect.remove(i);
                println!("The task {task_to_remove} has been destroyed.");
                return;
            }
        }
        println!("The task {task_to_remove} don't already exists.");
    }

    pub fn change_state(&mut self, title: &str){
        let i = self.get_index(title);
        if i == self.vect.len() {
            println!("The task {title} don't already exists.");
        }else{
            if self.vect[i].completed{
                println!("The task {title} is now incomplete.");
                self.vect[i].completed = false;
            }else{
                println!("The task {title} is now complete.");
                self.vect[i].completed = true;
            }
        }   
    }
}