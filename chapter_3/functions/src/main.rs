/* Functions */

// Basics
// -------------------------------------------------------------------------
// fn main() {
//     println!("Hello, world!");

//     another_function();
// }

// // Could have defined this before main too
// // Order of function declaration doesn't matter
// fn another_function() {
//     println!("Another function.");
// }
// -------------------------------------------------------------------------

// Parameters
// -------------------------------------------------------------------------
// fn main() {
//     another_function(5, 6);
// }

// // Must declare parameter types
// fn another_function(x: f32, y: u64) {
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
// }
// -------------------------------------------------------------------------

// Function bodies are made up of a series of statements and optionally ending in an expression
// STATEMENTS - instructions that perform some action and don't return a value
    // let x = 5;
// EXPRESSIONS - evaluate to a resulting value, most of the code written in Rust
    // Do not include ending semicolons, adding the semicolon makes it a statement
    // 5 + 6 is an expression that evaluates to the value 11
    // Can be part of statements 
        // let y = 6; // 6 is the expression that evaluates to the value 6
    // invoking a function is an expression
    // calling a macro is an expression
    // new scopes (i.e. {}) is an expression 
    // NO RETURN KEYWORD, MAKE IT AN EXPRESSION SO IT CAN RETURN VALUE 

// -------------------------------------------------------------------------
// fn main() {
//     let y = 6; // statement that includes an expression
    
//     // compiler error, because inner statement doesn't return a value to assign x to
//     // Unlike other languages, where you can do x = y = 6
//     // let x = (let y = 6); 
// }


// fn main() {
//     let x = 5;

//     let y = 
//     // Expression that evaluates to 4
//     {
//         let x = 3;
//         x + 1 // No semicolon (to let it return a value)!
//     };

//     println!("The value of y is: {}", y);
// }
// -------------------------------------------------------------------------

// Return Values
// declare return type with -> before opening curly brace {
// -------------------------------------------------------------------------
// fn five() -> f32 {
//     5 // no semicolon so its an expression whose value is returned
// }

// fn main() {
//     let x = five();

//     println!("The value of x is: {}", x);
// }
// -------------------------------------------------------------------------


// Control Flow
// -------------------------------------------------------------------------
// fn main() {

//     // If statements
//     let number = 7;
//     if number < 5 {
//         println!("1st condition was true");
//     } else if number < 7 {
//         println!("2nd condition was true");
//     } else {
//         println!("3rd condition was true");
//     }

//     let number = true;
//     // let number = 3; // compiler error, expected bool
//     if number { // HAS to be Boolean type, can't replace with 0 or 1 like in Python
//         println!("number is true");
//     }

//     let condition = true;
//     let number = if condition { 5 } else { 6 }; // TERNARY OPERATOR
//     // let number = if condition { 5 } else { "six" }; // compiler error, must return same type
//     println!("The value of number is {}", number);

//     // Loops
//     let mut counter = 0;
//     loop {
//         counter += 1;
//         println!("again!");
//         if counter == 2 {
//             break
//         }
//     }
//     counter = 0;
//     let result = loop {
//         counter += 1;
//         if counter == 10 {
//             // Value after break inside a loop will return that value from the expression
//             break counter * 2;
//         }
//     }; // Have to have semicolon here because its a variable declaration, so really an statement
//     println!("Result is {}", result);

    
//     // While Loops
//     let mut number = 3;
//     while number != 0 {
//         println!("{}!", number);
//         number -= 1;
//     }
//     println!("Liftoff!");

//     // For Loops (most commonly used)
//     let a = [10, 20, 30];

//     for element in a.iter(){
//         println!("The value is {}", element);
//     }

//     for number in (1..4).rev() {
//         println!("{}!", number);
//     }
//     println!("Liftoff!");
// }
// -------------------------------------------------------------------------


/* Practice Problems */

// Problem 1: Write 2 functions that converts temperatures b/w Fahrenheit and Celsius
// ----------------------------------------------------------------------------------
// use std::io;
// fn convert_to_celsius(f_temp: f32) -> f32 {
//     (f_temp - 32.0) * 0.555
// }

