use std::io;
use std::io::Write;
use std::env;

// TODO: Add touch,

use console::style;
#[path = "commands/cd/cd.rs"]
mod cd;
#[path = "commands/ls/ls.rs"]
mod ls;
#[path = "commands/neofetch/neofetch.rs"]
mod neofetch;
#[path = "commands/echo/echo.rs"]
mod echo;
#[path = "commands/general/general.rs"]
mod general;
fn main() {
    print!("\x1B[2J\x1B[1;1H");
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

        let input: String = input
            .to_lowercase()
            .trim()
            .to_string();
        let command: Vec<&str> = input
            .split(' ')
            .collect();

        let first: &str = command[0];

        match first {
            "neofetch" => { println!("{}", neofetch::fetch()) }
            "ls" => { ls::ls() }
            "cd" => { cd::cd(&command) }
            "exit" => { break }
            "echo" => { echo::echo(&command) }
            "clear" | "cls" => { general::clear() }
            "help" => { general::help() }
            "version" => { general::version() }
            _ => { println!("{}",style("Unknown command").red().bold()) }
        }
    }
}
