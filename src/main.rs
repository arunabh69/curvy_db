use std::io;
mod db;
mod curve;
mod command;

fn main() {
    loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read command");
        // Need to parse command
        if command.to_ascii_lowercase().contains("exit") ||
            command.to_ascii_lowercase().contains("quit") {
            println!("Exiting...");
            break;
        } 
        command::command::parse_user_input(command.as_str())
    }
}
