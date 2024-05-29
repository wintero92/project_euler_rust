/// Calculate the sum of all integers up to and including `multiple` that are divisible by `divisor`. 
/// This function employs the formula for the sum of an arithmetic sequence, which ensures that 
/// the computation is efficient and direct.
///
/// # Arguments
/// - `multiple` - The upper limit (inclusive) of the range within which to find numbers divisible by `divisor`.
/// - `divisor` - The number by which other numbers must be divisible.
///
/// # Returns
/// - `i32` - The sum of all numbers up to `multiple` that are divisible by `divisor`.
///
/// # Examples
/// ```
/// use project_euler::problems::p_0001::sum_divisible;
/// assert_eq!(sum_divisible(10, 2), 30);  // Explanation: 2 + 4 + 6 + 8 + 10 = 30
/// assert_eq!(sum_divisible(20, 3), 63);  // Explanation: 3 + 6 + 9 + 12 + 15 + 18 = 63
/// ```
pub fn sum_divisible(multiple: i32, divisor: i32) -> i32 {
    // Calculate the number of terms divisible by 'divisor' up to 'multiple'
    let p: i32 = multiple / divisor;
    // Use the arithmetic series sum formula: n/2 * (first term + last term)
    // Here, the first term is 'divisor', and the last term is 'divisor * p'
    divisor * p * (p + 1) / 2
}

/// Calculate the sum of all natural numbers below a specified limit that are multiples of either 3 or 5. 
/// The function corrects for numbers that are multiples of both 3 and 5 by subtracting the sum of multiples of 15, 
/// thereby ensuring such numbers are only counted once.
///
/// # Arguments
/// - `below` - The upper limit (exclusive) up to which to sum the multiples. Default is 1000.
///
/// # Returns
/// - `i32` - The sum of all numbers below `below` that are multiples of either 3 or 5, adjusted for double-counting.
///
/// # Examples
/// ```
/// use project_euler::problems::p_0001::p_0001;
/// assert_eq!(p_0001(10), 23);  // Explanation: 3 + 5 + 6 + 9 = 23
/// assert_eq!(p_0001(20), 78);  // Explanation: 3 + 5 + 6 + 9 + 10 + 12 + 15 + 18 = 78
/// assert_eq!(p_0001(1000), 233168);
/// ```
pub fn p_0001(below: i32) -> i32 {
    // Calculate the sum of multiples of 3 below the given limit
    let sum_3: i32 = sum_divisible(below - 1, 3);
    // Calculate the sum of multiples of 5 below the given limit
    let sum_5: i32 = sum_divisible(below - 1, 5);
    // Calculate the sum of multiples of 15 below the given limit
    let sum_15: i32 = sum_divisible(below - 1, 15);
    // Apply the inclusion-exclusion principle to correct for double-counted multiples of both 3 and 5
    sum_3 + sum_5 - sum_15
}
