use std::io;
use std::io::Write;
use console::style;
#[path= "commands/neofetch/neofetch.rs"]
mod neofetch;

fn main() {
    loop {
        print!("{}",style("Please enter a command: ").bold().blue());
        let _ = io::stdout().flush();
        let mut input = String::new();

        io::stdin().
            read_line(&mut input)
            .unwrap();

        let command = input.to_lowercase().trim().to_string();
        if command == "neofetch" {
            println!("{}", neofetch::fetch());
        } else {
            println!("{}", style(format!("[Invalid Command] {}",command)).red().bold());
        }
    }
}