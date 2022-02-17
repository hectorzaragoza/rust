#[allow(dead_code)] 
#[allow(unused_variables)]//This allows unsued variables, so you don't see the warnings
use std::mem;

// Defining constants. You can have global variables by declaring them towards the top, lifetime of whole program.
// Two way to define global variables with fixed values
const MEANING_OF_LIFE:u8 = 42; // no fixed address, the value is just copied to where its invoked.
// want a memory address for constant, use static
static mut Z:i32 = 123; // mutable variable can have various threads trying to change it so Rust protects against unsafe memory handling.

// fn operators() {
//     let mut a = 2+3*4; // Operators follow precedence (Order of Operations)
//     println!("{}", a);
//     a = a+1; // Rust does not support -- or ++ operators
//     a -= 2;
//     println!("remainder of {} / {} = {}", a, 3, (a%3));
// }

fn scope_and_shadowing() {
    let a = 123;
    // Creating an inner scope, b is not available outside of the internal scope but a does exist inside the inner scope.
    // In this example, I declared a variable with the same name in two different scopes, so it's allowed!
    {
        let b = 456;
        println!("inside, b = {}", b);

        let a = 777;
        println!("inside, a = {}", a);
    }
    println!("Outside, a ={}", a);
    // println!("Outside, b ={}", b);
}


// fn fundamental_data_types() {
//     let a: u8 = 123; // u = unsigned, 8 bits: 0 - 255 inclusive
//     println!("This is a: {}", a); // This is immutable variable
//     // Variables in rust are immutable by default
    
//     // u = unsigned, 0 to 2^N-1
//     // i = signed, -2^(N-1) .. 2^(N-1)-1; 128 to 127 
//     let mut b: i8 = 0;
//     println!("B is equal to {} before", b);
//     b = 42;
//     println!("B is equal to {} after", b);

//     let c = 123456789; // Rust compiler figures out what kind of variable type the value is. It infers i32
//     println!("C is {}, tkaes up {} bytes", c, mem::size_of_val(&c)); //mem is a librarythat shows memory stuff
//     // Make sure to explicitly make variables mutable
//     // Options: u8 - u64, and i8 - i64


//     //Two more data types: usize and isize
//     //usize = unsigned, isize = signed (im on 64 bit, so these are 64bit)
//     let z: isize = 123;
//     let size_of_z = mem::size_of_val(&z);
//     println!("Z is {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z*8);

//     let d: char = 'x'; //There is a different between a character and a letter
//     // single character,single quotes
//     println!("d is a char {} with size of {} bytes", d,mem::size_of_val(&d));

//     // Floating point numbers: f32 and f64
//     let e: f32 = 2.5;
//     println!("{}, size = {} bytes", e, mem::size_of_val(&e));


//     // Boolean
//     let g: bool = false;
//     println!("{}, size of {} bytes", g, mem::size_of_val(&g));
// }

fn main() {
    scope_and_shadowing()
}