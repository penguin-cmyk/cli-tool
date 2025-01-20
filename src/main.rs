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
#[path = "commands/echo/echo.rs"]
mod echo;
#[path = "commands/general/general.rs"]
mod general;
// Filesystem
#[path = "commands/fs/write.rs"]
mod write;
#[path = "commands/fs/cat.rs"]
mod cat;
#[path = "commands/fs/touch.rs"]
mod touch;
fn main() {
    print!("\x1B[2J\x1B[1;1H");
    loop {
        print!("{} \n[{}]{} ", style("[     Version: 1.1 ]").bold().blue(),env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(), style(" ❯")
            .bold()
            .green()
        );

        let _ = io::stdout().flush();
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => continue
        }

        let input: String = input
            .trim()
            .to_string();

        let mut command: Vec<&str> = input
            .split(' ')
            .collect();

        let first: &str = command[0];
        match first {
            "neofetch" => { println!("{}", neofetch::fetch()) }
            "ls" => { ls::ls() }
            "cd" => { cd::cd(&command) }
            "exit" => { break }
            "echo" => { echo::echo(&command) }
            // General stuff
            "clear" | "cls" => { general::clear() }
            "help" => { general::help() }
            "version" | "ver" => { general::version() }
            // FileSystem
            "cat" => { cat::cat(&command) }
            "write" => { write::write(&mut command)}
            "touch" => { touch::touch(&command)}
            _ => { println!("{} {first}",style("  Unknown command:").red().bold()) }
        }
        print!("\n");
    }
}
