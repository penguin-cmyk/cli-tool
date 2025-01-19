use std::io;
use std::io::Write;
use std::env;

use console::style;
#[path = "commands/cd/cd.rs"]
mod cd;
#[path = "commands/ls/ls.rs"]
mod ls;
#[path = "commands/neofetch/neofetch.rs"]
mod neofetch;

fn main() {
    loop {
        print!("{}[{}]: ", style("Please enter a command ").bold().blue(),env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string());
        let _ = io::stdout().flush();
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .unwrap();

        let trimmed_input = input.to_lowercase().trim().to_string();
        let command: Vec<&str> = trimmed_input.split(' ').collect();
        let first_arg = command[0];

        match first_arg {
            "neofetch" => { println!("{}", neofetch::fetch()) }
            "ls" => { ls::ls() }
            "cd" => { cd::cd(&command) }
            "exit" => { break }
            _ => { println!("{}",style("Unknown command").red().bold()) }
        }
    }
}
