// Declaring rand as an external crate
extern crate rand;

// io library
use std::io;
// Ordering - Greater/Lesser/Equal
use std::cmp::Ordering;
// Random Generator trait
use rand::Rng;

fn main() {
    // Usage of the println!() macro
    let mut target: u8;
    let mut guess: String;
    let endl = "------------------------------------";
    loop {
    println!("Guess the number (1, 10):");
    // Use thread rng to generate a u8 in the I[1, 10]
    target = rand::thread_rng().gen_range(1..10);
    guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read input");
   
    let guess: u8 = guess.trim().parse::<u8>()
        .expect("Please type a number");

    match guess.cmp(&target) {
        Ordering::Less => println!("Too small!\n{}", endl),
        Ordering::Greater => println!("Too big!\n{}", endl),
        Ordering::Equal => break,
    }
    }
    println!("Congragulations!");
    println!("Your Guess: {}Actual: {}", guess, target);
}
