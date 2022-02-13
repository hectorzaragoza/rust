## Lifetimes
- The length of time that a reference to some value may exist.
- The compiler is what tracks this. (if your code compiles, all references are VALID)
- References only exist while the value they point to has not been moved to another location or been cleaned up.

- Use lifetime parameter annotations to inform the compiler about lifetimes in your code.
- generic lifetime parameters are descriptive, not prescriptive. The purpose of lifetime parameters is only to describe to the compiler how the lifetimes of references are related.

### Concrete Lifetimes
- A values lifetime is the time during which a value exists at a particular memory address.
    - starts when a value is created or moved into a particular location in memory and ends when that value is moved out of or dropped from that location.
- a references lifetime is the same as that for a value. A reference is also a value. A references lifetime must be contained within the lifetime of the value being referenced.
- a lifetime of a reference must be contained within the value's it references lifetime.

### Visualizing lifetimes to understand borrow checker errors.
- You can't have the lifetime of a reference to be longer than the lifetime the value. Error: 'variable' does not live long enough.
- What is a hashmap?

### An exploration of generic lifetimes
- a generic lifetime is the lifetime of a reference in code, where we cant know all of the possible concrete lifetimes of the values being referenced at compile time.