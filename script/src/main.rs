use std::process::Command;
use std::cmp::Ordering;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("--- Opening Explorer ---");
    match (args.len() as u8).cmp(&1) {
        Ordering::Greater => {
        Command::new("explorer.exe")
            .arg(&args[1])
            .status()
            .expect("Error while running command");

        },
        _ => {
        Command::new("explorer.exe")
            .arg(".")
            .status()
            .expect("Error while running command");
        }
    }
}
