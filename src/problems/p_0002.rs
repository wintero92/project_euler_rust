/// Calculate the sum of all even Fibonacci numbers up to a specified limit. The function utilizes a property 
/// of the Fibonacci sequence that every third number is even, and computes the sum directly without iterating 
/// through all Fibonacci numbers.
///
/// # Arguments
/// - `below` - The upper limit (exclusive) for the Fibonacci sequence. Default is 4,000,000.
///
/// # Returns
/// - `i32` - The sum of even Fibonacci numbers less than `below`.
///
/// # Examples
/// ```
/// use project_euler::problems::p_0002::p_0002;
/// assert_eq!(p_0002(100), 44);  // Explanation: 2 + 8 + 34 = 44
/// assert_eq!(p_0002(4_000_000), 4613732);
/// ```
pub fn p_0002(below: i32) -> i32 {
    let mut f3: i32 = 2; // f3 holds the last even Fibonacci number computed
    let mut f6: i32 = 0; // f6 holds the even Fibonacci number before f3
    let mut r: i32 = 2; // r is the current Fibonacci number being considered
    let mut s: i32 = 0; // s accumulates the sum of even Fibonacci numbers

    while r < below {
        s += r; // Add the current even Fibonacci number to the sum
        r = 4 * f3 + f6; // Calculate the next even Fibonacci number
        f6 = f3; // Update the previous two even Fibonacci numbers
        f3 = r; // Update f3 to the current even Fibonacci number
    }

    s // Return the accumulated sum
}
