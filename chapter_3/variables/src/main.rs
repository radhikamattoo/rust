/* Variables and Mutability */
fn main() {
    /* Default Immutability */
    // Compiler error, variables are immutable by default
    // Also indicates to code readers that the value of this var will not change
    // -------------------------------------------------------------------------
    // let x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);
    // -------------------------------------------------------------------------

    // MUT keyword allows you to change the value of a variable
    // Also indicates to code readers that the value of this var can change
    // -------------------------------------------------------------------------
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);
    // -------------------------------------------------------------------------



    /* Constants */
    //  Declared with const keyword, must have type too
    // -------------------------------------------------------------------------
    // const MAX_POINTS: u32 = 100_000;
    // -------------------------------------------------------------------------



    /* Shadowing */
    // Can REDECLARE a variable with the same name 
    // -------------------------------------------------------------------------
    // let x = 5;
    // let x = x + 1; // x = 6
    // let x = x * 2; // x = 12
    // println!("The value of x is: {}", x); // x = 12
    // -------------------------------------------------------------------------

    // Can't do the above with a mut variable (compiler error)
    // let over mut:
        // Can perform transformations on a value but have the variable be immutable afterwards
        // Creating a new variable with let, so can change the type
    // -------------------------------------------------------------------------
    // // OK
    // let spaces = "   ";
    // let spaces = spaces.len(); 
    
    // // COMPILER ERROR
    // let mut spaces = "   ";
    // spaces = spaces.len();
    // -------------------------------------------------------------------------




    /* Data Types */
    // Statically typed language, so types of variables must be known at compile time
        // Compiler can INFER types based on value and how the variable is used
        // Case where multiple types are possible, must add a TYPE ANNOTATION
    // -------------------------------------------------------------------------            
    // let guess: u32 = "42".parse().expect("Not a number!");
    // -------------------------------------------------------------------------

    // SCALAR type represents a single value
        // INTEGER (default is i32)
            // Unsigned (u prefix) vs, signed (i prefix)
            // 8-128 and arch bit length options (i8 = signed 8bit, u64 = unsigned 64 bit, isize = signed arch bit)
                // arch means based on your computer architecture (32-bit or 64-bit architecture)
            // Integer Overflow
                //  Debug mode Rust compiler checks for overflow
                // Compiling with --release flag (i.e. for production), Rust performs two's complement and wraps the integer around 
        // FLOAT (default is f64)
        // BOOLEAN (one byte, easily inferred)
        // CHARACTER (single quotes, single character)
            // DOUBLE AND SINGLE QUOTES NOT INTERCHANGEABLE


    // COMPOUND type can group multiple values into one type
        // TUPLE (fixed length, use pattern matching or dot notation to extract values
    // -------------------------------------------------------------------------
    // let tup = (500, 6.4, 1);
    // let (x, y, z) = tup; // pattern matching
    // let last = tup.2; // dot notation
    // println!("The value of y is: {} and last is {}", y, last); // 6.4 and 1
    // -------------------------------------------------------------------------
        // ARRAY (fixed length, same type)
            // For when you data allocated on stack instead of heap
    // -------------------------------------------------------------------------            
    // // Will compile & run
    // let a = [3; 5]; // same as writing let a = [3, 3, 3, 3, 3]; [value; length];
    // let index = 0;
    // println!("element is {}",  a[index]); // 3

    // // Will compile & run but panic with index out of bounds error
    // // Shadowing side effect?
    // let index = 5;
    // println!("A[{}] is {}", index, a[index]); 

    // // Compiler error, index out of bounds
    // let element = a[var];
    // println!("element is {}", a[var]); 
    // -------------------------------------------------------------------------
        // FUNCTIONS (go to functions project)
            













        
            
            


    

    

    

}
