/// Calculate the largest prime factor of a given number. This function iteratively removes the smallest factors, 
/// beginning with 2, and continues with odd divisors. The search for factors stops once all prime factors are 
/// extracted or when the divisor exceeds the square root of the remaining number, which indicates the remaining number is prime.
///
/// # Arguments
/// - `value` - The number from which to find the largest prime factor. Default is 600,851,475,143.
///
/// # Returns
/// - `i64` - The largest prime factor of `value`.
///
/// # Examples
/// ```
/// use project_euler::problems::p_0003::p_0003;
/// assert_eq!(p_0003(13195), 29);  // Explanation: The prime factors of 13195 are 5, 7, 13, and 29, and the largest is 29.
/// assert_eq!(p_0003(600_851_475_143), 6857);  // The largest prime factor of 600,851,475,143.
/// ```
pub fn p_0003(mut value: i64) -> i64 {
    // Remove factors of 2 until the number is odd
    while value % 2 == 0 {
        value /= 2;
    }
    
    let mut divisor: i64 = 3;  // Start checking for factors from the smallest odd prime
    let mut max_divisor: i64 = (value as f64).sqrt() as i64;  // Calculate the initial square root of the number

    // Loop over odd divisors to find prime factors
    while value > 1 && divisor <= max_divisor {
        if value % divisor == 0 {
            value /= divisor;  // Remove the factor
            max_divisor = (value as f64).sqrt() as i64;  // Update the square root of the modified number
        } else {
            divisor += 2;  // Only test odd divisors
        }
    }

    // If value is reduced to 1, return the last divisor, otherwise return the remaining value
    if value == 1 {
        divisor - 2  // Subtract 2 because the loop increments it one last time before breaking
    } else {
        value  // Return the remaining value which is prime
    }
}
