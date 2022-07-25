# Rust Namespace #

Some examples of how namespacing in Rust is.

**Binary** : crate producing executable program with `fn main()` as its execution entry point.

**Library** : crate producing non-executable program acts as supporting code for binary crate without `fn main()`.

There are 4 kinds of namespacing:
- 1 binary
- 1 library
- 1 binary and 1 library
- n binary and 1 library
- workspace