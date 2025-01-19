use std::fs::File;
use std::io::{BufReader, BufRead};

use console::style;
fn parse_memory(parts: Vec<&str>) -> u64 {
    let mut mem: u64 = 0;
    if let Some(mem_str) = parts.get(1) { // 1 = size ; 2 = type
        mem = match mem_str.parse::<u64>() {
            Ok(memory) => memory / 1024, // turn it from kb to mb
            Err(_) => 0,
        } // turn it into an unsigned 64-bit integer
    }

    mem
}

pub fn memory() -> String {
    let memory:File = File::open("/proc/meminfo").unwrap();
    let memory: BufReader<File> = BufReader::new(memory);

    let mut max_memory: u64 = 0;
    let mut free_memory: u64 = 0;

    for line in memory.lines() {
        let line = line.unwrap();
        if line.starts_with("MemTotal:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            max_memory = parse_memory(parts);
        }
        else if line.starts_with("MemAvailable:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            free_memory = parse_memory(parts);
        }
    }

    let used_mem: u64 = max_memory - free_memory;
    format!("{}: {used_mem}MiB / {max_memory}MiB",style("Memory").bold().cyan())
}