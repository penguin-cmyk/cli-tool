use std::fs::File;
use std::io::{BufReader,BufRead};

use console::style;

fn info_append(line: &String) -> String {
    let mut out = String::new();
    if let Some(name) = line.split(":").nth(1) {
        out = name.trim_start().to_string();
    }
    out
}
pub fn get_cpu_info() -> String{
    let cpu_info: File = File::open("/proc/cpuinfo").unwrap();
    let reader: BufReader<File> = BufReader::new(cpu_info);

    let mut model_name = String::new();
    let mut cores = String::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("model name") {
            model_name = info_append(&line);
        } else if line.starts_with("siblings") {
            cores = info_append(&line);
        }
    }

    format!("{}: {model_name} ({})",style("CPU").bold().cyan(),style(cores).bold())

}