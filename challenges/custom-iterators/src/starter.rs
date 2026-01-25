/// A Fibonacci sequence iterator that yields Fibonacci numbers indefinitely.
///
/// The sequence starts with 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, ...
pub struct Fibonacci {
    // TODO: Add fields to track the current state of the sequence
}

impl Fibonacci {
    /// Creates a new Fibonacci iterator starting from 0.
    pub fn new() -> Self {
        // TODO: Initialize the Fibonacci iterator
        todo!()
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Return the current Fibonacci number and advance the state
        todo!()
    }
}

/// An iterator that steps through a range with a custom step size.
///
/// Supports both positive and negative steps for ascending and descending ranges.
pub struct StepRange {
    // TODO: Add fields to track current position, end, and step
}

impl StepRange {
    /// Creates a new StepRange iterator.
    ///
    /// # Arguments
    ///
    /// * `start` - The starting value (inclusive)
    /// * `end` - The ending value (exclusive)
    /// * `step` - The step size (positive or negative)
    pub fn new(start: i32, end: i32, step: i32) -> Self {
        // TODO: Initialize the StepRange iterator
        todo!()
    }
}

impl Iterator for StepRange {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Return the current value and step forward
        // Handle both positive and negative steps correctly
        todo!()
    }
}

/// An iterator that cycles through a slice a fixed number of times.
pub struct CycleN<T> {
    // TODO: Add fields to store items, track position, and count remaining cycles
    _marker: std::marker::PhantomData<T>,
}

impl<T: Clone> CycleN<T> {
    /// Creates a new CycleN iterator.
    ///
    /// # Arguments
    ///
    /// * `items` - The slice of items to cycle through
    /// * `times` - How many complete cycles to perform
    pub fn new(_items: &[T], _times: usize) -> Self {
        // TODO: Initialize the CycleN iterator
        todo!()
    }
}

impl<T: Clone> Iterator for CycleN<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Return the next item in the cycle
        // When reaching the end of items, wrap around and decrement remaining cycles
        todo!()
    }
}

/// An iterator that generates the Collatz sequence starting from a given number.
///
/// The Collatz sequence is defined as:
/// - If n is even: next = n / 2
/// - If n is odd: next = 3n + 1
///
/// The sequence stops when it reaches 1.
pub struct Collatz {
    // TODO: Add a field to track the current value
    // Consider using Option<u64> to handle the "exhausted" state
}

impl Collatz {
    /// Creates a new Collatz iterator starting from the given number.
    pub fn new(start: u64) -> Self {
        // TODO: Initialize the Collatz iterator
        todo!()
    }
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Yield the current value, then compute the next value
        // Stop after yielding 1
        todo!()
    }
}

/// An iterator that yields overlapping windows of a fixed size.
pub struct Windows<T> {
    // TODO: Add fields to store items, window size, and current position
    _marker: std::marker::PhantomData<T>,
}

impl<T: Clone> Windows<T> {
    /// Creates a new Windows iterator.
    ///
    /// # Arguments
    ///
    /// * `items` - The slice to create windows from
    /// * `size` - The size of each window
    pub fn new(_items: &[T], _size: usize) -> Self {
        // TODO: Initialize the Windows iterator
        todo!()
    }
}

impl<T: Clone> Iterator for Windows<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Return a window of `size` elements starting at current position
        // Advance the position by 1 for overlapping windows
        todo!()
    }
}

/// An iterator that generates values from a state and a function.
///
/// The function takes the current state and returns `Some(next_state)` to continue
/// or `None` to stop. Each state value is yielded, including the initial value.
pub struct Unfold<T, F> {
    // TODO: Add fields for state, the function, and tracking first yield
    _marker: std::marker::PhantomData<(T, F)>,
}

impl<T, F> Unfold<T, F>
where
    F: FnMut(&T) -> Option<T>,
{
    /// Creates a new Unfold iterator.
    ///
    /// # Arguments
    ///
    /// * `initial` - The initial state value (will be yielded first)
    /// * `f` - A function that takes the current state and returns the next state
    pub fn new(_initial: T, _f: F) -> Self {
        // TODO: Initialize the Unfold iterator
        todo!()
    }
}

impl<T: Clone, F> Iterator for Unfold<T, F>
where
    F: FnMut(&T) -> Option<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: On first call, yield the initial value
        // On subsequent calls, apply the function to get the next state
        // Return None when the function returns None
        todo!()
    }
}

pub fn main() {
    // Fibonacci example
    println!("First 10 Fibonacci numbers:");
    let fib = Fibonacci::new();
    let first_ten: Vec<u64> = fib.take(10).collect();
    println!("{:?}", first_ten);

    // StepRange example
    println!("\nEven numbers 0-10:");
    let evens: Vec<i32> = StepRange::new(0, 10, 2).collect();
    println!("{:?}", evens);

    println!("\nCountdown from 5:");
    let countdown: Vec<i32> = StepRange::new(5, 0, -1).collect();
    println!("{:?}", countdown);

    // CycleN example
    println!("\nCycle [1, 2, 3] twice:");
    let cycled: Vec<i32> = CycleN::new(&[1, 2, 3], 2).collect();
    println!("{:?}", cycled);

    // Collatz example
    println!("\nCollatz sequence starting from 6:");
    let collatz: Vec<u64> = Collatz::new(6).collect();
    println!("{:?}", collatz);

    // Windows example
    println!("\nWindows of size 3 over [1, 2, 3, 4, 5]:");
    let windows: Vec<Vec<i32>> = Windows::new(&[1, 2, 3, 4, 5], 3).collect();
    println!("{:?}", windows);

    // Unfold example
    println!("\nPowers of 2 up to 100:");
    let powers: Vec<u32> = Unfold::new(1u32, |&n| {
        let next = n * 2;
        if next <= 100 {
            Some(next)
        } else {
            None
        }
    })
    .collect();
    println!("{:?}", powers);
}
