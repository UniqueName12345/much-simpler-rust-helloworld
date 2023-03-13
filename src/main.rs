// This is an updated version of the upstream's hello world program
// We removed all the unnecessary functions and traits
// and simplified the code to make it more readable and memory safe

fn main() {
    println!("Hello, world!"); // A classic greeting
    // No need for a MsgWriter trait, a simple println! macro will do the job
    // Also, no need for a numerals.rs file, we're not dealing with numbers here!
}
// And that's it! A simple and effective hello world program.
