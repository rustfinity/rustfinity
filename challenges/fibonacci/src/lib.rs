/// Dynamic Programming implementation of the Fibonacci sequence.
///
/// # Arguments
/// 
/// * `n`:  u32, the nth number in the fibonacci sequence
///
/// returns: u32  the result of the nth number in the fibonacci sequence
///
/// # Examples
///
/// ```
/// use fibonacci::fibonacci;
/// let result = fibonacci(4);
/// assert_eq!(result, 3);
//
/// ```
pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    let mut fib = vec![0; (n + 1) as usize];
    fib[1] = 1;
    for i in 2..=n as usize {
        fib[i] = fib[i - 1] + fib[i - 2];
    }
    fib[n as usize]
}