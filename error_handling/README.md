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
- Result and Option types
- What do you do when you get one of these types returned from a function or method call to either use a successfully produced value or recover in the failure case.
- Result, enum defined by the std library.
    - Result has two variants: Ok and Err. One for success, one for failure.
- Option
    - Also has two variants: Some or None, have something or having nothing.
- What to do with OK or Some? You need to get the value out of the success variant to be able to use it. You can store the value in a variable through a match expression.
- With a failure case, you can use match expression to attempt to recover by taking an action like using a default value or retrying.

### Returning Result and using Question Mark
- How to return Result values from your own functions that might fail.
- How to use the ? operator within functions that return Result to propagate errors in a concise way.
- You can write a function that return the type Result and then account for success and failure case by specifying the types within angle brackets <><>
- The pattern of extractive a success value and propagating an Err value is so common in Rust that the ? operator was added to have this behavior.
- Older Rust uses the Try! for the ?
- The ? operator can only be used in a function that returns the "Result"

### Advantages of Rust's Error handling strategy
- Bugs, fail loudly, fast, and close to the cause.
- recoverable errors, possibility of errors is clear, compiler forces handling

- Panic, bad state that shouldn't happen no recovery, bugs that a programmer must fix
- Result, bad state that might happen in normal use, recovery is a possibiliyt, user might be able to fix.

- Rust Compared to C (Buffer Overread, reading past the index of a data structure.)
    - Fail loudly: Error is obvious from running the program. C is not as clear as to what the issue is.
    - Fast: Rust program stops once there is an error, C would continue past the error. (leaving room for vulnerability)
    - Rust fails close to the source. (stops where the issue happens!)

- Disadvantages:
    - Thinking about potential failures can make the development process a bit slower but quicker development usually has more bugs in production.

### Custom Error Types
- Box<Error>
- Error trait
- From trait
- Result type alias

- Box<Error>
    - trait object
        - consists of a Pointer(Box, that points to data in the heap)
        - Trait (std::error::Error)
        