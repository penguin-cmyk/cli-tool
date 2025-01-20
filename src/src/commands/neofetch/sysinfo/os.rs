use std::fs::File;
use std::io::{BufReader, BufRead, Read};

use console::style;

fn get(name: &str) -> String{
    let name = name.to_string();
    let out = (BufReader::new(File::open(format!("/proc/sys/kernel/{}",name)).unwrap()))
        .lines()
        .nth(0)
        .unwrap()
        .unwrap_or(String::from("unknown"));
    out
}

pub fn os() -> Vec<String>  {
    let mut out: Vec<String> = Vec::new();

    let os: String = get("hostname");

    let mut env = BufReader::new(File::open("/proc/self/environ").expect("Could not open /proc/self/environ | File prb doesn't exist"));

    let mut buffer = Vec::new();
    let _ = env.read_to_end(&mut buffer).expect("Could not read /proc/self/environ");

    let mut os_name: String = String::new();
    let mut desktop: String = String::new();
    let mut shell: String = String::new();

    for var in buffer.split(|&byte| byte== 0) {
        if let Ok(str) = String::from_utf8(var.to_vec()){
            if str.starts_with("USER=")  {
                os_name = format!("{}@{}", style(str.trim_start_matches("USER=").to_string()).bold().cyan(),style(&os).bold().cyan());
            } else if str.starts_with("DESKTOP_SESSION=") {
                desktop = format!("{}: {}",style("DE").bold().cyan(),str.trim_start_matches("DESKTOP_SESSION=").to_string());
            } else if str.starts_with("SHELL=")  {
                shell = format!("{}: {}",style("Shell").bold().cyan(),str.trim_start_matches("SHELL=").to_string());
            }
        }
    }

    out.push(os_name);
    out.push(desktop);
    out.push(shell);

    out


}