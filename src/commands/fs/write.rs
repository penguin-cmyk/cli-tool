use std::fs;
use console::style;
pub fn write(command: &mut Vec<&str>) {
    let path = match command.get(1) {
        Some(path) => path.to_string().replace('"',""),
        _ => {println!("{}", style("  No path argument").bold().red()); return}
    };

    if path.is_empty() || path.len() == 0 { println!("{}", style("  Path argument is empty").bold().red()); return };
    if command.len() <= 2 {println!("{}",style("  Not enough arguments to write in the file").bold().red()); return };

    let command = command[2..].join(" ");
    let content = command.replace('"',"");

    if content.is_empty() || content.len() == 0 { return }

    match fs::write(path, content) {
        Ok(_) => println!("{}",style("󰸞 Successfully wrote file").bold().green()),
        Err(e) => {eprintln!("{}: {}",style("  Error occured when trying to write file: ").bold().red(),e); return }
    };
}