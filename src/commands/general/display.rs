use console::style;
pub fn display(commands: &Vec<&str>) {
    if commands.len() <= 2 { return; }

    let commands = commands.join(" ");
    let commands = commands.split(' ').collect::<Vec<&str>>();

    let option = match commands.get(1) {
        Some(command) => command,
        None => ""
    };

    match option {
        "pointer" => {
            for string in commands[2..commands.len()].iter() { println!("{:?}",style(string.as_ptr()).bold().cyan()) }
        }
        "byte" | "bytes" => {
            for string in commands[2..commands.len()].iter() { println!("{:?}",style(string.as_bytes()).bold().cyan()) }
        }
        _ => {}
    }

}
