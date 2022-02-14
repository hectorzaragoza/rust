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
- You have to be careful about your values and reference lifetimes to ensure that everything is valid.

### A Short intro to generics
- Generic type parameters are an abstraction that allow us to write code once that can be used many times with different types.
- What is the impl keyword?

### Lifetime parameters are a kind of generic
- purpose of generic lifetime parameters is to tell the compiler how the lifetimes of references are related to check for validity in references.
- 'a could be used to tell Rust what a return type is referencing..in the example case it could reference either of the parameters of the function by appending each one with the "tick a"
- Specifying the lifetimes helps prevent bugs by stating in a different way what we intend for the body of the funtion to do. (keeps analysis local, enables errors to be deteced locally rather than caused by usages of the definition that might be far away from the cause of a problem.)

### Lifetime parameters are descriptive, not prescriptive
- lifetime parameters dont specify how long a concrete lifetime of a value or a reference must be.
- They dont change the concrete lifetimes of values or references.

### Lifetime elision
- Elision means omitting or leaving out.
- unique to rust, so that you dont have to add generic lifetime parameters on every reference.
- 1st rule. Each parameter thats a reference is considered to have its own lifetime parameter.
- 2nd rule. If there is only one lifetime in the parameters, a returned reference gets that lifetime.
- 3rd rule only applies ot methods. It gives return types the same lifetime as the self parameter, even if there are other parameters wtih different lifetimes.

