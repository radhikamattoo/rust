// Import input/output library from the standard (std) library
use std::io;
// After running "cargo build" with added rand dependency, can import this library
// Rng trait defines methods that random number generators implement and has to be in scope
// Eun the "cargo doc --open" command to build docs on your dependencies and open it locally in browser
use rand::Rng;
// Ordering is an enum, and the variants are Less, Greater, and Equal
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // thread_rng gives us a random number generator
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is {}", secret_number);

    // loop keyword creates an infinite loop
    loop {
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
            // io::Result's variants are "Ok" and "Err"
            // Result types encode error-handling info
            // If io::Result is an Err value, .expect will cause the program to crash and display the message
            // If io::Result is an Ok value, .expect will take the value of Ok and return it so you can use it
            // In this case, it's what the user entered on stdin
            .expect("Failed to read line");

        // Need to convert guess to an integer so the compiler doesn't throw a type mismatch error in the cmp function below (because secret_number is an int!)
        // Rust allows us to SHADOW the previous value of guess with a new one
        // Most often used when converting a value from one type to another
        // .trim() removes newline from read_line
        // .parse() method parses a string into a number
        // Need to tell Rust to convert it to a u32 int type (since there are several number types)
        // Below, when comparing guess with secret_number, Rust will infer that secret_number should be a u32 as well!
        // .parse() can easily thwo an error, so we handle the possible Result variant Err using an .expect() method

        // Switching from an .expect() call to a match expression is generally how you move from crashing on an error to handling the error
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse(){
            // Just return the num value that parse produced and put inside the Ok value
            Ok(num) => num,
            // Underscore is a catchall value, so will match this pattern no matter what value is inside Err
            Err(_) => {
                println!("Please enter a number!");
                continue; // Continue on to the next iteration of the loop
            }, 
        };

        println!("You guessed: {}", guess);

        // cmp compare two values that can be compared (takes a reference to the comparer)
        // cmp returns a variant of the Ordering enum
        // MATCH expression decides what to do next based on the variant result from cmp
        // MATCH expression is made up of ARMS
            // ARM consists of a PATTERN and the code that should be run if the result from cmp fits the arm's pattern
            // Think of it as a fancy "switch-case" clause
            // ORDER OF THE ARMS MATTERS! 
        match guess.cmp(&secret_number) {
            // ::Less, ::Greater, ::Equal are all VARIANTS from the Ordering ENUM
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            // We want the program to quit/exit when the correct value is guessed (so the loop doesn't go on forever)
            // Ordering::Equal => println!("You win!"),
            Ordering::Equal =>{
                println!("You win!");
                break;
            }
        }
    }
}
