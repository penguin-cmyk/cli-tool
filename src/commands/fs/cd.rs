use std::env;
use std::path::Path;
use std::fs;

use console::style;
fn exists(path: &String) -> bool {
    fs::metadata(path).is_ok()
}
pub fn cd(new_path: &Vec<&str>) {
    let new_path = match new_path.get(1) {
        Some(path) => path,
        None => { println!("{}",style("Didn't pass in any arguments").red().bold()); return },
    };

    if new_path.is_empty() { println!("{}",style("Nothing to cd").red().bold()); return }
    if !exists(&new_path.to_string()) { println!("{}",style("Path doesn't exist").red().bold()); return }

    let new_path = Path::new(&new_path);

    match env::set_current_dir(&new_path) {
        Ok(_) => {},
        Err(_) => { println!("{}",style("Could not change directory").red().bold()); }
    };
}
