use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];

    match command.as_str() {
        "help" => println!("This will print help"),
        "config" => println!("This will setup config"),
        _ => println!("This will print error message"),
    }
}
