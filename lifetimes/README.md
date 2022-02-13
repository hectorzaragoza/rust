## Lifetimes
- The length of time that a reference to some value may exist.
- The compiler is what tracks this. (if your code compiles, all references are VALID)
- References only exist while the value they point to has not been moved to another location or been cleaned up.

- Use lifetime parameter annotations to inform the compiler about lifetimes in your code.
- generic lifetime parameters are descriptive, not prescriptive. The purpose of lifetime parameters is only to describe to the compiler how the lifetimes of references are related.