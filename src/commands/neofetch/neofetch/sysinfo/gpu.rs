use std::process::Command;


pub fn gpu() -> String {
    let gpu_info = Command::new("sh")
        .arg("-c")
        .arg("lspci | grep VGA")
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8(gpu_info.stdout).unwrap();
    let output = output.split("[").nth(2).unwrap().split("]").nth(0).unwrap().to_string();

    output
}