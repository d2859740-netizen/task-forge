use std::io;
use std::fs;

struct Task {
    text: String,
    status: bool,
}

fn main() {
    let mut main_vector: Vec<Task> = Vec::new();
    /// тут

    if let Ok(content) = fs::read_to_string("tasks.txt") {

        for line in content.lines() {

            let parts: Vec<&str> = line.splitn(2, '|').collect();
        
 
            if parts.len() == 2 {

                let status = parts[0] == "true";
                let text = parts[1].to_string();
            

                main_vector.push(Task { text, status });
            }
        }
    }
    loop {
        println!("=====Task_Forge=====\n1 - Add task.\n2 - Remove task \n3 - List of all tasks\n4 - Change task \n5 - Change status of task\n6 - Save data\n0 - Exit\n====================");
        
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("error");
        let user_input: i32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("error, enter number");
                continue; 
            }
        };

        match user_input {
            1 => {
                println!("Enter text of your task:");
                let mut new_task = String::new();
                io::stdin().read_line(&mut new_task).expect("error");
                
                let task1 = Task {
                    text: new_task.trim().to_string(),
                    status: false,
                };

                println!("Task added - {}, status - {}", task1.text, task1.status);
                main_vector.push(task1);
            }

            2 => {
                println!("Removing task...");
                if main_vector.is_empty() {
                    println!("your task list is empty now");
                    continue;
                } else {

                    for (i, t) in main_vector.iter().enumerate() {
                        println!("{}, [{}] {}", i + 1, if t.status { "x" } else { " " }, t.text);                    
                    }
                
                    println!("enter number of task to remove");
                    let mut user_num = String::new();
                    io::stdin().read_line(&mut user_num).expect("error");
                    
                    let user_num: usize = match user_num.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("error, enter count");
                            continue;
                        }
                    };
                    
                    if user_num == 0 || user_num > main_vector.len() {
                        println!("error, task not found");
                    } else {

                        main_vector.remove(user_num - 1);
                        println!("task was removed");
                    }
                }
            }
            
            3 => {
                println!("list of your task:");
                if main_vector.is_empty() {
                    println!("your list is empty now");
                } else {
                    for (i, t) in main_vector.iter().enumerate() {
                        println!("{}, [{}] {}", i + 1, if t.status { "x" } else { " " }, t.text);
                    }
                }
            }

            4 => {
                println!("print number of task");
                if main_vector.is_empty() {
                    println!("your task is empty now");
                    continue;
                } else {
                    for (i, t) in main_vector.iter().enumerate() {
                        println!("{}, [{}] {}", i + 1, if t.status { "x" } else { " " }, t.text);                    
                    }
                
                    println!("enter number of task");
                    let mut user_num = String::new();
                    io::stdin().read_line(&mut user_num).expect("error");
                    
                    let user_num: usize = match user_num.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("error, enter count");
                            continue;
                        }
                    };
                    
                    if user_num == 0 || user_num > main_vector.len() {
                        println!("error, task not found");
                    } else {
                        println!("new text of task");
                        let mut new_text = String::new();
                        io::stdin().read_line(&mut new_text).expect("error, enter text");
                        
                        main_vector[user_num - 1].text = new_text.trim().to_string();
                        println!("task was updated");
                    }
                }
            } 

            5 => {
                println!("print number of task");
                
                if main_vector.is_empty() {
                    println!("your task is empty now");
                    continue;
                } else {
                    for (i, t) in main_vector.iter().enumerate() {
                        println!("{}, [{}] {}", i + 1, if t.status { "x" } else { " " }, t.text);                    
                    }                    
                }

                println!("enter number of task");
                let mut user_num1 = String::new();
                io::stdin().read_line(&mut user_num1).expect("error");
                    
                let user_num1: usize = match user_num1.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("error, enter count");
                        continue;
                    }
                };

                if user_num1 == 0 || user_num1 > main_vector.len() {
                    println!("error, task not found");
                } else {
                    main_vector[user_num1 - 1].status = !main_vector[user_num1 - 1].status;
                    println!("task was updated");
                }   
            } 

            6 => {
                println!("saving data");
                let mut saving = String::new();

                for task in &main_vector {
                    let line = format!("{}|{}\n", task.status, task.text);
                    saving.push_str(&line);
                }
                
                match fs::write("tasks.txt", saving) {
                    Ok(_) => println!("Data was saved successfully!"),
                    Err(e) => println!("Error saving data: {}", e),
                }
            }

            0 => {
                println!("exit...");
                break;
            }

            _ => {
                println!("Unknown command");
            }
        }
    }
}