### Rust with Demeteri Nesteruk

Types of Data:
- Boolean
- Ints and Floats
- Text
- Structured Data (HTML, XML, JSON)
- Binary data 

Two Types of Computing
- Analog (use electrical current to measure variable data)
- Digital (use pulses to encode data)
    - Communication using a stream of 0s and 1s
    - Each piece of information is called a bit
    - We can store bits in memory (RAM, SSD) or transmit them.
- N bits allow us to use 2 to the N combination of states. a byte, is the moste commmon one, 8bits.
- a byte or 8 bits can store 256 states
- two-byte number is sometimes called a 'short'
- 4 bytes together = 4 billion different possibilities.

Can store whole unsigned numbers, signed numbers, float numbers.
    - 32-bit, float, single precision
    - 64-bit, double, double-precision

- The bitness of a cpu, puts a limit on range of memory you can access.
- A 32-bit process can only use up to 4Gb RAM


### Application Entrypoint
- fn keyword for function
- fn main() is the entrypoint for a rust program. (can't have multiple main fn)
- commands followed by ! are macros.
- You can explicitly allow code that would spit an error: #![allow(dead_code)], won't report error for variables that are unused. (Keep things a bit quiet)

* Rust does not support the operators -- or ++ but you can do a -=2 or a += 2.
* There is no === in Rust...just ==

### MEMORY- Stack and Heap
Imagine a vertical block. Bottom part is the stack. When you bind a variable to a value, it takes a chunk of memory at the stack (bottom - LIFO, Last in, first out.) Stack is also used for functions,as you call a function and all its arguments, you are working on the stack.
- Stack is fast but size is limited. (Short term storage structure)

- The heap is at the top of the vertical memory block. (Long-term storage structure)
    - let x = Box: 'new(5); You end up with a reference, pointer.
    - x would still be in the stack but instead of holding the value in the stack, it has a memory address pointing to the value that lives in the heap.
    - instead of doing this println!("{}", *x) //The star makes use of the pointer.