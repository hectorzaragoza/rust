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