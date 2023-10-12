//! A module for generating random numbers.
use rand::thread_rng;
use rand::Rng;

/// Generates a random number between 0 (inclusive) and the given upper bound (exclusive).
///
/// # Arguments
///
/// * `i` - The upper bound for the random number generation. The generated number will be in the range [0, i).
///
/// # Returns
///
/// A random `usize` number between 0 (inclusive) and `i` (exclusive).
///
/// # Examples
///
/// ```
/// use your_crate_name::random::random;
///
/// let num = random(10);
/// assert!(num < 10);
/// ```
pub fn random(i: usize) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(0..i)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that the `random` function generates a number within the expected range.
    #[test]
    fn test_random_within_range() {
        for _ in 0..1000 {
            let upper_bound = 10;
            let num = random(upper_bound);
            assert!(num < upper_bound);
        }
    }

    /// Tests that the `random` function generates a number of 0 when the upper bound is 1.
    #[test]
    fn test_random_upper_bound_one() {
        let num = random(1);
        assert_eq!(num, 0);
    }

    /// Tests that the `random` function panics when the upper bound is 0.
    #[test]
    #[should_panic(expected = "cannot sample empty range")]
    fn test_random_upper_bound_zero() {
        random(0);
    }
}
