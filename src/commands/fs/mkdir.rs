use console::style;

use std::fs;
use std::path::Path;
fn error(message: &str) { println!("{}", style(format!(" {}",message)).bold().red()); }
pub fn mkdir(command: &Vec<&str>) {
    if command.len() <= 1 { return;}

    let dir = match command.get(1) {
        Some(dir) => dir.to_string(),
        _ => {
            error("Error when converting / getting the directory name");
            return
        }
    };

    if Path::new(&dir).exists() {
        error("Directory already exists");
        return
    }
    match fs::create_dir(&dir) {
        Ok(_) => println!("{}",style("󰸞 Successfully made the directory").bold().green()),
        Err(e) => {
            eprintln!("Error {e}");
            return
        },
    }

}