# Iterware

Simple middleware for [Rust](https://www.rust-lang.org/) iterators

Just a small package, mostly for debugging purposes.
Most common use would be easily printing each value in an iterator as it passes.

Think of it like a `for_each` adapter that doesn't change or consume the values of the iterator.

## Usage

Simply import the `IteratorMiddleware` trait

```rust
extern crate iterware; // Not really necessary

use iterware::IteratorMiddleware;
```

and use method chaining to add middleware to your iterators

```rust
fn sum(values: Vec<i32>) -> i32 {
    // Type annotations added for clarity
    values
        .into_iter()
        .middleware(|value: &i32| println!("Adding value {}", value))
        .sum::<i32>()
}
```