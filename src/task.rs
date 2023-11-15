use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

struct Task{
    title: String,
    task: String,
    completed: bool
}

impl Task{
    fn new(title: String, task: String) -> Task{
        Task{title: title, task: task, completed: false}
    }

    fn change_state(&mut self, display_the_change: bool){
        if self.completed{
            if display_the_change { println!("The task {} is now incomplete.", self.title)};
            self.completed = false;
        }else{
            if display_the_change { println!("The task {} is now complete.", self.title)};
            self.completed = true;
        }
    }
}

pub struct TaskList{
    vect: Vec::<Task>,
    file_name: PathBuf
}

impl TaskList{

    pub fn new(file_name: &str) -> TaskList{
        let mut task_vect = Vec::<Task>::new();
        let tasks_in_file = fs::read_to_string(file_name);
        match tasks_in_file {
            Ok(content) => {
                let mut lines: Vec<&str> = content.split('\n').collect();
                lines.pop();
                for task in lines{
                    let splited_task: Vec<&str> = task.split("|").collect();
                    let mut task = Task::new(splited_task[0].to_string(), splited_task[1].to_string());
                    if splited_task[2] == "true" {
                        task.change_state(false);
                    }
                    task_vect.push(task);
                }
            }Err(err) => {
                eprintln!("Failed to read your task list : {:?}", err);
            }
        }
        TaskList{vect: task_vect, file_name: PathBuf::from(file_name)}
        
    }

    pub fn add_task(&mut self, title: &str, task: &str){
        if title == "" || task == "" {
            println!("Syntax error, the right way for add a task is new_task/title/task.")
        }else if self.get_index(title) == self.vect.len(){
            let result = OpenOptions::new().append(true).open(self.file_name.clone());
            match result{
                Ok(mut fichier) => {
                    let _ = fichier.write_all(format!("{}|{}|{}\n", title, task, false).as_bytes());
                }
                Err(e) => {
                    println!("The writting of the task has failed: {e}");
                }
            }
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
        self.vect.iter().for_each(|task| {
            let status = if task.completed { "complete" } else { "incomplete" };
            println!("{}: {}    {}", task.title, task.task, status);
        });
        if self.vect.len() == 0{
            println!("No task yet.");
        }
    }

    pub fn remove_task(&mut self, task_to_remove: &str){
        for (i, task) in self.vect.iter().enumerate(){
            if task.title == task_to_remove {
                self.vect.remove(i);
                self.actualise_file();
                println!("The task {task_to_remove} has been destroyed.");
                return;
            }
        }
        println!("The task {task_to_remove} don't already exists.");
    }

    pub fn actualise_file(&self){
        let mut new_text : String = String::new();
        for task in &self.vect{
            new_text.push_str(&format!("{}|{}|{}\n", task.title , task.task, task.completed));
        }
        let _result = fs::write(self.file_name.clone(), new_text.as_bytes());
    }

    pub fn change_state(&mut self, title: &str){
        let i = self.get_index(title);
        if i == self.vect.len() {
            println!("The task {title} don't already exists.");
        }else{
            self.vect[i].change_state(true);
            self.actualise_file()
        }   
    }
}