use std::collections::HashMap;
use std::io::stdin;

fn main() {
    let mut todo: HashMap<i32, String> = HashMap::new();

    let mut idx: i32 = 0;

    loop {
        println!("1. Add to Todo List");
        println!("2. View Todo List");
        println!("3. Delete Todo by Index");
        println!("4. Exit");

        let mut option: String = String::new();

        stdin()
            .read_line(&mut option)
            .expect("Failed to read input");
    

        match option.trim() {
            "1" =>{
                idx += 1;
                add_to_todo(&mut todo, idx);
            },
            "2" => {
                println!("\nTodo values");
                for (key, val) in todo.iter() {
                    println!("{key} : {val}");
                }
                println!("");
            },
            "3" =>{
                delete_from_todo(&mut todo);
            },
            "4" => {
                println!("Good Bye!!");
                break;
            },
             _  => println!("Invalid option selected")
        };
    }
}

fn add_to_todo(todo: &mut HashMap<i32, String>, idx: i32) {
    let mut todo_value: String = String::new();

    stdin()
        .read_line(&mut todo_value)
        .expect("Failed to read input");


    todo.insert(idx, todo_value.trim().to_string());
    println!("Added '{}' successfully\n", todo_value.trim());
}

fn delete_from_todo(todo: &mut HashMap<i32, String>){
    let mut delete_value: String = String::new();

    stdin()
        .read_line(&mut delete_value)
        .expect("Failed to read input");

    let delete_idx:i32 = delete_value.trim().to_string().parse::<i32>().unwrap();

    if !todo.contains_key(&delete_idx){
        println!("Invalid index inputed\n");
        return;
    }

    let deleted_data = todo.remove(&delete_idx).expect("Failed to delete the string");
    println!("{deleted_data} has been successfully deleted\n");
}