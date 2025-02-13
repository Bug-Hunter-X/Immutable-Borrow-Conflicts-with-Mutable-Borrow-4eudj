# Rust Borrow Checker Error: Immutable Borrow Conflicts with Mutable Borrow

This example showcases a common error in Rust: attempting to mutably borrow a vector while an immutable borrow already exists through an iterator.

The `bug.rs` file demonstrates the problem, while `bugSolution.rs` offers solutions.

## How to reproduce the error:
1. Save the code in `bug.rs`.
2. Compile and run with `rustc bug.rs && ./bug`

This will result in a compile-time error if the error is not handled. If it is compiled the program will panic at runtime. 

## Solutions:  See `bugSolution.rs`