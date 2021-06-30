fn main() {
    // Create a std::String::String from a
    let mut a: String = String::from("hello");
    // Perform a mutable borrow on a, transferring ownership to b
    let b: &mut String = &mut a;
    b.push('!');
    println!("b = {}", b);
    // b goes out of scope here on line 8, drop() is called. a regains ownership of memory
    println!("a = {}", a);
}
