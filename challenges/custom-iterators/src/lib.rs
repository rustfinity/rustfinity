/// A Fibonacci sequence iterator that yields Fibonacci numbers indefinitely.
///
/// The sequence starts with 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, ...
///
/// # Examples
///
/// ```
/// use custom_iterators::Fibonacci;
///
/// let fib = Fibonacci::new();
/// let first_five: Vec<u64> = fib.take(5).collect();
/// assert_eq!(first_five, vec![0, 1, 1, 2, 3]);
/// ```
pub struct Fibonacci {
    current: u64,
    next: u64,
}

impl Fibonacci {
    /// Creates a new Fibonacci iterator starting from 0.
    pub fn new() -> Self {
        Fibonacci {
            current: 0,
            next: 1,
        }
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.current;
        self.current = self.next;
        self.next = value.saturating_add(self.next);
        Some(value)
    }
}

/// An iterator that steps through a range with a custom step size.
///
/// Supports both positive and negative steps for ascending and descending ranges.
///
/// # Examples
///
/// ```
/// use custom_iterators::StepRange;
///
/// let evens: Vec<i32> = StepRange::new(0, 10, 2).collect();
/// assert_eq!(evens, vec![0, 2, 4, 6, 8]);
///
/// let countdown: Vec<i32> = StepRange::new(5, 0, -1).collect();
/// assert_eq!(countdown, vec![5, 4, 3, 2, 1]);
/// ```
pub struct StepRange {
    current: i32,
    end: i32,
    step: i32,
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
        StepRange {
            current: start,
            end,
            step,
        }
    }
}

impl Iterator for StepRange {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        // Check if we can make progress toward the end
        if self.step == 0 {
            return None;
        }

        let in_bounds = if self.step > 0 {
            self.current < self.end
        } else {
            self.current > self.end
        };

        if in_bounds {
            let value = self.current;
            self.current += self.step;
            Some(value)
        } else {
            None
        }
    }
}

/// An iterator that cycles through a slice a fixed number of times.
///
/// # Examples
///
/// ```
/// use custom_iterators::CycleN;
///
/// let cycle = CycleN::new(&[1, 2, 3], 2);
/// let result: Vec<i32> = cycle.collect();
/// assert_eq!(result, vec![1, 2, 3, 1, 2, 3]);
/// ```
pub struct CycleN<T> {
    items: Vec<T>,
    current_index: usize,
    remaining_cycles: usize,
}

impl<T: Clone> CycleN<T> {
    /// Creates a new CycleN iterator.
    ///
    /// # Arguments
    ///
    /// * `items` - The slice of items to cycle through
    /// * `times` - How many complete cycles to perform
    pub fn new(items: &[T], times: usize) -> Self {
        CycleN {
            items: items.to_vec(),
            current_index: 0,
            remaining_cycles: times,
        }
    }
}

impl<T: Clone> Iterator for CycleN<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.items.is_empty() || self.remaining_cycles == 0 {
            return None;
        }

        let value = self.items[self.current_index].clone();
        self.current_index += 1;

        if self.current_index >= self.items.len() {
            self.current_index = 0;
            self.remaining_cycles -= 1;
        }

        Some(value)
    }
}

/// An iterator that generates the Collatz sequence starting from a given number.
///
/// The Collatz sequence is defined as:
/// - If n is even: next = n / 2
/// - If n is odd: next = 3n + 1
///
/// The sequence stops when it reaches 1.
///
/// # Examples
///
/// ```
/// use custom_iterators::Collatz;
///
/// let collatz = Collatz::new(6);
/// let sequence: Vec<u64> = collatz.collect();
/// assert_eq!(sequence, vec![6, 3, 10, 5, 16, 8, 4, 2, 1]);
/// ```
pub struct Collatz {
    current: Option<u64>,
}

impl Collatz {
    /// Creates a new Collatz iterator starting from the given number.
    ///
    /// # Arguments
    ///
    /// * `start` - The starting number for the sequence
    pub fn new(start: u64) -> Self {
        Collatz {
            current: if start == 0 { None } else { Some(start) },
        }
    }
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.current?;

        if value == 1 {
            self.current = None;
        } else if value % 2 == 0 {
            self.current = Some(value / 2);
        } else {
            self.current = Some(3 * value + 1);
        }

        Some(value)
    }
}

/// An iterator that yields overlapping windows of a fixed size.
///
/// # Examples
///
/// ```
/// use custom_iterators::Windows;
///
/// let windows = Windows::new(&[1, 2, 3, 4, 5], 3);
/// let groups: Vec<Vec<i32>> = windows.collect();
/// assert_eq!(groups, vec![vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]);
/// ```
pub struct Windows<T> {
    items: Vec<T>,
    size: usize,
    current_start: usize,
}

impl<T: Clone> Windows<T> {
    /// Creates a new Windows iterator.
    ///
    /// # Arguments
    ///
    /// * `items` - The slice to create windows from
    /// * `size` - The size of each window
    pub fn new(items: &[T], size: usize) -> Self {
        Windows {
            items: items.to_vec(),
            size,
            current_start: 0,
        }
    }
}

impl<T: Clone> Iterator for Windows<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size == 0 || self.current_start + self.size > self.items.len() {
            return None;
        }

        let window = self.items[self.current_start..self.current_start + self.size].to_vec();
        self.current_start += 1;
        Some(window)
    }
}

/// An iterator that generates values from a state and a function.
///
/// The function takes the current state and returns `Some(next_state)` to continue
/// or `None` to stop. Each state value is yielded, including the initial value.
///
/// # Examples
///
/// ```
/// use custom_iterators::Unfold;
///
/// // Powers of 2 until > 100
/// let powers = Unfold::new(1u32, |&n| {
///     let next = n * 2;
///     if next <= 100 { Some(next) } else { None }
/// });
/// let result: Vec<u32> = powers.collect();
/// assert_eq!(result, vec![1, 2, 4, 8, 16, 32, 64]);
/// ```
pub struct Unfold<T, F> {
    state: Option<T>,
    f: F,
    first: bool,
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
    pub fn new(initial: T, f: F) -> Self {
        Unfold {
            state: Some(initial),
            f,
            first: true,
        }
    }
}

impl<T: Clone, F> Iterator for Unfold<T, F>
where
    F: FnMut(&T) -> Option<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.state.as_ref()?;

        if self.first {
            self.first = false;
            return Some(current.clone());
        }

        let next_state = (self.f)(current);
        match next_state {
            Some(next) => {
                self.state = Some(next.clone());
                Some(next)
            }
            None => {
                self.state = None;
                None
            }
        }
    }
}