// fn convert_to_fahrenheit(c_temp: f32) -> f32 {
//     (c_temp * 1.8) + 32.0
// }

// fn main(){
//     let temp_type = loop {
//         println!("Is your input temp Fahrenheit or Celsius? [F/C]: ");
//         let mut type_ = String::new();
//         io::stdin().read_line(&mut type_).expect("Failed to read line");
//         type_ = type_.trim().to_string();
        
//         if type_ != "F" && type_ != "C" {
//             continue;
//         }
//         break type_
//     };
    
//     let temperature :f32 = loop {
//         println!("Please enter the temperature in {} to convert", temp_type);
//         let mut temp = String::new();
//         io::stdin().read_line(&mut temp).expect("Failed to read line");
//         let temp :f32 = match temp.trim().parse(){
//             Ok(num) => num,
//             Err(_) => {
//                 continue;
//             }
//         };
//         break temp
//     };

//     if temp_type == "F"{
//         let converted = convert_to_celsius(temperature);
//         println!("{}F is {}C", temperature, converted);
//     }else{
//         let converted = convert_to_fahrenheit(temperature);
//         println!("{}C is {}F", temperature, converted);
//     }
// }
// ----------------------------------------------------------------------------------

// Problem 2: Generate the nth Fibonacci number
// ----------------------------------------------------------------------------------
// use std::io;
// use std::io::Write; // for flushing print! output
// // f(n) = f(n-1) + f(n-2)
// // f(0) = 0
// // f(1) = 1
// // TODO/ FIXME: Optimize this solution using MEMOIZATION (i.e. closures)
//     // Optimization technique via storing the results of expensive function calls and returning the cached result when the same inputs occur again
// fn generate_fibonacci(n :u64) -> u64 {
//     if n < 2 {
//         n
//     }else{
//         generate_fibonacci(n - 1) + generate_fibonacci(n - 2)
//     }
// }

// fn main(){
//     loop {
//         let mut input = String::new();
//         // stdout is frequently line-buffered by default, so may need to flush buffer to emit output immediately
//         print!("Please enter an integer N to generate the Nth Fibonacci Number: ");
//         io::stdout().flush().unwrap();
//         io::stdin().read_line(&mut input).expect("Failed to read line");
//         let input :u64 = match input.trim().parse() {
//             Ok(n) => n,
//             Err(_) => {
//                 println!("Please enter a number!");
//                 continue;
//             }
//         };
//         let fibonacci_number = generate_fibonacci(input); // regular recursive function
//         println!("The {}(th/nd) Fibonacci number is {}", input, fibonacci_number);
//     }
// }
// ----------------------------------------------------------------------------------


// Problem 3: Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
// ----------------------------------------------------------------------------------
// use std::io;
// use std::thread;
// use std::io::Write; 

// fn main(){
//     let days_of_xmas = [
//         "a partridge in a pear tree",
//         "two turtle doves",
//         "three french hens",
//         "four calling birds",
//         "five golden rings",
//         "six geese a-laying",
//         "seven swans a-swimming",
//         "eight maids a-milking",
//         "nine ladies dancing",
//         "ten lords a-leaping",
//         "eleven pipers piping",
//         "twelve drummers drumming"
//     ];

//     for (day, present) in days_of_xmas.iter().enumerate(){
//         print!("On the ");
//         if day == 0 {
//             print!("1st");
//         } else if day == 1 {
//             print!("2nd");
//         } else if day == 2 {
//             print!("3rd");
//         } else {
//             let middleman = (day + 1).to_string();
//             print!("{}th", middleman);
//         }

//         print!(" day of christmas my true love gave to me, ");
//         io::stdout().flush().unwrap(); // dump content to stdout and pause
//         thread::sleep_ms(3000);
//         println!("{}!", present); // then print the present
//         thread::sleep_ms(4000);
//     }
// }
// ----------------------------------------------------------------------------------