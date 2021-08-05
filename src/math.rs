//! Miscellaneous math functions.
//!
//! Does this need to exist?
//! Used for dev but should probably be replaced with something faster.
//!
//! TODO: Find a better name for this module, with more clarity of its purpose
//!
//! TODO: make generic for floats?
//! TODO: benchmark
//! TODO: handle panics as Result

/////////////////////////////
// Consts

/// `f64` equality comparison ratio epsilon
pub const F64_EQ_EPSILON: f64 = 0.000_000_1;

/////////////////////////////
// Functions

/// Computes the inner product of two `f64` array slices. Arrays must be of *equal, non-zero length*.
///
/// # Panics
/// * Non-compatible array lengths
/// * Zero-length arrays
///
/// # Example
/// ```
/// use petgraph_cluster::math::{F64_EQ_EPSILON, inner_product};
/// use float_eq::float_eq;
///
/// let c = inner_product(&[1.0, 2.5, 3.0, 1.0], &[1.0, 2.0, 4.0, 1.0]);
/// assert!(float_eq!(c, 19.0, rel <= F64_EQ_EPSILON));
/// ```
pub fn inner_product(a: &[f64], b: &[f64]) -> f64 {
    assert_eq!(a.len(), b.len());
    assert!(a.len() > 0);

    a.iter()
        .zip(b.iter())
        .fold(0.0, |acc, (ai, bi)| acc + ai * bi)
}

/// Computes the L2-norm of a vector.
pub fn l2_norm(a: &[f64]) -> f64 {
    inner_product(a, a).sqrt()
}

/// Computes the correlation of two vectors as their cosine similarity.
///
/// TODO: numerical stability? speed trade-offs.
pub fn correlation(a: &[f64], b: &[f64]) -> f64 {
    inner_product(a, b) / (l2_norm(a) * l2_norm(b))
}

/////////////////////////////
// Unit tests

#[cfg(test)]
mod test {
    use super::*;
    use float_eq::float_eq;

    #[test]
    #[should_panic]
    fn inner_product_zero_len_fail() {
        let _c = inner_product(&[], &[]);
    }

    #[test]
    #[should_panic]
    fn inner_product_unequal_len_fail() {
        let _c = inner_product(&[1.0, 2.5, 3.0, 1.0], &[1.0, 2.0, 4.0]);
    }
}
