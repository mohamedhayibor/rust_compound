//! *********** Typical usage ***************
//!
//! use compound;
//!
//! fn main() {
//!     let percentages = [10., 11., 23., 34.];
//!     let initial_investment: f64 = 1000.00;
//!     let balance = compound( &initial_investment, &percentages);

//!     println!("Your balance is {}", balance );
//!     println!("The percentages were {:?}", percentages);
//! }

//! > major change: just noticed that primitives already implement the copy trait
//! and therefore wouldn't mutate anything
pub fn compound(investment: f64, returns: &[f64]) -> f64 {
    (returns.iter().fold(investment, |value, perc | {
        value * ( 1. + (perc / 100. ))
    }) * 100.).round() / 100.
}

// compound should work with vectors
// #[cfg(test)]
#[test]
fn test_compound_vectors() {
    let percentages = vec![10.2, -11.3, 23.5, -34., 0.];
    let init_value = 1000.00;

    let balance = compound(init_value, &percentages);

    assert_eq!(balance, 796.74);

    // the passed in vector should still be available
    assert_eq!(percentages, vec![10.2, -11.3, 23.5, -34., 0.]);
}

// compound should work with slices
// #[cfg(test)]
#[test]
fn test_compound_slices() {
    let percentages = [10.2, -11.3, 23.5, -34., 0.];

    let balance = compound(1000.00, &percentages);

    assert_eq!(balance, 796.74);
}

// *** Special Thanks to zsck, marciaux, exoticon for helping out ***
// See discussion at: https://users.rust-lang.org/t/pull-me-out-of-insanity-cant-wrap-my-head-around-this-error-expected-std-vec-vec-f64-found---4-expected-ptr-found-array-of-4-elements-e0308/6688

