This module will be to learn about the concepts of ownership and borrowing, which are what provide much of Rust's memory-safety guarantees.

How is data stored? 
- Disk Space: quicker than cloud
- RAM: fast. Like your kitchen counter, you need to manage this space as you prepare your food and have a strategy for cleaning up as you go, otherwise you might run out of space and not be able to do any work. Memory management and garbage collection are important for embedded devices since most of the time you are working with limited amounts of memory space.

## Ownership
- Rust's way of managing data in memory and preventing common problems. Every chunk of memory must have one variable that is the memory's owner.
- The owner is responsible for cleaning up, or deallocating, that chunk of memory. This cleanup happens automatically when the owner variable goes out of scope. The owner of data is also who is responsible for deciding whether that data is mutable or not.

* In C, you do manual allocation and deallocation of memory. (Control of when and how much memory is used)
* A Memory Leak, is when you allocate memory and forgetting to free it...reducing the overall amount of space your program has to work with.
* Memory corruption, two parts of your program trying to free up the same data (double free)
* These problems cause many problems in C since C is not very helpful in helping manage it.
* Ruby has a garbage collector that does this automatically, so you can program without thinking about memory. But, this leads to slower performance since your GC is using resources your program could otherwise be using and GC only cleans up after it is sure the program is not going to need that memory.

*RUST's ownership feature gives control over memory allocation. It cleans up data automatically when the owner goes out of scope so we can't mess up the memory access and wont be using memory longer than we strictly need to.

- By default, with non-primitive types, Rust moves ownership. If i define a, and then t, and then assign s to t, ownership moves to t while s is no longer available.
- If you define a function, then call that function using a variable defined outside of its scope as its argument, the ownership of that argument gets passed to that of the paramenter in the calling function that is then ultimately cleaned up at the end of the function that was called. ("It was moved.")

- You can also transfer ownership out of a function by returning a value, so the cleaning up happens not in the function but where the return value was invoked.

- You can also keep ownership of data but give ownership of the data to another piece of code by cloning (making a deep copy) of the allocated memory and making each copy owner of its data and responsible for cleaning up itself.

- Ownership is a different paradigm. No garbage collector. Can transfer ownership ("moving" a variable, there's also cloning)

- But, you won't be calling clone() all over the place, instead, idiomatic Rust uses Borrowing.

## Borrowing
- Allow some code to use a value without moving ownership.
- You can borrow a value by appending an & to the referenced value. This will borrow the value but not move ownership so the original owner is still responsible for cleaning up.
- Why borrow? for Performance. Instead of cloning the value, we can just reference the value without requiring extra allocation.
**Similar to pointers in C and C++ but with one important difference, the borrow checker ensures, at compile time, that you'll never have an invalid reference - aka a reference that points to nothing or to invalid memory. This prevents a lot of bugs.
- In general, can't return a reference from a function that points to something that was created within that function, because whatever was created in that function, would go out of scope at the end of the function and get cleaned up. (instead of returning a borrowed value, which you can't, return ownership of the value)
- Lend out value instead of ownership, reduce allocations and improve performance, references are always valid.


## Slices
- Slice is a data type that always borrows data owned by some other data structure.
- consists of a pointer and a length
- The pointer references the start of the data that it contains, length is the number of elements it contains after the start of the slice.
- To code a slice, you use the ampersand, the name of the variable you're referencing, square brackets, and a range...e.g. let s_slice = &s[2..4] (Start at two inclusive, end at 4 exclusive)
- if start at beginning, starting index is optional, same for if you're going to the end. If you're borrowing all the data, both are optional. Just use ..
- standard library has the implementation of a trait called Deref on String, that converts a reference to a String, into a string slice containing the whole String. Deref coercion, the compiler will auto dereference the arguments, if need be, to convert them to match the function parameter type, when you call a function or method.

## Borrowing and Mutability
- it is possible to mutably borrow a value in order to change it without taking ownership.
* Borrowing rules: (when you have a reference)
- You can either:
- have many immutable references to a value or a single mutable reference to that value
e.g. (a painting class that uses the same object as reference to paint their own representation of it.)
Many people can reference the same object to paint but you can't have it change, be mutable, while there are people trying to paint it. So, it's either or. If you mix both, you could end up with iterator invalidation.


## Borrowing Code Patterns
- New Scopes
- Temporary Variables
- Entry API
- Splitting up Structs

- Borrows happen at the same time (created in the same lexical scope in between two {})
- TO tell Rust that we're done with the immutable borrows before the end of main, we can add a new inner scope that ends before the outer scope does.

## Ownership of more than just memory
- ownership can also help manage other resources
- sockets:
- socket is a system resource thats a connection to a network endpoint for sending and receiving data (like TCP). We create it and bind it to a particular port, close it when we're done. Trying to use a closed port also causes bugs. You can have socket leaks as well if you dont close them. 
- other resources include: Mutex, Rc, and File
- Mutex is short for mutual exclusion
- Rc is reference counted, allows for multiple owners of a single piece of data
- Files, close when done using.
- Drop trait, takes a mutable reference to self, variables go out of scope in the reverse order in which they were declared. Drop trait lets you customize what happens when a value goes out of scope, and how to implement that trait on our own type.