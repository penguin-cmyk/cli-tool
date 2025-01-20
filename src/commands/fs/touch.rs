use std::fs;
use console::style;

pub fn touch(command: &Vec<&str>) {
    let path: String = match command.get(1) {
        Some(path) => path.to_string().replace('"',""),
        _ => {println!("{}", style("  No Filename argument").bold().red()); return}
    };

    if path.is_empty() || path.len() == 0 { println!("{}", style("  Filename argument is empty").bold().red()); return };


    match fs::write(path, " ") {
        Ok(_) => println!("{}",style("󰸞 Successfully wrote file").bold().green()),
        Err(e) => {eprintln!("{}: {}",style("  Error occurred when trying to write file: ").bold().red(),e); return }
    };
}