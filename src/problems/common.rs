/// Determine whether a given integer is a palindrome. A palindrome is a sequence that reads
/// the same backward as forward.
///
/// # Arguments
/// - `value` - The integer to check for palindromicity.
///
/// # Returns
/// - `bool` - Returns `true` if `value` is a palindrome; otherwise, `false`.
///
/// # Examples
/// ```
/// use project_euler::problems::common::is_palindrome;
/// assert_eq!(is_palindrome(121), true);
/// assert_eq!(is_palindrome(-121), false);
/// assert_eq!(is_palindrome(123), false);
/// ```
pub fn is_palindrome(value: i32) -> bool {
    // Handle negative numbers as not palindromes
    if value < 0 {
        return false;
    }

    // Convert the integer to a string to check palindromicity
    let value_str: String = value.to_string();
    let value_chars: Vec<char> = value_str.chars().collect();
    let length: usize = value_chars.len();
    let mid: usize = length / 2;

    // Check each character from the start against its corresponding from the end
    for i in 0..mid {
        if value_chars[i] != value_chars[length - i - 1] {
            // Early exit with `false` if a mismatch is found
            return false;
        }
    }

    // If no mismatches were found, return `true`
    true
}

/// Implements the Sieve of Eratosthenes algorithm to identify all prime numbers up to a specified `limit`.
/// Named after the ancient Greek mathematician Eratosthenes, this algorithm is a highly efficient way
/// to find all prime numbers smaller than a given limit. It does so by iteratively marking the multiples
/// of each discovered prime number starting from 2.
///
/// The core idea is simple: Start with an assumption that all numbers are prime. Then, sequentially eliminate
/// multiples of each prime number starting from 2. By the time you reach a number `n`, if it has not been marked
/// as non-prime, then it is prime. The reason this method is efficient is because it eliminates the need to check
/// each number individually through division; instead, it systematically removes non-prime candidates in a series
/// of sweeping actions across the number range.
///
/// # Arguments
/// - `limit` - The upper boundary up to which primes are to be found, exclusive of the limit itself.
///
/// # Returns
/// - `Vec<i32>` - A vector containing all prime numbers less than `limit`.
///
/// # Examples
/// ```
/// use project_euler::problems::common::sieve_of_eratosthenes;
/// assert_eq!(sieve_of_eratosthenes(10), vec![2, 3, 5, 7]);  // The primes less than 10 are 2, 3, 5, and 7.
/// assert_eq!(sieve_of_eratosthenes(30), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);  // The primes less than 30 are listed.
/// ```
pub fn sieve_of_eratosthenes(limit: i32) -> Vec<i32> {
    // Initialize a boolean vector, `is_prime`, to true for all positions indicating all numbers are initially considered prime.
    let mut is_prime: Vec<bool> = vec![true; limit as usize];
    if limit > 0 {
        is_prime[0] = false; // Set the first index to false as 1 is not a prime number.
    }
    if limit > 1 {
        is_prime[1] = false; // Set the second index to false as 1 is not a prime number.
    }

    // Begin marking non-prime numbers starting with the first prime number, 2.
    let mut p: i32 = 2;
    while p * p < limit {
        if is_prime[p as usize] {
            // If `p` is a prime, mark all multiples of `p` starting from `p^2` to `limit` as non-prime.
            let mut multiple = p * p;
            while multiple < limit {
                is_prime[multiple as usize] = false;
                multiple += p;
            }
        }
        p += 1;  // Move to the next number.
    }

    // Collect the numbers that remain marked as prime into the list `primes`.
    let mut primes: Vec<i32> = Vec::new();
    for p in 2..limit {
        if is_prime[p as usize] {
            primes.push(p);
        }
    }

    primes  // Return the list of primes.
}

