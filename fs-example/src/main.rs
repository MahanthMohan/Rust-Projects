use std::env;
use std::io::prelude::*;
use std::cmp::Ordering;
use std::fs;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();

    match (args.len() as u8).cmp(&1) {
        Ordering::Greater => {
            match args[1].as_str() {
                "-r" | "--read" => {
                    let mut file = fs::File::open(&args[2]).expect("Error opening file for read");
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).expect("Error reading file");
                    println!("--- File contents ---\n{}", contents);
                },
                "-w" | "--write" => {
                    let mut data = String::new();
                    println!("Enter file contents:");
                    io::stdin().read_line(&mut data).unwrap();
                    fs::write(&args[2], data).expect("Error writing file");
                    println!("Successfully written data to file");
                },
                "-d"| "--delete" | "-rm" | "--remove" => {
                    fs::remove_file(&args[2]).expect("Error deleting file");
                    println!("Successfully deleted file");
                }
                "-md" | "--mkdir" | "-cdir" | "--create-dir" => {
                    for i in 2..args.len() {
                        fs::create_dir(&args[i]).expect("Error creating directory");   
                    }
                    println!("Successfully created directories");
                }
                "-rd" | "--rmdir" | "--delete-dir" => {
                    for i in 2..args.len() {
                        fs::remove_dir(&args[i]).expect("Error removing directory");
                    }
                    println!("Successfully removed directories");
                }
                "-h" | "-u" | "--help" | "--usage" => {
                    println!("--- Usage Info ---");
                    println!("fs-example <flag> <filename>");
                    println!("List of Commands:");
                    println!("-r | --read => Read File");
                    println!("-w | --write => Write File (Create => no file content args)");
                    println!("-d | --delete | -rm | --remove => Delete File");
                    println!("-md | --mkdir | -cdir | --create-dir => Create Directory");
                    println!("-h | -u | --help | --usage => Help/Usage Info");
                    println!("Anything else => Invalid Argument");
                },
                _ => {
                    eprintln!("Invalid argument");
                }
            }
        },
        
        Ordering::Equal | Ordering::Less => {
            eprintln!("Few/No arguments provided!");
        }

    }

}
