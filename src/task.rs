struct Task{
    pub title: String,
    pub task: String,
    pub completed: bool
}

impl Task{
    pub fn new(title: String, task: String) -> Task{
        Task{title: title, task: task, completed: false}
    }
}

pub struct TaskList{
    vect: Vec::<Task>
}

impl TaskList{

    pub fn new() -> TaskList{
        TaskList{vect: Vec::<Task>::new()}
    }

    pub fn add_task(&mut self, title: &str, task: &str){
        if !(self.exist(title)){
            self.vect.push(Task::new(title.to_string(), task.to_string()));
            println!("The task {title} has been add to the task list.");
        }else{
            println!("Its seems like you already had this task.");
        }
    }

    pub fn exist(&self, title: &str) -> bool{
        for task in self.vect.iter(){
            if task.title == title{
                return true;
            }
        }
        return false;
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
}