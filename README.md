# Rust Borrow Checker Example

This repository demonstrates a common error encountered when working with mutable and immutable references in Rust. The error arises when attempting to use an immutable reference (&) after a mutable reference (&mut) to the same data has been established.  Rust's borrow checker prevents this to guarantee data safety and prevent race conditions.  The `bug.rs` file shows the problematic code, while `bugSolution.rs` provides a corrected approach.

## How to run

1. Clone this repository.
2. Navigate to the repository's directory.
3. Compile and run the Rust file using `rustc bug.rs && ./bug` (and similarly for `bugSolution.rs`).

## Understanding the Error

The core issue lies in Rust's ownership and borrowing system.  The borrow checker ensures that at any given time, there is only one mutable reference or multiple immutable references to a particular piece of data. The code in `bug.rs` violates this rule, leading to a compile-time error.