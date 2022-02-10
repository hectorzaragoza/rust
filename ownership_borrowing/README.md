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