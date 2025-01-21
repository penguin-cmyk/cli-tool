use std::io::{stdout,stdin,Write};
use std::process::exit;
use std::env;

use console::style;

mod commands;
use crate::commands::fs::{cat,cd,copy,ls,touch,write};
use crate::commands::general::{display,general};

use crate::commands::echo::echo::echo;
use crate::commands::neofetch::neofetch::fetch;

/* TODO: add
    mkdir,
    time,
    kill [pid_id] | [process_name],
    pidof [process_name],
    processes, (returns all processes with: name | pid)
    arrow key support (for e.g changing cursor position, going up to the last command, etc)
*/

fn register(command_name: &str, mut command: &mut Vec<&str>, history: &Vec<String>) {
    match command_name {
        "neofetch" => { println!("{}", fetch()) }
        "ls" => { ls::ls() }
        "cd" => { cd::cd(&command) }
        "exit" => { exit(0) }
        "echo" => { echo(&command) }
        // General stuff
        "clear" | "cls" => { print!("\x1B[2J\x1B[1;1H"); }
        "help" => { general::help() }
        "version" | "ver" => { general::version() }
        "display" => { display::display(&command)} // serves no real use just for fun
        "history" | "hy" => {
            for command in history {
                println!("{}",style(command).yellow())
            }
        }
        // FileSystem
        "cat" => { cat::cat(&command) }
        "write" => { write::write(&mut command)}
        "touch" => { touch::touch(&command)}
        "copy" | "cp" => { copy::copy(&command)}
        _ => { println!("{} {command_name}",style("  Unknown command:").red().bold()) }
    }
}

fn main() {
    let mut history: Vec<String> = Vec::new();
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

        let _ = stdout().flush();
        let mut input = String::new();

        match stdin().read_line(&mut input) { Ok(_) => {} Err(_) => continue }

        let command = input.trim().to_string();
        let mut command: Vec<&str> = command.split(' ' ).collect();

        let first: &str = command[0];
        register(first,&mut command,&history);
        history.push(first.to_string());

        print!("\n");
    }
}
