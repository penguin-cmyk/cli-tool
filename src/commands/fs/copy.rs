use std::fs;
use std::path::Path;
use console::style;
pub fn copy(command: &Vec<&str>) {
    if command.len() <= 2 { println!("{}",style("  Not enough arguments").bold().red()); return;}
    let file_name = match command.get(1) { Some(s) => s.to_string(), None => {println!("{}",style("  Not enough arguments").bold().red());return}, };
    let destionation =  match command.get(2) { Some(s) => s.to_string(), None => {println!("{}", style("  Not enough arguments").bold().red());return}, };

    let path = Path::new(&destionation);

    if !Path::new(&file_name).exists() { println!("{}",style("  The first file does not exist").bold().red()); return; }
    if !path.exists() { println!("{}",style("  The second file / directory does not exist").bold().red()) ; return; }

    if path.is_dir() {
        match fs::copy(&file_name, &format!("{}/{}", destionation, &file_name)) {
            Ok(_) => println!("{}",style("󰸞 Successfully copied file").bold().green()),
            Err(e) => eprintln!("{} copy error: {}", destionation, e),
        };
    }
    else if path.is_file() {
        match fs::copy(&file_name, &destionation) {
            Ok(_) => println!("{}",style("󰸞 Successfully copied file").bold().green()),
            Err(e) => eprintln!("{} copy error: {}", destionation, e),
        };
    } 

}
