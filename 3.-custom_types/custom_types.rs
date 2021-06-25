use std::io;

fn main() {
    // Rust custom data types are formed mainly through the two keywords:
    //   struct: define a structure
    //   enum: define an enumeration
    // Constants can also be created via the const and static keywords.

    io::stdin().read_line(&mut String::new()).unwrap();
}
