// Rust Tutorial

/*
RUNNING CODE
1. rustc tutorial.rs // Rust compiler that creates a binary executable
2. ./tutorial  // Execute the binary
* Rust is an ahead-of-time compiled language
    * You can compile a program and give the executable to someone else, and they can run it even without having Rust installed
*/
/*
CARGO
* Cargo is Rust's build system and package manager
* Handles building code, downloading dependencues, and building those dependencies
* Can create a new project with "cargo new <project_name>", which creates
    * <project_name> dir
    * <project_name>/Cargo.toml  // Like a package.json
    * <project_name>/src/main.rs
    * Initializes a git repo
* Build & run the code using "cargo run"
* "cargo check" checks that your code compiles without having to build the executable
* "cargo build --release" to compile with optimizations
*/

// main function is special, always first code that runs in Rust executables
fn main() {
    // println! calls a Rust macro
    // The ! means its invoking a macro
    // A call like this without ! is a function call
    println!("Hello, world!");
}
