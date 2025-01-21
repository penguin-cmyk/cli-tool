use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use console::style;

pub fn cat(command: &Vec<&str>) {
    if command.len() <= 0 { return; }

    let path = command[1];
    if Path::new(&path).is_dir() { println!("{}", style("  Path is a directory").bold().red()); return; }

    let file: File = match File::open(&path) {
        Ok(file) => file,
        Err(e) => { eprintln!("Error opening file: {}\n {}", &command[1],e); return; }
    };
    let reader = BufReader::new(file);
    let contents = reader.lines();

    for line in contents {
        let _ = match line {
            Ok(line) => println!("{}", line),
            Err(_) => { continue }
        };
    }
}