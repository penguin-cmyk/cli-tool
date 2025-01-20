pub fn echo(command: &Vec<&str>) {
    let length = command.len();
    if length > 1 {
        for i in 1..length {
            print!("{} ",command[i]);
        }
    }
    print!("\n");
}