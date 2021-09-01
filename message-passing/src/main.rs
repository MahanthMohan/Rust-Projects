use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, reciever) = mpsc::channel(); 
    let messages = vec!["foo", "bar", "baz"];
    thread::spawn(move || {
        for msg in messages {
            sender.send(msg).unwrap();
        }
    });

    for msg in reciever.into_iter() {
        println!("{}", msg);
    }
}
