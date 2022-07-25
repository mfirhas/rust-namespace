# Rust Namespace #

Namespace is a declarative region that provides a scope to the identifiers[[1]](https://docs.microsoft.com/en-us/cpp/cpp/namespaces-cpp).
It's a way to segment and organize your code well so that it's easy to reason about. It's also a way to encapsulate some implementations/details of your code creating interafce to the code consumer.

In Rust, it's called **mod**. Mod behaves like C++ namespace where it segments code to contains implementation details like type, constants, functions and traits. Mod is how you access certain parts of you codebase. Mod is the one which segregate between packages, crates, and other smaller components providing a path to certain parts of code. Mod can be used for both type of program in rust's crate *binary* and *library*.

**Binary** : crate producing executable program with `fn main()` as its execution entry point.

**Library** : crate producing non-executable program acts as supporting code for binary crate without `fn main()`.

Here are examples of Rust's namespacing with mod in these 5 kind of codebase structures:
- 1 binary
- 1 library
- 1 binary and 1 library
- n binary and 1 library
- workspace