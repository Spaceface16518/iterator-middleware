/// Middleware-enabled iterator
///
/// This struct wraps an iterator and applies a middleware function to each item yielded by the iterator
pub struct Iterware<I, F> {
    iter: I,
    f: F,
}

impl<I: Iterator, F> Iterator for Iterware<I, F>
    where
        F: Fn(&I::Item) -> (),
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(t) => {
                (self.f)(&t);
                Some(t)
            }
            None => None,
        }
    }
}

/// Method chaining trait
/// Import to add the `middleware` method to iterators.
/// See the [`middleware` method](#tymethod.middleware) for more example usage.
pub trait IteratorMiddleware<I, F> {
    /// use method chaining to add middleware to your iterators
    ///
    ///```rust
    /// use iterware::IteratorMiddleware;
    /// let values = vec![1, 2, 3, 4, 5];
    /// let sum = values
    ///     .into_iter()
    ///     .middleware(|value: &i32| println!("Adding value {}", value))
    ///     .sum::<i32>();
    ///
    /// assert_eq!(sum, 15);
    ///```
    fn middleware(self, f: F) -> Iterware<I, F>;
}

impl<I, F> IteratorMiddleware<I, F> for I
    where
        I: Iterator,
        F: Fn(&I::Item) -> (),
{
    fn middleware(self, f: F) -> Iterware<I, F> {
        Iterware { iter: self, f }
    }
}

#[cfg(test)]
mod tests {
    use crate::IteratorMiddleware;

    #[test]
    fn test_borrow() {
        let values = vec![1, 2, 3, 4, 5];

        let sum: i32 = values
            .into_iter()
            .middleware(|value| println!("Adding value {}", value))
            .sum();

        println!("The sum is {}", sum);

        assert_eq!(sum, 15);
    }
}
