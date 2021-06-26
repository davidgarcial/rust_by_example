use std::io;

fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");
    
    io::stdin().read_line(&mut String::new()).unwrap();
}
