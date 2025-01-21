// this was made for linux only maybe windows support soon
mod sysinfo;
use console::style;

pub fn fetch() -> String {
    let memory: String = sysinfo::memory::memory();
    let cpu: String = sysinfo::cpu::get_cpu_info();
    let os: Vec<String> = sysinfo::os::os();
    let gpu: String = sysinfo::gpu::gpu(); // idek if this work for every gpu

    let mut stripes: String = String::new();

    for _ in 0..os.get(0).unwrap().len() / 2 {
        stripes.push('-');
    }


    let output = format!("    {}\n{}\n{memory}\n{cpu}\n{}\n{}\n{}{gpu}",
                          os.get(0).unwrap(),
                          style(stripes).bold(),
                          os.get(1).unwrap(),
                          os.get(2).unwrap(),
                          style("GPU: ").bold().cyan()
    );

    output
}