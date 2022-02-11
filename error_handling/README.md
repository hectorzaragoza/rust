# Error Handling in Rust 2/11/22
-Handling errors could include:
    - Stopping the program immediately
    - trying a different operation to recover
    - using default values
    - returngin an error for some other programmer to handle
    - giving a nice error message to the user.

### Module 2 - Panicking when something goes wrong
- panic! macro...cause your program to panic.
- Like an emergency stop button if unexpected value is being processed that may result in a vulnerability.
- In code, you could create an enum with the accepted values, and then run an if else statement with an else statement that will run the panic! macro and include an error message for the user to suggest invalid value and provide an opportunity to correct their input.
- Error in C: buffer overread, C happily returns a memory that doesn't belong to the current data structure (out of bounds) which has been an entry point of lots of exploits.
- other macros that cause a panic!, unreachable! impossible to get to this spot.
    - in a match expression, your catchall statement _ can call the unreachable!() macro to cause a panic. Since match expressions must get a value.
- other macro: unimplemented!()
    - when writing a program, you can use this placeholder to simply state that the code simply hasnt been written for this yet.
- other macros: assert family
    - panic if a given condition isn't true.

### Module 3 - Handling results and options