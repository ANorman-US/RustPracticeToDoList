use std::{io, process::exit};

static HELP_TEXT: &str = r#"+---------------------------+
List of commands and syntax:
    add <list item>
        (adds a new item to the todo list)
    remove <# index>
        (removes the item at # index)
    show
        (prints contents of list)
    help
        (prints this list of commands and syntax)
    quit
        (exits the program)
+---------------------------+"#;

/*enum InputCommand {
    Add,
    Remove,
    Show,
    Help,
    Quit,
}*/
fn main()
{
    println!("Welcome to the Todo List! Type \"help\" to get started...");
    let mut user_token: &str = "";
    let mut todo_vector: Vec<String> = Vec::new();
    loop
    {

        println!("Enter Command");

        let mut user_input: String = String::new();
    
        io::stdin().read_line(&mut user_input).expect("Failed to Read Line");
    
        let mut it = user_input.trim().split_whitespace();
        
        match it.next(){
            Some(token) => user_token = token,
            None => continue,
        }
        
        let user_string: String = it.collect::<Vec<&str>>().join(" ");

        //println!("{user_token}");
        //println!("{user_string}");

        match user_token{
            "add"=>todo_vector.push(user_string),
            "help"=>println!("{HELP_TEXT}"),
            "show"=>
            {
                for i in &todo_vector{
                    println!("{i}");
                }
            },
            "remove"=>
            {
                todo_vector.remove(user_string.parse::<usize>().expect("No usize?"));
            },
            "quit"=>exit(0),
            _ => continue,
        }

    }
}

