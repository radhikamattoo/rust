// Import input/output library from the standard (std) library
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess:");

    /*
    VARIABLES
    * "let" statement used to create a variable
        let foo = bar; // create new var and binds it to the value of bar
    * Variables are immutable by default
    * Must use the "mut" keyword to make a variable mutable
        let foo = 5; // immutable
        let mut bar = 5; // mutable
    * :: indicates new is a static method (impelmented on type rather than instance)
    */
    let mut guess = String::new();

    // without the use/import at the top, the full call would be std::io::stdin
    io::stdin()
        // & is a REFERENCE (immutable by default)
        // that's why we add "mut" to this, to allow the read_line function to mutate the reference
        .read_line(&mut guess)
        // read_line returns an io::Result, an ENUM
        // Enum is a type that can have a fixed set of values, called VARIANTS
        // Result's variants are "Ok" and "Err"
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
