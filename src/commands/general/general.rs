use console::style;

fn add_command(commands: &mut Vec<String>,command_name:&str, command_info: &str) {
    let command = format!("[{}]: {}",style(command_name).blue(),command_info);
    commands.push(command);
}
pub fn version() {
    let version: &str = "1.0";
    println!("Running on version: {}", style(version).green());
}
pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}
pub fn help() {
    let mut commands: Vec<String> = Vec::new();
    add_command(&mut commands,"clear / cls","clears the screen");
    add_command(&mut commands,"neofetch","displays a copy of neofetch");
    add_command(&mut commands,"version","displays the version of this program");
    add_command(&mut commands,"help","display this help message");
    add_command(&mut commands,"exit","closes this program");
    add_command(&mut commands,"ls","lists all files in the current directory");
    add_command(&mut commands,"cd","changes to a directory specified in the command");

    for command in commands {
        println!("{}", command);
    }

}