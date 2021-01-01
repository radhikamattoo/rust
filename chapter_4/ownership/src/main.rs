/* Ownership */
// Memory managed through ownership rules that the compiler checks
// Stack vs. Heap 
    // All data stored on STACK must have a known, fixed size at compile time 
        // Very fast retrieval/allocation because data used together is close together physically on stack
    // All other data allocated onto HEAP
        // Less organized, request a certain amount of space that may not be entirely used during runtime
        // Slower to access because have to follow pointer
            // Data used together may be far away from each other in the heap
            // Allocation may take time
        // ALLOCATING: find empty spot in heap big enough, marks it as being in use, and return a POINTER
            // Pointer is the address of that location on the heap (known size, pushed onto stack)

// Ownership Rules
/*
1. Each value in Rust has a variable that is its OWNER
2. Only one owner at a time
3. When the owner goes out of scope, value is dropped (i.e. can be garbage collected)
*/

// Memory and Allocation
    // To allocate memory on heap (for variables with unknown size at compile time):
        // Request memory from memory allocator at runtime 
        // Return this memory to the allocator once the variable goes out of SCOPE
    // When variable goes out of scope (closing curly bracket) Rust calls DROP function 

    // How heap variables interact:
// -------------------------------------------------------------------------    
// fn main() {
//     // String stored in heap, metadata stored in stack
//     let s1 = String::from("hello"); 
    
//     // Stack metadata copied, points to same heap memory as s1
//     // s1 is INVALIDATED, can only use s2 from here on (removes any DOUBLE FREE errors)
//     // Seems like a shallow copy, but because of invalidation is called a MOVE
//         // s1 was MOVED into s2
//     let s2 = s1; 
//     // println!("{}, world!", s1); // throws compiler error
//     println!("{}, world!", s2); // prints hello world!
// } 
// -------------------------------------------------------------------------
