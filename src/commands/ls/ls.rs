use std::{env,fs};

use console::style;
pub fn ls() {
    let cwd = env::current_dir().unwrap();
    for file in fs::read_dir(cwd.as_path()).unwrap() {
        let file = file.unwrap();

        let is_dir: bool = file.file_type().unwrap().is_dir();
        let is_file: bool = file.file_type().unwrap().is_file();

        if is_dir  {
            println!("{}: {}",style("[Directory]").blue(),file.file_name().to_str().unwrap());
        } else if is_file {
            println!("{}: {}",style("[File]").green(),file.file_name().to_str().unwrap());
        } else if !is_dir && !is_file { // id why but it's like this for linux for some reason?
            println!("{}: {}",style("[Directory]").blue(),file.file_name().to_str().unwrap());
        }
    }

}